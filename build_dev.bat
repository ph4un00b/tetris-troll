@echo off

setlocal

:: Ensure you have Cargo and Rust set up in your Windows environment

cargo build --target wasm32-unknown-unknown --release

rmdir /s /q web\assets
del /q web\tetris-troll.wasm

xcopy /e /i resources web\assets
copy target\wasm32-unknown-unknown\release\tetris-troll.wasm web\

dir /s /b web

:: Remember to replace "port_number" with the desired port number
basic-http-server web/

endlocal


