use std::process::Command;

const SERVICE_NAME: &str = "NigPingWGEngine";
#[allow(dead_code)]
const SERVICE_DISPLAY_NAME: &str = "NigPing WireGuard Engine";

/// Create a Windows service for WireGuard engine
pub fn create_service(wrapper_path: &str, engine_path: &str, adapter_name: &str) -> Result<(), String> {
    // First check if service already exists
    if service_exists()? {
        println!("服務已存在，跳過創建");
        return Ok(());
    }

    println!("正在創建 Windows 服務...");
    
    // sc.exe create requires exact format: key=value (no space after =)
    // binPath should be: "wrapper_path" "engine_path" "adapter_name"
    let bin_path = format!("\"{}\" \"{}\" \"{}\"", wrapper_path, engine_path, adapter_name);
    
    let create_args = [
        "create",
        SERVICE_NAME,
        &format!("binPath={}", bin_path),  // No space after =
        "start=demand",                     // No space after =
        "DisplayName=NigPing WireGuard Engine",  // No space after =
    ];
    
    println!("執行命令: sc.exe {:?}", create_args);
    
    let output = Command::new("sc.exe")
        .args(create_args)
        .output()
        .map_err(|e| format!("無法執行 sc.exe create: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    println!("sc.exe stdout: {}", stdout);
    println!("sc.exe stderr: {}", stderr);
    println!("sc.exe exit code: {:?}", output.status.code());

    if !output.status.success() {
        return Err(format!("創建服務失敗: stdout={} stderr={}", stdout, stderr));
    }

    println!("✓ 服務創建成功");
    Ok(())
}

/// Start the Windows service
pub fn start_service() -> Result<(), String> {
    println!("正在啟動 Windows 服務...");
    
    let output = Command::new("sc.exe")
        .args(["start", SERVICE_NAME])
        .output()
        .map_err(|e| format!("無法執行 sc.exe start: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    println!("sc.exe start stdout: {}", stdout);
    println!("sc.exe start stderr: {}", stderr);
    println!("sc.exe start exit code: {:?}", output.status.code());

    if !output.status.success() {
        // Check if already started
        if stdout.contains("已經啟動") || stderr.contains("已經啟動") {
            println!("⚠️  服務已經在運行中");
            return Ok(());
        }
        
        return Err(format!("啟動服務失敗: stdout={} stderr={}", stdout, stderr));
    }

    println!("✓ 服務啟動成功");
    Ok(())
}

/// Stop the Windows service
pub fn stop_service() -> Result<(), String> {
    println!("正在停止 Windows 服務...");
    
    let output = Command::new("sc.exe")
        .args(["stop", SERVICE_NAME])
        .output()
        .map_err(|e| format!("無法執行 sc stop: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Check if already stopped
        if stdout.contains("未啟動") || stderr.contains("未啟動") || stdout.contains("1062") {
            println!("⚠️  服務未在運行");
            return Ok(());
        }
        
        return Err(format!("停止服務失敗: {} {}", stdout, stderr));
    }

    println!("✓ 服務已停止");
    Ok(())
}

/// Delete the Windows service
#[allow(dead_code)]
pub fn delete_service() -> Result<(), String> {
    println!("正在刪除 Windows 服務...");
    
    // Stop first if running
    let _ = stop_service();
    
    // Wait a moment for service to fully stop
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    let output = Command::new("sc.exe")
        .args(["delete", SERVICE_NAME])
        .output()
        .map_err(|e| format!("無法執行 sc delete: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Check if doesn't exist
        if stdout.contains("不存在") || stderr.contains("不存在") || stdout.contains("1060") {
            println!("⚠️  服務不存在");
            return Ok(());
        }
        
        return Err(format!("刪除服務失敗: {} {}", stdout, stderr));
    }

    println!("✓ 服務已刪除");
    Ok(())
}

/// Check if service exists
pub fn service_exists() -> Result<bool, String> {
    let output = Command::new("sc.exe")
        .args(["query", SERVICE_NAME])
        .output()
        .map_err(|e| format!("無法執行 sc query: {}", e))?;

    // If query succeeds, service exists
    Ok(output.status.success())
}

/// Wait for service to be running
pub fn wait_for_service_running(timeout_secs: u64) -> Result<(), String> {
    let start = std::time::Instant::now();
    
    while start.elapsed().as_secs() < timeout_secs {
        let output = Command::new("sc.exe")
            .args(["query", SERVICE_NAME])
            .output()
            .map_err(|e| format!("無法執行 sc query: {}", e))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("RUNNING") || stdout.contains("執行中") {
                println!("✓ 服務已進入運行狀態");
                return Ok(());
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Err(format!("等待服務啟動超時 ({} 秒)", timeout_secs))
}
