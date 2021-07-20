@echo off
del /q %UserProfile%\.rustup\fallback
cargo skyline install
pause