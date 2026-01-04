# watch_me_front

## dev
dx serve
http://127.0.0.1:8080/watch_me_front/

## build
dx build --release --platform web

// Linux
rm -rf docs
cp -r target/dx/watch_me_front/release/web/public docs

// Windows (PowerShell)
Remove-Item -Recurse -Force docs
Copy-Item -Recurse -Force target\dx\watch_me_front\release\web\public docs

git add docs
git commit -m "deploy wasm"
git push
