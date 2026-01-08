use std::process::Command;
use tauri::command;

/// 获取已安装的 Node 版本列表
#[command]
pub fn list_installed_versions() -> Result<String, String> {
    let output = Command::new("fnm")
        .arg("list")
        .output()
        .map_err(|e| format!("执行 fnm list 失败: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    String::from_utf8(output.stdout).map_err(|e| e.to_string())
}

/// 获取远程可用版本列表
#[command]
pub fn list_remote_versions(lts_only: bool) -> Result<String, String> {
    let mut cmd = Command::new("fnm");
    cmd.arg("list-remote");

    if lts_only {
        cmd.arg("--lts");
    }

    let output = cmd
        .output()
        .map_err(|e| format!("执行 fnm list-remote 失败: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    String::from_utf8(output.stdout).map_err(|e| e.to_string())
}

/// 安装指定版本
#[command]
pub async fn install_version(version: String) -> Result<String, String> {
    let output = Command::new("fnm")
        .arg("install")
        .arg(&version)
        .output()
        .map_err(|e| format!("执行 fnm install 失败: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(format!("成功安装 {}", version))
}

/// 卸载指定版本
#[command]
pub fn uninstall_version(version: String) -> Result<String, String> {
    let output = Command::new("fnm")
        .arg("uninstall")
        .arg(&version)
        .output()
        .map_err(|e| format!("执行 fnm uninstall 失败: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(format!("成功卸载 {}", version))
}

/// 切换使用的 Node 版本
#[command]
pub fn use_version(version: String) -> Result<String, String> {
    let output = Command::new("fnm")
        .arg("use")
        .arg(&version)
        .output()
        .map_err(|e| format!("执行 fnm use 失败: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(format!("已切换到 {}", version))
}

/// 获取当前使用的 Node 版本
#[command]
pub fn get_current_version() -> Result<String, String> {
    let output = Command::new("fnm")
        .arg("current")
        .output()
        .map_err(|e| format!("执行 fnm current 失败: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    String::from_utf8(output.stdout)
        .map(|s| s.trim().to_string())
        .map_err(|e| e.to_string())
}

/// 设置默认 Node 版本
#[command]
pub fn set_default_version(version: String) -> Result<String, String> {
    let output = Command::new("fnm")
        .arg("default")
        .arg(&version)
        .output()
        .map_err(|e| format!("执行 fnm default 失败: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(format!("已将 {} 设为默认版本", version))
}
