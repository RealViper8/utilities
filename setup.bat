@echo off
cls
goto main

:main
echo "Setup"
echo "\n1. build"
echo "2. build-macos"
echo "3. setup-rust"
echo "4. clean"
echo "5. exit"
set /p Input="Number > "
if /i %Input% == "1" goto 1
if /i %Input% == "2" goto 2
if /i %Input% == "3" goto 3
if /i %Input% == "4" goto 4
if /i %Input% == "5" exit 0
goto main
pause > nul

:1
cargo build

:2
cargo build --target x86_64-apple-darwin || echo Failed to build for macos && exit 1

:3
cargo || echo Install Rust && exit 1
cls
cargo r || echo failed

:4
del /Q /S target || echo target not found && exit 1