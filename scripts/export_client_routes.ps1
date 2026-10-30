#Requires -Version 7.0
<#
.SYNOPSIS
Export installed Zwift client route metadata to a private, local JSON file.
.DESCRIPTION
Reads the selected cache ZIP without extracting or executing anything. Previously
extracted routes*.xml files may supply IDs absent from the archive; archive
records win as a whole. Inputs and the race database are never modified.
Choose an OutputPath outside the repository. Existing output files are refused.
.EXAMPLE
pwsh.exe -NoProfile -File scripts/export_client_routes.ps1 -ClientArchive C:\path\cache.zip -ExtractedDirectory C:\path\Worlds -OutputPath C:\Users\you\AppData\Local\Temp\client-routes.json
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory)][ValidateNotNullOrEmpty()][string] $ClientArchive,
    [ValidateNotNullOrEmpty()][string] $ExtractedDirectory,
    [Parameter(Mandatory)][ValidateNotNullOrEmpty()][string] $OutputPath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# These are client ASSET world IDs, not the public event API's map IDs.
$worldNames = @{
    '1' = 'Watopia'; '2' = 'Richmond'; '3' = 'London'; '4' = 'New York'
    '5' = 'Innsbruck'; '6' = 'Bologna'; '7' = 'Yorkshire'; '8' = 'Crit City'
    '9' = 'Makuri Islands'; '10' = 'France'; '11' = 'Paris'
    '12' = 'Gravel Mountain'; '13' = 'Scotland'
}

function Get-StreamSha256([IO.Stream] $Stream) {
    $hasher = [Security.Cryptography.SHA256]::Create()
    try { return [BitConverter]::ToString($hasher.ComputeHash($Stream)).Replace('-', '').ToLowerInvariant() }
    finally { $hasher.Dispose() }
}

function Get-AssetWorld([string[]] $Locations) {
    $ids = @($Locations | ForEach-Object {
        [regex]::Matches($_, '(?i)(?:^|[\\/])world(\d+)(?=[\\/.]|$)') |
            ForEach-Object { [uint32]::Parse($_.Groups[1].Value).ToString() }
    } | Sort-Object -Unique)
    if ($ids.Count -ne 1) {
        throw "Cannot determine one asset world ID from '$($Locations -join ' :: ')'; expected a consistent world<N> in the filename or path."
    }
    if ($worldNames.ContainsKey($ids[0])) { return $worldNames[$ids[0]] }
    return "world$($ids[0])"
}

function Read-ClientXml([string] $Text, [string] $Location, [switch] $Fragment) {
    if ($Fragment) {
        # Client source files have several top-level elements (VERSION, route,
        # checkpoints, ...). Remove only the leading declaration, then wrap.
        $Text = [regex]::Replace($Text, '\A\uFEFF?\s*<\?xml\s[^?]*\?>', '')
        $Text = "<client-fragment>$Text</client-fragment>"
    }
    $settings = [Xml.XmlReaderSettings]::new()
    $settings.DtdProcessing = [Xml.DtdProcessing]::Prohibit
    $settings.XmlResolver = $null
    $reader = [Xml.XmlReader]::Create([IO.StringReader]::new($Text), $settings)
    try {
        $doc = [Xml.XmlDocument]::new()
        $doc.XmlResolver = $null
        $doc.Load($reader)
        return ,$doc
    }
    catch { throw "Invalid client XML in '$Location': $($_.Exception.Message)" }
    finally { $reader.Dispose() }
}

function Get-OptionalAttribute([Xml.XmlElement] $Node, [string] $Attribute) {
    if ($Node.HasAttribute($Attribute)) { return $Node.GetAttribute($Attribute) }
    return $null
}

function Get-Metres([Xml.XmlElement] $Node, [string] $Attribute, [string] $Location, [switch] $Required) {
    if (-not $Node.HasAttribute($Attribute)) {
        if ($Required) { throw "Missing '$Attribute' in '$Location'." }
        return $null
    }
    $value = 0.0
    $raw = $Node.GetAttribute($Attribute)
    if (-not [double]::TryParse($raw, [Globalization.NumberStyles]::Float,
            [Globalization.CultureInfo]::InvariantCulture, [ref] $value) -or
        -not [double]::IsFinite($value) -or $value -lt 0 -or ($Required -and $value -le 0)) {
        throw "Invalid '$Attribute' value '$raw' in '$Location'; expected finite nonnegative metres (route distance must be positive)."
    }
    return $value
}

function ConvertTo-Kilometres($Metres) {
    if ($null -eq $Metres) { return $null }
    return [double] $Metres / 1000.0
}

function Convert-ClientRoute([Xml.XmlElement] $Node, [string] $World, $Source) {
    $location = "$($Source.path) :: $($Source.entry)"
    $rawId = $Node.GetAttribute('nameHash')
    $id = [long] 0
    if (-not [long]::TryParse($rawId, [Globalization.NumberStyles]::AllowLeadingSign,
            [Globalization.CultureInfo]::InvariantCulture, [ref] $id) -or
        $id -lt [int]::MinValue -or $id -gt [uint32]::MaxValue) {
        throw "Invalid nameHash '$rawId' in '$location'; expected a u32 or signed i32 hash."
    }
    if ($id -lt 0) { $id += 4294967296L }
    $name = $Node.GetAttribute('name').Trim()
    if ([string]::IsNullOrWhiteSpace($name)) { throw "Missing route name for ID $id in '$location'." }
    return [pscustomobject][ordered]@{
        route_id = [uint32] $id
        name = $name
        world = $World
        surface = 'road'
        distance_km = ConvertTo-Kilometres (Get-Metres $Node 'distanceInMeters' $location -Required)
        elevation_m = Get-Metres $Node 'ascentInMeters' $location
        lead_in_distance_km = ConvertTo-Kilometres (Get-Metres $Node 'leadinDistanceInMeters' $location)
        lead_in_elevation_m = Get-Metres $Node 'leadinAscentInMeters' $location
        lead_in_distance_free_ride_km = ConvertTo-Kilometres (Get-Metres $Node 'freeRideLeadinDistanceInMeters' $location)
        lead_in_elevation_free_ride_m = Get-Metres $Node 'freeRideLeadinAscentInMeters' $location
        lead_in_distance_meetups_km = ConvertTo-Kilometres (Get-Metres $Node 'meetupLeadinDistanceInMeters' $location)
        lead_in_elevation_meetups_m = Get-Metres $Node 'meetupLeadinAscentInMeters' $location
        slug = $null
        event_only = Get-OptionalAttribute $Node 'eventOnly'
        sport_type = Get-OptionalAttribute $Node 'sportType'
        supported_laps = Get-OptionalAttribute $Node 'supportedLaps'
        source = $Source
    }
}

function Add-UniqueRoute($Records, $Record) {
    if ($Records.ContainsKey($Record.route_id)) {
        $previous = $Records[$Record.route_id].source
        throw "Duplicate route ID $($Record.route_id) within '$($Record.source.kind)': '$($previous.path) :: $($previous.entry)' and '$($Record.source.path) :: $($Record.source.entry)'. Select a source without duplicate IDs."
    }
    $Records.Add($Record.route_id, $Record)
}

$archivePath = (Get-Item -LiteralPath $ClientArchive -Force).FullName
if (-not [IO.File]::Exists($archivePath)) { throw "ClientArchive must be a ZIP file: '$ClientArchive'." }
$extractedPath = $null
if ($ExtractedDirectory) {
    $extractedPath = (Get-Item -LiteralPath $ExtractedDirectory -Force).FullName
    if (-not [IO.Directory]::Exists($extractedPath)) { throw "ExtractedDirectory must be a directory: '$ExtractedDirectory'." }
}
$output = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($OutputPath)
$output = [IO.Path]::GetFullPath($output)
if ($output.StartsWith('\\') -or ([IO.DriveInfo]::new([IO.Path]::GetPathRoot($output))).DriveType -eq [IO.DriveType]::Network) {
    throw 'OutputPath must be on a local drive, not a network share.'
}
if ([IO.Path]::GetExtension($output) -ine '.json') { throw 'OutputPath must have a .json extension.' }
if (Test-Path -LiteralPath $output) { throw "OutputPath already exists; choose a new private JSON path: '$output'." }
if (-not [IO.Directory]::Exists([IO.Path]::GetDirectoryName($output))) {
    throw "OutputPath parent directory does not exist: '$output'."
}

$primary = [Collections.Generic.Dictionary[uint32, object]]::new()
$secondary = [Collections.Generic.Dictionary[uint32, object]]::new()
# Keep a read-only, non-writable-sharing handle open for both hashing and parsing.
$archiveStream = [IO.File]::Open($archivePath, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)
try {
    $archiveHash = Get-StreamSha256 $archiveStream
    $archiveStream.Position = 0
    $zip = [IO.Compression.ZipArchive]::new($archiveStream, [IO.Compression.ZipArchiveMode]::Read, $true)
    try {
        $entries = @($zip.Entries | Where-Object {
            $_.FullName.Replace('\', '/') -match '(?i)(?:^|/)(?:world\d+\.routes|routes[^/]*)\.xml$'
        } | Sort-Object FullName)
        if ($entries.Count -eq 0) { throw "No client route XML entries found in '$archivePath'; select the installed client routes cache ZIP." }
        foreach ($entry in $entries) {
            $location = "$archivePath :: $($entry.FullName)"
            $reader = [IO.StreamReader]::new($entry.Open(), [Text.UTF8Encoding]::new($false, $true), $true)
            try { $doc = Read-ClientXml $reader.ReadToEnd() $location }
            finally { $reader.Dispose() }
            $wrappers = @($doc.SelectNodes('/routes/route'))
            if ($doc.DocumentElement.Name -cne 'routes' -or $wrappers.Count -eq 0) {
                throw "Expected /routes/route/route records in '$location'."
            }
            foreach ($wrapper in $wrappers) {
                $world = Get-AssetWorld @($entry.FullName, $wrapper.GetAttribute('file'))
                $nodes = @($wrapper.SelectNodes('route'))
                if ($nodes.Count -ne 1) { throw "Expected exactly one inner route in '$location' (file '$($wrapper.GetAttribute('file'))')." }
                $source = [ordered]@{ kind = 'client-cache'; path = $archivePath; sha256 = $archiveHash; entry = $entry.FullName }
                Add-UniqueRoute $primary (Convert-ClientRoute $nodes[0] $world $source)
            }
        }
    }
    finally { $zip.Dispose() }
}
finally { $archiveStream.Dispose() }

if ($extractedPath) {
    $files = @(Get-ChildItem -LiteralPath $extractedPath -Recurse -File -Filter 'routes*.xml' | Sort-Object FullName)
    if ($files.Count -eq 0) { throw "No routes*.xml files found under ExtractedDirectory '$extractedPath'." }
    foreach ($file in $files) {
        $stream = [IO.File]::Open($file.FullName, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)
        try {
            $hash = Get-StreamSha256 $stream
            $stream.Position = 0
            $reader = [IO.StreamReader]::new($stream, [Text.UTF8Encoding]::new($false, $true), $true, 1024, $true)
            try { $doc = Read-ClientXml $reader.ReadToEnd() $file.FullName -Fragment }
            finally { $reader.Dispose() }
        }
        finally { $stream.Dispose() }
        $nodes = @($doc.SelectNodes('/client-fragment/route'))
        if ($nodes.Count -ne 1) { throw "Expected exactly one top-level route in extracted file '$($file.FullName)'." }
        $source = [ordered]@{ kind = 'extracted-client'; path = $file.FullName; sha256 = $hash; entry = $file.FullName }
        Add-UniqueRoute $secondary (Convert-ClientRoute $nodes[0] (Get-AssetWorld $file.FullName) $source)
    }
}
$merged = [Collections.Generic.Dictionary[uint32, object]]::new($secondary)
foreach ($record in $primary.Values) { $merged[$record.route_id] = $record }
$result = [ordered]@{ schema_version = 1; routes = @($merged.Values | Sort-Object route_id) }
$json = ConvertTo-Json -InputObject $result -Depth 6
# CreateNew also prevents accidentally overwriting an input or a concurrent export.
$outputStream = [IO.File]::Open($output, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::None)
try {
    $bytes = [Text.UTF8Encoding]::new($false).GetBytes($json + [Environment]::NewLine)
    $outputStream.Write($bytes, 0, $bytes.Length)
}
finally { $outputStream.Dispose() }
Write-Host "Exported $($merged.Count) routes ($($primary.Count) primary, $($secondary.Count) fallback candidates) to $output"
