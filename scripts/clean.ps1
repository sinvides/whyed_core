Remove-Item -Recurse -Force target -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force node_modules -ErrorAction SilentlyContinue
Remove-Item -Force package-lock.json -ErrorAction SilentlyContinue
Remove-Item -Force Cargo.lock -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force .cargo -ErrorAction SilentlyContinue
Remove-Item -Force whyedcode.exe -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force bin -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force obj -ErrorAction SilentlyContinue
