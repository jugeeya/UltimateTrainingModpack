@echo off
del /q %UserProfile%\.rustup\fallback
cargo skyline build --release
pause