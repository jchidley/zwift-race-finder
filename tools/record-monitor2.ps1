# Ultra-Refactored Monitor Recorder
param(
    [int]$fps = 8, [int]$pngFps = 1, [int]$duration = 14400,
    [string]$outputDir = "$env:USERPROFILE\Videos\Recordings", [string]$name = "",
    [switch]$pngOnly, [switch]$smartFilter, [string]$resolution = ""
)

# Single-line initialization
if (!(Get-Command ffmpeg -EA 0)) { throw "FFmpeg not found" }
Add-Type -AssemblyName System.Windows.Forms
$monitor = [System.Windows.Forms.Screen]::AllScreens | Where-Object {!$_.Primary} | Select-Object -First 1
if (!$monitor) { throw "No second monitor detected" }

# Path and directory setup
$baseName = $(if($name){"${name}_"}else{""}) + (Get-Date -Format "yyyy-MM-dd_HH-mm-ss")
$videoFile, $pngFolder = "$outputDir\$baseName.mp4", "$outputDir\$baseName"
$null = New-Item @($outputDir, $pngFolder) -ItemType Directory -Force -EA 0

# Display info and build arguments
$b = $monitor.Bounds
$size = if ($resolution) { $resolution } else { "$($b.Width)x$($b.Height)" }
Write-Host "🎥 Monitor: $($b.Width)x$($b.Height) @ ($($b.X),$($b.Y)) → Recording: $size @ ${fps}fps → ${pngFps}fps PNG ($(($duration/3600).ToString('0.0'))h)"
Write-Host "📁 Output: $baseName"

$base = @("-hide_banner", "-loglevel", "warning", "-y", "-rtbufsize", "2048M", "-probesize", "50M", "-f", "gdigrab", "-framerate", $fps, "-offset_x", $b.X, "-offset_y", $b.Y, "-video_size", $size, "-i", "desktop", "-t", $duration)
$filter = $(if($smartFilter){"mpdecimate=hi=200:lo=100:frac=0.33,fps=$pngFps"}else{"fps=$pngFps"})
$pngPath = "$pngFolder\frame_%06d.png"

# Execute based on mode
if ($pngOnly) {
    & ffmpeg @base -vf $filter -q:v 1 $pngPath
} else {
    & ffmpeg @base -c:v libx264 -preset veryfast -tune zerolatency -crf 25 -pix_fmt yuv420p -movflags +faststart $videoFile -vf $filter -q:v 1 $pngPath
}

# Results
$pngCount, $videoSize = @(Get-ChildItem "$pngFolder\*.png" -EA 0).Count, $(if(Test-Path $videoFile){" ($('{0:N1}MB' -f ((Get-Item $videoFile).Length/1MB)))"}else{""})
Write-Host "✅ $pngCount PNG frames$(if(!$pngOnly){" + video$videoSize"}) → $baseName"