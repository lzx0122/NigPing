use std::fs::File;
use std::io::Write;
use std::process::Command;
use tauri::{AppHandle, Runtime};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;

// 定義網卡名稱 (固定方便管理，避免跟系統其他 VPN 衝突)
const INTERFACE_NAME: &str = "NigPingAdapter";

#[tauri::command]
pub async fn connect_korea<R: Runtime>(app: AppHandle<R>, config_content: String, ipv4_address: String) -> Result<String, String> {
    // --- 步驟 1: 準備設定檔 ---
    // 我們需要把字串存成暫存檔，因為 wg-tool 只吃檔案路徑
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("nigping.conf");
    
    let mut file = File::create(&config_path).map_err(|e| format!("無法建立設定檔: {}", e))?;
    file.write_all(config_content.as_bytes()).map_err(|e| format!("無法寫入設定: {}", e))?;
    
    println!("Step 1: 設定檔已寫入 {:?}", config_path);

    // --- 步驟 2: 啟動 wg-engine (建立虛擬網卡) ---
    // 這會在背景執行，不能讓它停下來
    let sidecar_command = app.shell().sidecar("wg-engine").map_err(|e| e.to_string())?;
    let (mut rx, _child) = sidecar_command
        .args(["-tun", INTERFACE_NAME]) // 關鍵參數：告訴它建立一個叫 NigPingAdapter 的網卡
        .spawn()
        .map_err(|e| format!("引擎啟動失敗: {}", e))?;

    // 在背景監聽它的輸出 (Debug 用，方便你除錯)
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line) = event {
                println!("WG-ENGINE: {:?}", String::from_utf8_lossy(&line));
            }
        }
    });

    // 給引擎一點時間建立網卡驅動 (重要！)
    std::thread::sleep(std::time::Duration::from_secs(2));

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
pub fn disconnect_vpn() -> Result<String, String> {
    // 斷線最簡單的方法：直接砍掉那個 Process
    // 我們用 taskkill 強制結束 sidecar
    let _ = Command::new("taskkill")
        .args(["/IM", "wg-engine-x86_64-pc-windows-msvc.exe", "/F"])
        .output();
        
    // 雙重保險：移除網卡
    let _ = Command::new("wg-tool-x86_64-pc-windows-msvc.exe")
        .args(["/uninstalltunnelservice", INTERFACE_NAME])
        .output();

    Ok("VPN 已斷線".to_string())
}