$cargo = Get-Content "Cargo.toml"
$version = ($cargo | Select-String "version = ").ToString().Split('"')[1]
$parts = $version.Split('.')
$parts[2] = [int]$parts[2] + 1
$newVersion = $parts -Join '.'

(Get-Content "Cargo.toml") -replace 'version = ".*"', "version = `"$newVersion`"" | Set-Content "Cargo.toml"
(Get-Content "package.json") -replace '"version": ".*"', '"version": "' + $newVersion + '"' | Set-Content "package.json"

git add .
git commit -m "chore: release v$newVersion"
git tag "v$newVersion"
git push origin main --tags
git push origin master --tags 2>$null
npm publish
