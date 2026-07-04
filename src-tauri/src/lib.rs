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
            commands::credential::credential_diagnose,
            commands::fs::write_text_file,
            commands::fs::read_text_file,
            commands::git::git_is_repo,
            commands::git::git_read_config,
            commands::git::git_set_config,
            commands::git::git_status,
            commands::health::health_check,
            commands::identity::detect_framework,
            commands::identity::read_identity,
            commands::identity::write_identity,
            commands::github_cli::gh_auth_status,
            commands::github_cli::gh_auth_switch,
            commands::github_cli::gh_auth_setup_git,
            commands::launcher::detect_tools,
            commands::launcher::launch_tool,
            commands::sync::sync_repository,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Gitward");
}
