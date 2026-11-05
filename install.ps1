$src = Join-Path $PSScriptRoot "whyedcode.exe"
$destDir = Join-Path $env:USERPROFILE "bin"
if (!(Test-Path $destDir)) { New-Item -ItemType Directory -Path $destDir | Out-Null }
$dest = Join-Path $destDir "whyedcode.exe"
Copy-Item $src -Destination $dest -Force
$path = [Environment]::GetEnvironmentVariable("Path", "User")
if ($path -notlike "*$destDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$path;$destDir", "User")
}
