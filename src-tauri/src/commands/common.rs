use std::path::PathBuf;
use std::process::Command;
use tauri::command;

/// 获取用户主目录（跨平台，支持打包后的 GUI 应用）
fn get_home_dir() -> Option<PathBuf> {
    // 优先使用 dirs crate，它能在打包后的应用中正确工作
    dirs::home_dir()
}

/// 调试命令：获取 fnm 查找信息
#[command]
pub fn debug_fnm_lookup() -> Result<String, String> {
    let mut info = String::new();

    // 显示 HOME 目录
    info.push_str(&format!("Home directory: {:?}\n", get_home_dir()));
    info.push_str(&format!("HOME env var: {:?}\n", std::env::var("HOME")));

    // 显示所有可能的路径及其存在性
    info.push_str("\nPossible fnm paths:\n");
    let paths = get_possible_fnm_paths();
    for path in &paths {
        let exists = path.exists();
        let is_file = path.is_file();
        info.push_str(&format!("  {:?} - exists: {}, is_file: {}\n", path, exists, is_file));
    }

    // 尝试获取 fnm 路径
    info.push_str(&format!("\nget_fnm_path result: {:?}\n", get_fnm_path()));

    // 如果找到了，尝试执行 fnm --version
    if let Ok(fnm_path) = get_fnm_path() {
        let version_result = Command::new(&fnm_path)
            .arg("--version")
            .output();
        match version_result {
            Ok(output) => {
                info.push_str(&format!("fnm --version: {}\n",
                    String::from_utf8_lossy(&output.stdout).trim()));
                if !output.status.success() {
                    info.push_str(&format!("stderr: {}\n",
                        String::from_utf8_lossy(&output.stderr)));
                }
            }
            Err(e) => {
                info.push_str(&format!("Failed to run fnm --version: {}\n", e));
            }
        }

        // 测试 fnm list 命令
        info.push_str("\n--- Testing fnm list ---\n");
        match create_fnm_command() {
            Ok(mut cmd) => {
                let list_result = cmd.arg("list").output();
                match list_result {
                    Ok(output) => {
                        info.push_str(&format!("exit status: {}\n", output.status));
                        info.push_str(&format!("stdout: {}\n", String::from_utf8_lossy(&output.stdout)));
                        info.push_str(&format!("stderr: {}\n", String::from_utf8_lossy(&output.stderr)));
                    }
                    Err(e) => {
                        info.push_str(&format!("Failed to run fnm list: {}\n", e));
                    }
                }
            }
            Err(e) => {
                info.push_str(&format!("Failed to create fnm command: {}\n", e));
            }
        }

        // 测试 fnm current 命令
        info.push_str("\n--- Testing fnm current ---\n");
        match create_fnm_command() {
            Ok(mut cmd) => {
                let current_result = cmd.arg("current").output();
                match current_result {
                    Ok(output) => {
                        info.push_str(&format!("exit status: {}\n", output.status));
                        info.push_str(&format!("stdout: {}\n", String::from_utf8_lossy(&output.stdout)));
                        info.push_str(&format!("stderr: {}\n", String::from_utf8_lossy(&output.stderr)));
                    }
                    Err(e) => {
                        info.push_str(&format!("Failed to run fnm current: {}\n", e));
                    }
                }
            }
            Err(e) => {
                info.push_str(&format!("Failed to create fnm command: {}\n", e));
            }
        }
    }

    Ok(info)
}

/// 获取 fnm 可执行文件的完整路径
/// 在打包后的 Tauri 应用中，需要使用完整路径来调用 fnm
pub fn get_fnm_path() -> Result<PathBuf, String> {
    // 常见的 fnm 安装路径
    let possible_paths = get_possible_fnm_paths();

    for path in &possible_paths {
        if path.exists() && path.is_file() {
            return Ok(path.clone());
        }
    }

    // 如果在预定义路径中找不到，尝试使用 which 命令
    #[cfg(unix)]
    {
        if let Ok(output) = Command::new("which").arg("fnm").output() {
            if output.status.success() {
                let path_str = String::from_utf8_lossy(&output.stdout);
                let path = PathBuf::from(path_str.trim());
                if path.exists() {
                    return Ok(path);
                }
            }
        }
    }

    #[cfg(windows)]
    {
        if let Ok(output) = Command::new("where").arg("fnm").output() {
            if output.status.success() {
                let path_str = String::from_utf8_lossy(&output.stdout);
                // where 可能返回多个路径，取第一个
                if let Some(first_line) = path_str.lines().next() {
                    let path = PathBuf::from(first_line.trim());
                    if path.exists() {
                        return Ok(path);
                    }
                }
            }
        }
    }

    Err("未找到 fnm，请确保已正确安装 fnm。\n\n安装方式：\nmacOS: brew install fnm\nWindows: winget install Schniz.fnm\nLinux: curl -fsSL https://fnm.vercel.app/install | bash".to_string())
}

/// 获取可能的 fnm 安装路径列表
fn get_possible_fnm_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let home = get_home_dir();

    #[cfg(target_os = "macos")]
    {
        // Homebrew (Apple Silicon)
        paths.push(PathBuf::from("/opt/homebrew/bin/fnm"));
        // Homebrew (Intel)
        paths.push(PathBuf::from("/usr/local/bin/fnm"));

        if let Some(ref home_dir) = home {
            // cargo install
            paths.push(home_dir.join(".cargo/bin/fnm"));
            // fnm 官方安装脚本
            paths.push(home_dir.join(".fnm/fnm"));
            paths.push(home_dir.join(".local/bin/fnm"));
        }
    }

    #[cfg(target_os = "linux")]
    {
        // 常见 Linux 路径
        paths.push(PathBuf::from("/usr/bin/fnm"));
        paths.push(PathBuf::from("/usr/local/bin/fnm"));

        if let Some(ref home_dir) = home {
            paths.push(home_dir.join(".cargo/bin/fnm"));
            paths.push(home_dir.join(".fnm/fnm"));
            paths.push(home_dir.join(".local/bin/fnm"));
        }
    }

    #[cfg(target_os = "windows")]
    {
        let local_appdata = std::env::var("LOCALAPPDATA").ok();
        let program_files = std::env::var("ProgramFiles").ok();

        if let Some(ref home_dir) = home {
            paths.push(home_dir.join(".cargo\\bin\\fnm.exe"));
            // scoop
            paths.push(home_dir.join("scoop\\shims\\fnm.exe"));
        }

        if let Some(ref appdata) = local_appdata {
            paths.push(PathBuf::from(format!("{}\\fnm\\fnm.exe", appdata)));
        }

        if let Some(ref pf) = program_files {
            paths.push(PathBuf::from(format!("{}\\fnm\\fnm.exe", pf)));
        }
    }

    paths
}

/// 创建一个配置好 PATH 的 fnm Command
pub fn create_fnm_command() -> Result<Command, String> {
    let fnm_path = get_fnm_path()?;
    let mut cmd = Command::new(&fnm_path);

    // 确保 PATH 环境变量包含常用路径
    let current_path = std::env::var("PATH").unwrap_or_default();
    let enhanced_path = get_enhanced_path(&current_path);
    cmd.env("PATH", enhanced_path);

    // 设置 FNM_DIR 环境变量，这是 fnm 正常工作所必需的
    let fnm_dir = get_fnm_dir_path();
    if !fnm_dir.is_empty() {
        cmd.env("FNM_DIR", &fnm_dir);
    }

    Ok(cmd)
}

/// 获取 fnm 数据目录路径
fn get_fnm_dir_path() -> String {
    let home = get_home_dir();

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

/// 扩展 PATH 环境变量，添加常用的可执行文件路径
fn get_enhanced_path(current_path: &str) -> String {
    let home = get_home_dir();
    let mut paths: Vec<String> = vec![current_path.to_string()];

    #[cfg(target_os = "macos")]
    {
        paths.push("/opt/homebrew/bin".to_string());
        paths.push("/usr/local/bin".to_string());
        if let Some(ref home_dir) = home {
            paths.push(home_dir.join(".cargo/bin").to_string_lossy().to_string());
            paths.push(home_dir.join(".fnm").to_string_lossy().to_string());
            paths.push(home_dir.join(".local/bin").to_string_lossy().to_string());
        }
    }

    #[cfg(target_os = "linux")]
    {
        paths.push("/usr/bin".to_string());
        paths.push("/usr/local/bin".to_string());
        if let Some(ref home_dir) = home {
            paths.push(home_dir.join(".cargo/bin").to_string_lossy().to_string());
            paths.push(home_dir.join(".fnm").to_string_lossy().to_string());
            paths.push(home_dir.join(".local/bin").to_string_lossy().to_string());
        }
    }

    #[cfg(unix)]
    {
        paths.join(":")
    }

    #[cfg(windows)]
    {
        paths.join(";")
    }
}
