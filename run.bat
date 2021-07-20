@echo off
del /q %UserProfile%\.rustup\fallback
cargo skyline run
pause