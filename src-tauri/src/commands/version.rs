use super::common::create_fnm_command;
use std::fs;
use std::path::PathBuf;
use tauri::command;

/// 获取已安装的 Node 版本列表
#[command]
pub fn list_installed_versions() -> Result<String, String> {
    let mut cmd = create_fnm_command()?;
    let output = cmd
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
pub fn list_remote_versions(lts_only: bool, filter: Option<String>) -> Result<String, String> {
    let mut cmd = create_fnm_command()?;
    cmd.arg("list-remote");
    cmd.arg("--sort").arg("desc"); // 最新版本排在前面

    if lts_only {
        cmd.arg("--lts");
    }

    // 支持服务端过滤
    if let Some(ref keyword) = filter {
        if !keyword.is_empty() {
            cmd.arg("--filter").arg(keyword);
        }
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
    let mut cmd = create_fnm_command()?;
    let output = cmd
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
    let mut cmd = create_fnm_command()?;
    let output = cmd
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
    let mut cmd = create_fnm_command()?;
    let output = cmd
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
    // 首先尝试 fnm current
    let mut cmd = create_fnm_command()?;
    let output = cmd
        .arg("current")
        .output()
        .map_err(|e| format!("执行 fnm current 失败: {}", e))?;

    if output.status.success() {
        return String::from_utf8(output.stdout)
            .map(|s| s.trim().to_string())
            .map_err(|e| e.to_string());
    }

    // 如果 fnm current 失败（通常在 GUI 环境中），尝试读取 default 别名
    get_default_version_from_alias()
}

/// 从 fnm 别名目录读取默认版本
fn get_default_version_from_alias() -> Result<String, String> {
    let fnm_dir = get_fnm_dir();
    if fnm_dir.is_empty() {
        return Err("无法确定 fnm 目录".to_string());
    }

    let default_alias = PathBuf::from(&fnm_dir).join("aliases").join("default");

    if default_alias.exists() {
        // default 是一个符号链接，指向实际版本目录
        match fs::read_link(&default_alias) {
            Ok(target) => {
                // 从路径中提取版本号
                if let Some(version) = target.file_name() {
                    return Ok(version.to_string_lossy().to_string());
                }
            }
            Err(_) => {
                // 如果不是符号链接，尝试直接读取
                if let Ok(content) = fs::read_to_string(&default_alias) {
                    return Ok(content.trim().to_string());
                }
            }
        }
    }

    // 返回 "none" 表示没有设置默认版本
    Ok("none".to_string())
}

/// 获取 fnm 目录路径
fn get_fnm_dir() -> String {
    let home = dirs::home_dir();

    #[cfg(target_os = "macos")]
    {
        if let Some(home_dir) = home {
            return home_dir
                .join("Library/Application Support/fnm")
                .to_string_lossy()
                .to_string();
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(local_appdata) = std::env::var("LOCALAPPDATA") {
            return format!("{}\\fnm", local_appdata);
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Some(home_dir) = home {
            return home_dir
                .join(".local/share/fnm")
                .to_string_lossy()
                .to_string();
        }
    }

    String::new()
}

/// 设置默认 Node 版本
#[command]
pub fn set_default_version(version: String) -> Result<String, String> {
    let mut cmd = create_fnm_command()?;
    let output = cmd
        .arg("default")
        .arg(&version)
        .output()
        .map_err(|e| format!("执行 fnm default 失败: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(format!("已将 {} 设为默认版本", version))
}
