mod commands;

use commands::common::*;
use commands::env::*;
use commands::fs::*;
use commands::version::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // 版本管理 commands
            list_installed_versions,
            list_remote_versions,
            install_version,
            uninstall_version,
            use_version,
            get_current_version,
            set_default_version,
            // 环境变量 commands
            get_fnm_env,
            // 文件系统 commands
            get_fnm_dir,
            get_version_dir,
            open_version_directory,
            open_fnm_directory,
            // 调试 commands
            debug_fnm_lookup,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
