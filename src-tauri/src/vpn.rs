use std::fs::File;
use std::io::Write;
use std::process::Command;
use tauri::{AppHandle, Runtime};
use tauri_plugin_shell::ShellExt;
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

    // --- 步驟 0.5: 啟用 WireGuard 所需的特殊權限 ---
    // WireGuard 需要這些權限來建立受保護的命名管道 (Named Pipe)
    if let Err(e) = privileges::enable_se_restore_privilege() {
        println!("警告: 無法啟用所需權限: {}", e);
        // 不 return error，因為也許使用者環境不同，這不是絕對必要的，可以嘗試繼續
    } else {
        println!("已啟用 WireGuard 所需權限");
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

    // --- 步驟 2: 使用 Windows 服務啟動 wg-engine (SYSTEM 權限) ---
    println!("Step 2: 設定 Windows 服務...");
    
    // Get the path to wg-engine sidecar
    // Sidecars are in src-tauri/bin/ directory (NOT in target/debug/)
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("無法獲取當前執行檔路徑: {}", e))?
        .parent()
        .ok_or("無法獲取執行檔目錄")?
        .to_path_buf();
    
    // In dev mode: target/debug/nigping.exe -> we need to go to ../../bin
    // In production: the exe and sidecars should be in the same directory
    let engine_path = if cfg!(debug_assertions) {
        // Development mode: go up to src-tauri and into bin
        exe_dir
            .parent().ok_or("無法獲取 target 目錄")?
            .parent().ok_or("無法獲取 src-tauri 目錄")?
            .join("bin")
            .join("wg-engine-x86_64-pc-windows-msvc.exe")
    } else {
        // Production mode: sidecar should be in same directory as exe
        exe_dir.join("wg-engine-x86_64-pc-windows-msvc.exe")
    };

    // Wrapper path (wg_service.exe)
    // In Dev: target/debug/wg_service.exe (same as main exe)
    // In Prod: wg_service.exe (same as main exe)
    let wrapper_path = exe_dir.join("wg_service.exe");
    
    let engine_path_str = engine_path
        .to_str()
        .ok_or("無法轉換路徑為字串")?
        .to_string();

    let wrapper_path_str = wrapper_path
        .to_str()
        .ok_or("無法轉換 wrapper 路徑為字串")?
        .to_string();
    
    println!("wg-engine 路徑: {}", engine_path_str);
    println!("wg-service 路徑: {}", wrapper_path_str);
    
    // Verify file exists before creating service
    if !engine_path.exists() {
        return Err(format!("wg-engine 不存在於: {}", engine_path_str));
    }
    // Check wrapper existence only if we are about to create service, 
    // but better to check early. In dev it might not exist if not built yet, 
    // but the user should have built it.
    if !wrapper_path.exists() {
        return Err(format!("wg-service 不存在於: {} (請確認已編譯 wg_service)", wrapper_path_str));
    }
    
    // Always recreate the service to ensure correct binPath and arguments
    // Ignore error if delete fails (e.g. if it doesn't exist)
    println!("正在重置 Windows 服務...");
    let _ = crate::service_manager::delete_service();
    
    // Create service
    crate::service_manager::create_service(&wrapper_path_str, &engine_path_str, INTERFACE_NAME)?;
    
    // Start the service (this runs with SYSTEM privileges)
    println!("Step 2.5: 啟動服務 (SYSTEM 權限)...");
    crate::service_manager::start_service()?;
    
    // Wait for service to be fully running
    crate::service_manager::wait_for_service_running(10)?;
    
    // Wait a moment for the adapter to be created
    println!("Step 2.8: 等待網卡建立...");
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // Verify adapter exists
    let mut interface_verified = false;
    for i in 0..10 {
        let check_output = Command::new("netsh")
            .args(["interface", "show", "interface", &format!("name=\"{}\"", INTERFACE_NAME)])
            .output();
            
        if let Ok(output) = check_output {
            if output.status.success() {
                interface_verified = true;
                println!("✓ 網卡已確認存在！(嘗試: {})", i + 1);
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    if !interface_verified {
         return Err("服務已啟動但網卡未出現，請確認 WireGuard 驅動已安裝".to_string());
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

    // --- 步驟 5: 設定路由 (Split Tunneling) ---
    println!("Step 5: 設定路由...");
    // 找出設定檔中的 AllowedIPs 行
    let allowed_ips_line = config_content
        .lines()
        .find(|line| line.trim().starts_with("AllowedIPs"))
        .unwrap_or("");

    if !allowed_ips_line.is_empty() {
        // 解析 AllowedIPs (例如: AllowedIPs = 1.2.3.4/32, 5.6.7.0/24)
        // 1. 去掉 "AllowedIPs" 和 "="
        // 2. 用逗號分割
        // 3. 針對每個 IP/CIDR 加路由
        let ips: Vec<&str> = allowed_ips_line
            .split('=')
            .nth(1)
            .unwrap_or("")
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        // Get the interface index for NigPingAdapter
        let if_index_output = Command::new("netsh")
            .args(["interface", "ipv4", "show", "interfaces"])
            .output()
            .map_err(|e| format!("無法獲取介面索引: {}", e))?;

        let if_output_text = String::from_utf8_lossy(&if_index_output.stdout);
        let mut interface_idx = None;
        
        // Parse output to find interface index
        for line in if_output_text.lines() {
            if line.contains(INTERFACE_NAME) {
                // Extract the interface index (first number in the line)
                let parts: Vec<&str> = line.split_whitespace().collect();
                if !parts.is_empty() {
                    if let Ok(idx) = parts[0].parse::<u32>() {
                        interface_idx = Some(idx);
                        println!("✓ 找到介面索引: {}", idx);
                        break;
                    }
                }
            }
        }

        let if_idx = interface_idx.ok_or("無法找到 VPN 介面索引")?;

        for ip_cidr in ips {
            // Parse CIDR notation (e.g., "13.124.0.0/16" -> network and mask)
            let parts: Vec<&str> = ip_cidr.split('/').collect();
            if parts.len() != 2 {
                println!("警告: 無效的 CIDR 格式: {}", ip_cidr);
                continue;
            }

            let network = parts[0];
            let prefix_len: u32 = match parts[1].parse() {
                Ok(p) => p,
                Err(_) => {
                    println!("警告: 無效的前綴長度: {}", parts[1]);
                    continue;
                }
            };

            // Convert prefix length to netmask
            let netmask = prefix_to_netmask(prefix_len);

            println!("正在加入路由: {} (mask: {})", network, netmask);
            
            // Use "route add" command with IF parameter
            // Format: route add <network> mask <netmask> 0.0.0.0 IF <interface_index> METRIC 1
            let route_output = Command::new("route")
                .args([
                    "add",
                    network,
                    "mask",
                    &netmask,
                    "0.0.0.0",
                    "IF",
                    &if_idx.to_string(),
                    "METRIC",
                    "1"
                ])
                .output()
                .map_err(|e| format!("無法加入路由 {}: {}", ip_cidr, e))?;

            if !route_output.status.success() {
                let err_msg = String::from_utf8_lossy(&route_output.stderr);
                // 有些路由可能已經存在，這邊只印警告不中斷
                println!("警告: 無法加入路由 {} -> {}", ip_cidr, err_msg);
            } else {
                println!("✓ 路由已加入: {}", ip_cidr);
            }
        }
    } else {
        println!("警告: 設定檔中找不到 AllowedIPs，未加入任何路由 (Split Tunneling 無效)");
    }

    Ok("韓服連線成功！GLHF!".to_string())
}

#[tauri::command]
pub fn disconnect_vpn() -> Result<String, String> {
    println!("正在斷開 VPN...");
    
    // 停止 Windows 服務
    crate::service_manager::stop_service()?;
    
    // 可選：刪除服務（如果您想要每次都重新創建）
    // crate::service_manager::delete_service()?;
    
    println!("✓ VPN 已斷線");
    Ok("VPN 已斷線".to_string())
}

// Helper function to convert CIDR prefix length to netmask
fn prefix_to_netmask(prefix: u32) -> String {
    if prefix > 32 {
        return "255.255.255.255".to_string();
    }
    
    let mask: u32 = if prefix == 0 {
        0
    } else {
        0xFFFFFFFF << (32 - prefix)
    };
    
    format!(
        "{}.{}.{}.{}",
        (mask >> 24) & 0xFF,
        (mask >> 16) & 0xFF,
        (mask >> 8) & 0xFF,
        mask & 0xFF
    )
}