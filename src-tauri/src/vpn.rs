use std::fs::File;
use std::io::Write;
use std::process::Command;
use tauri::{AppHandle, Runtime};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;
use crate::privileges;

// 定義網卡名稱 (固定方便管理，避免跟系統其他 VPN 衝突)
const INTERFACE_NAME: &str = "NigPingAdapter";

#[tauri::command]
pub async fn connect_korea<R: Runtime>(app: AppHandle<R>, config_content: String, ipv4_address: String) -> Result<String, String> {
    // --- 步驟 0: 檢查管理員權限 ---
    let admin_check = Command::new("net")
        .args(["session"])
        .output()
        .map_err(|e| format!("無法檢查權限: {}", e))?;

    if !admin_check.status.success() {
        return Err("請以「系統管理員身分」執行此程式，否則無法建立 VPN 連線。".to_string());
    }

    // --- 步驟 0.5: 啟用 SeRestorePrivilege ---
    // 這是 wg-engine 建立 Pipe 所需的特殊權限
    if let Err(e) = privileges::enable_se_restore_privilege() {
        println!("警告: 無法啟用 SeRestorePrivilege: {}", e);
        // 不 return error，因為也許使用者環境不同，這不是絕對必要的，可以嘗試繼續
    } else {
        println!("已啟用 SeRestorePrivilege 權限");
    }

    // --- 步驟 1: 準備設定檔 ---
    // 我們需要把字串存成暫存檔，因為 wg-tool 只吃檔案路徑
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("nigping.conf");
    
    // Filter out unsupported lines
    let filtered_config: String = config_content
        .lines()
        .filter(|line| {
            let trim = line.trim();
            !trim.starts_with("Address")
                && !trim.starts_with("DNS")
                && !trim.starts_with("MTU")
                && !trim.starts_with("PreUp")
                && !trim.starts_with("PostUp")
                && !trim.starts_with("PreDown")
                && !trim.starts_with("PostDown")
                && !trim.starts_with("Table")
                && !trim.starts_with("SaveConfig")
        })
        .collect::<Vec<&str>>()
        .join("\n");

    let mut file = File::create(&config_path).map_err(|e| format!("無法建立設定檔: {}", e))?;
    file.write_all(filtered_config.as_bytes()).map_err(|e| format!("無法寫入設定: {}", e))?;
    
    println!("Step 1: 設定檔已寫入 {:?}", config_path);

    // --- 步驟 2: 啟動 wg-engine (建立虛擬網卡) ---
    // 這會在背景執行，不能讓它停下來
    let sidecar_command = app.shell().sidecar("wg-engine").map_err(|e| e.to_string())?;
    let (mut rx, _child) = sidecar_command
        .args([INTERFACE_NAME]) // 修正：Windows 版 wg-engine 直接吃網卡名稱，不用 -tun 參數
        .spawn()
        .map_err(|e| format!("引擎啟動失敗: {}", e))?;

    // 在背景監聽它的輸出 (Debug 用，方便你除錯)
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
             match event {
                CommandEvent::Stdout(line) => println!("WG-ENGINE: {:?}", String::from_utf8_lossy(&line)),
                CommandEvent::Stderr(line) => println!("WG-ENGINE (ERR): {:?}", String::from_utf8_lossy(&line)),
                CommandEvent::Terminated(payload) => {
                     println!("WG-ENGINE (EXIT): 引擎意外終止！代碼: {:?}, 訊號: {:?}", payload.code, payload.signal);
                }
                _ => {}
            }
        }
    });

    // 給引擎一點時間建立網卡驅動 (重要！)
    // 使用 Polling (輪詢) 確認網卡是否已經出現
    println!("Step 2.5: 等待網卡建立...");
    let mut interface_ready = false;
    for i in 0..10 {
        let check_output = Command::new("netsh")
            .args(["interface", "show", "interface", &format!("name=\"{}\"", INTERFACE_NAME)])
            .output();
            
        if let Ok(output) = check_output {
            if output.status.success() {
                interface_ready = true;
                println!("網卡已偵測到！(嘗試次數: {})", i + 1);
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    if !interface_ready {
         return Err("無法建立虛擬網卡 (Timeout)，請確認是否安裝 WireGuard 驅動或是 wintun.dll".to_string());
    }

    // --- 步驟 3: 注入設定 (Keys & Peers) ---
    // 指令等同於: wg.exe setconf NigPingAdapter C:\temp\nigping.conf
    println!("Step 3: 注入設定...");
    let wg_tool = app.shell().sidecar("wg-tool").map_err(|e| e.to_string())?;
    let output = wg_tool
        .args(["setconf", INTERFACE_NAME, config_path.to_str().unwrap()])
        .output()
        .await
        .map_err(|e| format!("設定注入失敗: {}", e))?;

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("wg-tool 錯誤: {}", err_msg));
    }

    // --- 步驟 4: 設定 Windows IP (Netsh) ---
    // 因為我們沒有用 wg-quick，所以要自己用 Windows 指令設 IP
    // 指令: netsh interface ip set address "NigPingAdapter" static 10.13.13.2 255.255.255.255
    println!("Step 4: 設定 IP ({}) ...", ipv4_address);
    let netsh_output = Command::new("netsh")
        .args([
            "interface", "ip", "set", "address",
            &format!("name=\"{}\"", INTERFACE_NAME),
            "static", &ipv4_address, "255.255.255.255"
        ])
        .output() // 這裡用標準 Rust Command 即可，因為 netsh 是系統內建
        .map_err(|e| format!("Netsh 執行失敗: {}", e))?;

    if !netsh_output.status.success() {
        let err_msg = String::from_utf8_lossy(&netsh_output.stderr);
        return Err(format!("IP 設定失敗 (請確認有管理員權限): {}", err_msg));
    }

    Ok("韓服連線成功！GLHF!".to_string())
}

#[tauri::command]
pub fn disconnect_vpn<R: Runtime>(app: AppHandle<R>) -> Result<String, String> {
   // 1. 強制結束 Engine
    let _ = Command::new("taskkill")
        .args(["/IM", "wg-engine-x86_64-pc-windows-msvc.exe", "/F"])
        .output();
        
    // 2. 為了保險起見，使用正確的路徑移除服務 (需注入 AppHandle)
    // 注意：這需要把函式簽名改成 fn disconnect_vpn<R: Runtime>(app: AppHandle<R>)
    let _ = app.shell().sidecar("wg-tool")
         .map(|c| c.args(["/uninstalltunnelservice", INTERFACE_NAME]).spawn());

    Ok("VPN 已斷線".to_string())
}