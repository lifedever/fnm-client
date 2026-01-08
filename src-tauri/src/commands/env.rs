use super::common::create_fnm_command;
use serde::{Deserialize, Serialize};
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
    let mut env = FnmEnv::default();

    // 直接设置 fnm 目录
    env.fnm_dir = get_default_fnm_dir();

    // 设置系统架构
    env.arch = get_system_arch();

    // 尝试从环境变量获取其他配置
    if let Ok(mirror) = std::env::var("FNM_NODE_DIST_MIRROR") {
        env.node_dist_mirror = mirror;
    }

    if let Ok(strategy) = std::env::var("FNM_VERSION_FILE_STRATEGY") {
        env.version_file_strategy = strategy;
    }

    if let Ok(corepack) = std::env::var("FNM_COREPACK_ENABLED") {
        env.corepack_enabled = corepack.to_lowercase() == "true" || corepack == "1";
    }

    if let Ok(engines) = std::env::var("FNM_RESOLVE_ENGINES") {
        env.resolve_engines = engines.to_lowercase() != "false" && engines != "0";
    }

    if let Ok(loglevel) = std::env::var("FNM_LOGLEVEL") {
        env.loglevel = loglevel;
    }

    // 检测 corepack 是否实际启用（检查 default 版本的 bin 目录）
    env.corepack_enabled = check_corepack_enabled(&env.fnm_dir);

    Ok(env)
}

/// 检测 corepack 是否启用
fn check_corepack_enabled(fnm_dir: &str) -> bool {
    if fnm_dir.is_empty() {
        return false;
    }

    let default_alias = std::path::PathBuf::from(fnm_dir).join("aliases").join("default");
    if !default_alias.exists() {
        return false;
    }

    if let Ok(version_path) = std::fs::read_link(&default_alias) {
        // 检查是否有 yarn 或 pnpm 在 bin 目录（corepack enable 会创建这些）
        let yarn_path = version_path.join("bin").join("yarn");
        let pnpm_path = version_path.join("bin").join("pnpm");
        return yarn_path.exists() || pnpm_path.exists();
    }

    false
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

/// 切换 Corepack 状态
#[command]
pub fn toggle_corepack(enable: bool) -> Result<String, String> {
    // 获取 fnm 目录
    let fnm_dir = get_default_fnm_dir();
    if fnm_dir.is_empty() {
        return Err("无法确定 fnm 目录".to_string());
    }

    // 读取 default 别名指向的版本
    let default_alias = std::path::PathBuf::from(&fnm_dir).join("aliases").join("default");
    if !default_alias.exists() {
        return Err("请先设置一个默认 Node.js 版本".to_string());
    }

    let version_path = std::fs::read_link(&default_alias)
        .map_err(|_| "无法读取默认版本".to_string())?;

    // 构建 corepack 路径
    let corepack_path = version_path.join("bin").join("corepack");
    if !corepack_path.exists() {
        return Err("找不到 corepack，请确保 Node.js 版本 >= 16.9.0".to_string());
    }

    // 执行 corepack enable/disable
    let output = std::process::Command::new(&corepack_path)
        .arg(if enable { "enable" } else { "disable" })
        .output()
        .map_err(|e| format!("执行 corepack 命令失败: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(stderr.to_string());
    }

    Ok(if enable {
        "Corepack 已启用".to_string()
    } else {
        "Corepack 已禁用".to_string()
    })
}
