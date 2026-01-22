@echo off
cd Tauri
echo Building auxiliary services...
cd src-tauri
cargo build --bin wg_service
cd ..
echo Starting Tauri App...
npm run tauri dev
pause
