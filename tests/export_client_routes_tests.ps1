#Requires -Version 7.0
# Standalone tests: no Pester installation, network, database, or real client data.
[CmdletBinding()]
param(
    [string] $CollectorPath = (Join-Path $PSScriptRoot '../scripts/export_client_routes.ps1')
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Join-Path ([IO.Path]::GetTempPath()) ("client-route-tests-" + [guid]::NewGuid().ToString('N'))
$null = New-Item -ItemType Directory -Path $root
$script:assertions = 0

function Assert-That([bool] $Condition, [string] $Message) {
    $script:assertions++
    if (-not $Condition) { throw "Assertion failed: $Message" }
}

function New-Archive([string] $Xml, [string] $Entry = 'world1.routes.xml') {
    $path = Join-Path $root ([guid]::NewGuid().ToString('N') + '.zip')
    $zip = [IO.Compression.ZipFile]::Open($path, [IO.Compression.ZipArchiveMode]::Create)
    try {
        $writer = [IO.StreamWriter]::new($zip.CreateEntry($Entry).Open(), [Text.UTF8Encoding]::new($false))
        try { $writer.Write($Xml) } finally { $writer.Dispose() }
    } finally { $zip.Dispose() }
    return $path
}

function Invoke-Export([string] $Archive, [string] $Directory) {
    $out = Join-Path $root ([guid]::NewGuid().ToString('N') + '.json')
    $args = @{ ClientArchive = $Archive; OutputPath = $out }
    if ($Directory) { $args.ExtractedDirectory = $Directory }
    & $CollectorPath @args
    return [pscustomobject]@{ Path = $out; Data = (Get-Content -LiteralPath $out -Raw | ConvertFrom-Json) }
}

function Assert-Rejected([string] $Archive, [string] $Directory, [string] $Pattern) {
    $out = Join-Path $root ([guid]::NewGuid().ToString('N') + '.json')
    $args = @{ ClientArchive = $Archive; OutputPath = $out }
    if ($Directory) { $args.ExtractedDirectory = $Directory }
    $message = $null
    try { & $CollectorPath @args } catch { $message = $_.Exception.Message }
    Assert-That ($null -ne $message -and $message -match $Pattern) "expected '$Pattern', got '$message'"
    Assert-That (-not (Test-Path -LiteralPath $out)) 'invalid input must not create output'
}

function Wrap-Routes([string] $Routes) { return "<routes><route file='routes0.xml'>$Routes</route></routes>" }
function Write-Fragment([string] $Directory, [string] $Name, [string] $Xml) {
    $null = New-Item -ItemType Directory -Path $Directory -Force
    [IO.File]::WriteAllText((Join-Path $Directory $Name), $Xml, [Text.UTF8Encoding]::new($false))
}

try {
    $fallback = Join-Path $root 'extracted/Worlds/world1/routes'
    Write-Fragment $fallback 'routes0.xml' @'
<?xml version="1.0" encoding="UTF-8"?>
<VERSION>1</VERSION>
<route nameHash="4294967295" name="Old name" distanceInMeters="9999" ascentInMeters="999" leadinDistanceInMeters="999"/>
<leveleditorinfo><routenodes/></leveleditorinfo>
'@
    Write-Fragment $fallback 'routes1.xml' @'
<?xml version="1.0" encoding="UTF-8"?>
<VERSION>1</VERSION><route nameHash="2" name="Fallback" distanceInMeters="1234.56789" leadinDistanceInMeters="0" meetupLeadinAscentInMeters="0"/>
'@
    $primary = New-Archive @'
<routes>
 <route file="../data/Worlds/world1/routes/routes0.xml"><VERSION>0</VERSION>
  <route nameHash="-1" name="  Primary &amp; precise  " distanceInMeters="1965.4321987654" ascentInMeters="17.123456789" leadinAscentInMeters="0" freeRideLeadinDistanceInMeters="94.7654321" freeRideLeadinAscentInMeters="0.125" meetupLeadinDistanceInMeters="0" eventOnly="0" sportType="1" supportedLaps="1,2"/>
 </route>
 <route file="../data/Worlds/world1/routes/routes2.xml"><route nameHash="0" name="Zero ID" distanceInMeters="0.001" ascentInMeters="0"/></route>
</routes>
'@
    # Parsing must be invariant under a comma-decimal process culture.
    $culture = [Globalization.CultureInfo]::CurrentCulture
    try {
        [Globalization.CultureInfo]::CurrentCulture = [Globalization.CultureInfo]::GetCultureInfo('fr-FR')
        $export = Invoke-Export $primary (Split-Path (Split-Path (Split-Path $fallback)))
    } finally { [Globalization.CultureInfo]::CurrentCulture = $culture }
    $routes = @($export.Data.routes)
    Assert-That ($export.Data.schema_version -eq 1 -and $routes.Count -eq 3) 'version and merged route count'
    Assert-That (($routes.route_id -join ',') -eq '0,2,4294967295') 'numeric ID sorting and signed hash normalization'
    $p = $routes[2]
    Assert-That ($p.name -ceq 'Primary & precise') 'primary record wins and names are trimmed'
    Assert-That ([Math]::Abs($p.distance_km - 1.9654321987654) -lt 1e-14) 'fractional metres retained as km'
    Assert-That ($p.elevation_m -eq 17.123456789) 'fractional ascent retained'
    Assert-That ($null -eq $p.lead_in_distance_km) 'missing primary field not filled by fallback'
    Assert-That ($p.lead_in_elevation_m -eq 0 -and $null -ne $p.lead_in_elevation_m) 'explicit zero ascent retained'
    Assert-That ($p.lead_in_distance_free_ride_km -eq 0.0947654321 -and $p.lead_in_elevation_free_ride_m -eq 0.125) 'free ride fields'
    Assert-That ($p.lead_in_distance_meetups_km -eq 0 -and $null -eq $p.lead_in_elevation_meetups_m) 'meetup zero versus missing'
    Assert-That ($p.event_only -ceq '0' -and $p.sport_type -ceq '1' -and $p.supported_laps -ceq '1,2') 'metadata strings retained'
    Assert-That ($p.world -ceq 'Watopia' -and $p.surface -ceq 'road' -and $null -eq $p.slug) 'world and fixed schema values'
    Assert-That ($p.source.kind -ceq 'client-cache' -and $p.source.path -ceq $primary -and $p.source.entry -ceq 'world1.routes.xml') 'primary provenance'
    Assert-That ($p.source.sha256 -ceq (Get-FileHash -LiteralPath $primary -Algorithm SHA256).Hash.ToLowerInvariant()) 'archive hash'
    $f = $routes[1]
    $fallbackFile = Join-Path $fallback 'routes1.xml'
    Assert-That ($f.source.kind -ceq 'extracted-client' -and $f.source.path -ceq $fallbackFile -and $f.source.entry -ceq $fallbackFile) 'fallback provenance'
    Assert-That ($f.source.sha256 -ceq (Get-FileHash -LiteralPath $fallbackFile -Algorithm SHA256).Hash.ToLowerInvariant()) 'file hash'
    Assert-That ($null -eq $f.elevation_m -and $null -eq $f.event_only -and $f.lead_in_distance_km -eq 0 -and $null -ne $f.lead_in_distance_km) 'missing and explicit zero remain distinct'
    $again = Invoke-Export $primary (Split-Path (Split-Path (Split-Path $fallback)))
    Assert-That ([IO.File]::ReadAllText($again.Path) -ceq [IO.File]::ReadAllText($export.Path)) 'deterministic output'

    $unknown = Invoke-Export (New-Archive (Wrap-Routes '<route nameHash="2147483648" name="Unknown" distanceInMeters="1000"/>') 'Worlds/world99/routes/routes0.xml')
    Assert-That ($unknown.Data.routes[0].world -ceq 'world99') 'unknown world preserved'
    $worlds = @('Watopia','Richmond','London','New York','Innsbruck','Bologna','Yorkshire','Crit City','Makuri Islands','France','Paris','Gravel Mountain','Scotland')
    for ($i = 1; $i -le 13; $i++) {
        $mapped = Invoke-Export (New-Archive (Wrap-Routes '<route nameHash="1" name="Map" distanceInMeters="1"/>') "world$i.routes.xml")
        Assert-That ($mapped.Data.routes[0].world -ceq $worlds[$i - 1]) "asset world$i mapping"
    }

    foreach ($attributes in @(
        'nameHash="4294967296" distanceInMeters="1"',
        'nameHash="-2147483649" distanceInMeters="1"',
        'nameHash="abc" distanceInMeters="1"',
        'nameHash="1" distanceInMeters="0"',
        'nameHash="1" distanceInMeters="-1"',
        'nameHash="1" distanceInMeters="NaN"',
        'nameHash="1" distanceInMeters="Infinity"',
        'nameHash="1" distanceInMeters="1e999"',
        'nameHash="1" distanceInMeters=""',
        'nameHash="1"',
        'nameHash="1" distanceInMeters="1,5"',
        'nameHash="1" distanceInMeters="1" ascentInMeters="-0.1"',
        'nameHash="1" distanceInMeters="1" leadinDistanceInMeters="NaN"',
        'nameHash="1" distanceInMeters="1" freeRideLeadinAscentInMeters="-1"',
        'nameHash="1" distanceInMeters="1" meetupLeadinDistanceInMeters="Infinity"'
    )) {
        Assert-Rejected (New-Archive (Wrap-Routes "<route name='Invalid' $attributes/>")) '' 'Invalid|Missing'
    }
    Assert-Rejected (New-Archive (Wrap-Routes '<route nameHash="1" distanceInMeters="1"/>')) '' 'Missing route name'
    Assert-Rejected (New-Archive '<routes><route>') '' 'Invalid client XML'
    Assert-Rejected (New-Archive '<routes/>') '' 'Expected /routes/route/route'
    Assert-Rejected (New-Archive (Wrap-Routes '<route nameHash="1" name="One" distanceInMeters="1"/><route nameHash="2" name="Two" distanceInMeters="1"/>')) '' 'exactly one inner route'
    Assert-Rejected (New-Archive '<!DOCTYPE routes [<!ENTITY x SYSTEM "file:///nonexistent">]><routes/>') '' 'Invalid client XML'
    Assert-Rejected (New-Archive (Wrap-Routes '<route nameHash="1" name="One" distanceInMeters="1"/>') 'routes.xml') '' 'asset world ID'
    $wrapperWorld = Invoke-Export (New-Archive '<routes><route file="../data/Worlds/world4/routes/routes0.xml"><route nameHash="1" name="Path" distanceInMeters="1"/></route></routes>' 'routes.xml')
    Assert-That ($wrapperWorld.Data.routes[0].world -ceq 'New York') 'world from archive wrapper path'
    Assert-Rejected (New-Archive '<routes><route file="../data/Worlds/world4/routes/routes0.xml"><route nameHash="1" name="Conflict" distanceInMeters="1"/></route></routes>') '' 'consistent world'
    Assert-Rejected (New-Archive '<unrelated/>' 'notes.xml') '' 'No client route XML'
    $emptyDirectory = Join-Path $root 'empty'
    $null = New-Item -ItemType Directory -Path $emptyDirectory
    Assert-Rejected $primary $emptyDirectory 'No routes\*.xml'
    Write-Fragment $fallback 'routes2.xml' '<VERSION>1</VERSION><route nameHash="3" name="Bad fallback" distanceInMeters="-1"/>'
    Assert-Rejected $primary (Split-Path (Split-Path (Split-Path $fallback))) 'Invalid.*distanceInMeters'
    [IO.File]::Delete((Join-Path $fallback 'routes2.xml'))
    $duplicate = New-Archive '<routes><route><route nameHash="-1" name="One" distanceInMeters="1"/></route><route><route nameHash="4294967295" name="Two" distanceInMeters="2"/></route></routes>'
    Assert-Rejected $duplicate '' 'Duplicate route ID 4294967295'
    Write-Fragment $fallback 'routes2.xml' '<VERSION>1</VERSION><route nameHash="2" name="Duplicate" distanceInMeters="1"/>'
    Assert-Rejected $primary (Split-Path (Split-Path (Split-Path $fallback))) 'Duplicate route ID 2'
    [IO.File]::Delete((Join-Path $fallback 'routes2.xml'))
    Write-Fragment $fallback 'routes2.xml' '<?xml version="1.0"?><VERSION>1</VERSION><?xml version="1.0"?><route/>'
    Assert-Rejected $primary (Split-Path (Split-Path (Split-Path $fallback))) 'Invalid client XML'
    [IO.File]::Delete((Join-Path $fallback 'routes2.xml'))
    $before = (Get-FileHash -LiteralPath $export.Path).Hash
    $message = $null
    try { & $CollectorPath -ClientArchive $primary -OutputPath $export.Path } catch { $message = $_.Exception.Message }
    Assert-That ($message -match 'already exists') 'existing output rejected'
    Assert-That ((Get-FileHash -LiteralPath $export.Path).Hash -ceq $before) 'existing output unchanged'
    Write-Host "PASS: $script:assertions assertions"
}
finally { Remove-Item -LiteralPath $root -Recurse -Force }
