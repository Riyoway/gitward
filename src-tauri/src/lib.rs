mod commands;
mod error;
mod models;
mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::git::git_is_repo,
            commands::git::git_read_config,
            commands::git::git_set_config,
            commands::git::git_status,
            commands::github_cli::gh_auth_status,
            commands::github_cli::gh_auth_switch,
            commands::github_cli::gh_auth_setup_git,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Gitward");
}
