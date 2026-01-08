use std::path::PathBuf;
use std::process::Command;
use tauri::command;

/// 获取 fnm 根目录路径
#[command]
pub fn get_fnm_dir() -> Result<String, String> {
    get_fnm_base_dir()
}

/// 获取指定版本的安装目录
#[command]
pub fn get_version_dir(version: String) -> Result<String, String> {
    let fnm_dir = get_fnm_base_dir()?;
    let version_path = PathBuf::from(&fnm_dir)
        .join("node-versions")
        .join(&version)
        .join("installation");

    version_path
        .to_str()
        .ok_or_else(|| "无效的路径".to_string())
        .map(|s| s.to_string())
}

/// 打开指定版本的安装目录
#[command]
pub fn open_version_directory(version: String) -> Result<(), String> {
    let dir = get_version_dir(version)?;
    open_directory(&dir)
}

/// 打开 fnm 根目录
#[command]
pub fn open_fnm_directory() -> Result<(), String> {
    let dir = get_fnm_base_dir()?;
    open_directory(&dir)
}

/// 获取 fnm 基础目录
fn get_fnm_base_dir() -> Result<String, String> {
    // 首先尝试从 fnm env 获取
    let output = Command::new("fnm")
        .arg("env")
        .output()
        .ok();

    if let Some(out) = output {
        if out.status.success() {
            let env_str = String::from_utf8_lossy(&out.stdout);
            for line in env_str.lines() {
                if line.contains("FNM_DIR=") {
                    let value = line
                        .split("FNM_DIR=")
                        .nth(1)
                        .unwrap_or("")
                        .trim()
                        .trim_matches('"')
                        .trim_matches('\'')
                        .trim_end_matches(';')
                        .to_string();
                    if !value.is_empty() {
                        return Ok(value);
                    }
                }
            }
        }
    }

    // 回退到默认路径
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").map_err(|_| "无法获取 HOME 目录")?;
        Ok(format!("{}/Library/Application Support/fnm", home))
    }

    #[cfg(target_os = "windows")]
    {
        let local_appdata =
            std::env::var("LOCALAPPDATA").map_err(|_| "无法获取 LOCALAPPDATA 目录")?;
        Ok(format!("{}\\fnm", local_appdata))
    }

    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").map_err(|_| "无法获取 HOME 目录")?;
        Ok(format!("{}/.local/share/fnm", home))
    }
}

/// 使用系统默认程序打开目录
fn open_directory(path: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(path)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }

    Ok(())
}
