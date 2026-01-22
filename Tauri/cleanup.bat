@echo off
echo Stopping service...
sc stop NigPingWGEngine
timeout /t 2 /nobreak
echo Killing processes...
taskkill /F /IM wg_service.exe
taskkill /F /IM wg-engine-x86_64-pc-windows-msvc.exe
echo Deleting binary...
del src-tauri\target\debug\wg_service.exe
exit 0
