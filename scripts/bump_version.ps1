$cargo = Get-Content "Cargo.toml" -Raw
$start = $cargo.IndexOf('version = "') + 11
$end = $cargo.IndexOf('"', $start)
$version = $cargo.Substring($start, $end - $start)
$parts = $version.Split('.')
$parts[2] = [int]$parts[2] + 1
$newVersion = $parts -Join '.'

$oldCargo = 'version = "' + $version + '"'
$newCargo = 'version = "' + $newVersion + '"'
$cargoNew = $cargo -replace [regex]::Escape($oldCargo), $newCargo
Set-Content "Cargo.toml" $cargoNew -Encoding UTF8

$oldJson = '"version": "' + $version + '"'
$newJson = '"version": "' + $newVersion + '"'
$json = Get-Content "package.json" -Raw
$jsonNew = $json -replace [regex]::Escape($oldJson), $newJson
Set-Content "package.json" $jsonNew -Encoding UTF8

Write-Output $newVersion
