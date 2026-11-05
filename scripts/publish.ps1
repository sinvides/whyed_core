git config core.autocrlf false
git config core.eol lf

git pull origin main --rebase 2>$null

& scripts/bump_version.ps1

git add .
git commit -m ("chore: release v" + $newVersion) 2>$null
git tag ("v" + $newVersion) 2>$null
git push origin main --tags 2>$null
npm publish 2>$null
if ($LASTEXITCODE -ne 0) {
    npm publish --force 2>$null
}
