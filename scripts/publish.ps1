git config core.autocrlf false
git config core.eol lf

git pull origin main --rebase 2>$null

$cargo = Get-Content "Cargo.toml" -Raw
$match = [regex]::Match($cargo, 'version = "([^"]+)"')
if (-not $match.Success) {
    Write-Error "Не удалось найти версию в Cargo.toml"
    exit 1
}
$version = $match.Groups[1].Value
$parts = $version.Split('.')
if ($parts.Length -lt 3) {
    Write-Error "Неверный формат версии: $version"
    exit 1
}
$parts[2] = [int]$parts[2] + 1
$newVersion = $parts -Join '.'

$cargoReplacement = 'version = "' + $newVersion + '"'
(Get-Content "Cargo.toml" -Raw) -replace 'version = "[^"]+"', $cargoReplacement | Set-Content "Cargo.toml" -Encoding UTF8

$jsonReplacement = '"version": "' + $newVersion + '"'
(Get-Content "package.json" -Raw) -replace '"version": "[^"]+"', $jsonReplacement | Set-Content "package.json" -Encoding UTF8

git add .
git commit -m ("chore: release v" + $newVersion) 2>$null
git tag ("v" + $newVersion) 2>$null
git push origin main --tags 2>$null
git push origin master --tags 2>$null
npm publish 2>$null
if ($LASTEXITCODE -ne 0) {
    npm publish --force 2>$null
}
