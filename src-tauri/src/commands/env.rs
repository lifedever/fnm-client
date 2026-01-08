use serde::{Deserialize, Serialize};
use std::process::Command;
use tauri::command;

/// fnm 环境配置信息
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FnmEnv {
    pub fnm_dir: String,
    pub node_dist_mirror: String,
    pub version_file_strategy: String,
    pub corepack_enabled: bool,
    pub resolve_engines: bool,
    pub arch: String,
    pub loglevel: String,
}

impl Default for FnmEnv {
    fn default() -> Self {
        FnmEnv {
            fnm_dir: String::new(),
            node_dist_mirror: "https://nodejs.org/dist".to_string(),
            version_file_strategy: "local".to_string(),
            corepack_enabled: false,
            resolve_engines: true,
            arch: String::new(),
            loglevel: "info".to_string(),
        }
    }
}

/// 获取 fnm 环境配置
#[command]
pub fn get_fnm_env() -> Result<FnmEnv, String> {
    let output = Command::new("fnm")
        .arg("env")
        .output()
        .map_err(|e| format!("执行 fnm env 失败: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let env_str = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;

    parse_fnm_env(&env_str)
}

/// 解析 fnm env 输出
fn parse_fnm_env(env_str: &str) -> Result<FnmEnv, String> {
    let mut env = FnmEnv::default();

    for line in env_str.lines() {
        // 处理 export 格式: export FNM_DIR="/path/to/fnm"
        // 或者简单格式: FNM_DIR="/path/to/fnm"
        let line = line.trim();

        if let Some(value) = extract_env_value(line, "FNM_DIR") {
            env.fnm_dir = value;
        } else if let Some(value) = extract_env_value(line, "FNM_NODE_DIST_MIRROR") {
            env.node_dist_mirror = value;
        } else if let Some(value) = extract_env_value(line, "FNM_VERSION_FILE_STRATEGY") {
            env.version_file_strategy = value;
        } else if let Some(value) = extract_env_value(line, "FNM_COREPACK_ENABLED") {
            env.corepack_enabled = value.to_lowercase() == "true" || value == "1";
        } else if let Some(value) = extract_env_value(line, "FNM_RESOLVE_ENGINES") {
            env.resolve_engines = value.to_lowercase() == "true" || value == "1";
        } else if let Some(value) = extract_env_value(line, "FNM_ARCH") {
            env.arch = value;
        } else if let Some(value) = extract_env_value(line, "FNM_LOGLEVEL") {
            env.loglevel = value;
        }
    }

    // 如果未从 fnm env 获取到目录，尝试获取默认路径
    if env.fnm_dir.is_empty() {
        env.fnm_dir = get_default_fnm_dir();
    }

    // 获取系统架构
    if env.arch.is_empty() {
        env.arch = get_system_arch();
    }

    Ok(env)
}

/// 从环境变量行中提取值
fn extract_env_value(line: &str, key: &str) -> Option<String> {
    // 匹配格式: export KEY="value" 或 KEY="value" 或 set KEY="value"
    let patterns = [
        format!("export {}=", key),
        format!("set {}=", key),
        format!("{}=", key),
    ];

    for pattern in patterns {
        if line.contains(&pattern) {
            let value = line
                .split(&pattern)
                .nth(1)?
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .trim_end_matches(';')
                .to_string();
            return Some(value);
        }
    }

    None
}

/// 获取默认的 fnm 目录
fn get_default_fnm_dir() -> String {
    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{}/Library/Application Support/fnm", home);
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
        if let Ok(home) = std::env::var("HOME") {
            return format!("{}/.local/share/fnm", home);
        }
    }

    String::new()
}

/// 获取系统架构
fn get_system_arch() -> String {
    #[cfg(target_arch = "x86_64")]
    {
        "x64".to_string()
    }

    #[cfg(target_arch = "aarch64")]
    {
        "arm64".to_string()
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        "unknown".to_string()
    }
}
