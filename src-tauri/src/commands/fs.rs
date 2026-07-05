//! Minimal file read/write for import/export of user data.
//!
//! Paths come from the OS file dialog (user-chosen), so these back the
//! Export/Import buttons rather than exposing arbitrary filesystem access.

use std::fs;
use std::path::Path;

use base64::Engine;

use crate::error::{AppError, AppResult};
use crate::models::result::{run_blocking, CommandResult};

#[tauri::command]
pub async fn write_text_file(path: String, contents: String) -> CommandResult<()> {
    run_blocking(move || {
        CommandResult::from_result(fs::write(&path, contents).map_err(AppError::from))
    })
    .await
}

#[tauri::command]
pub async fn read_text_file(path: String) -> CommandResult<String> {
    run_blocking(move || {
        CommandResult::from_result(fs::read_to_string(&path).map_err(AppError::from))
    })
    .await
}

/// Read an image file and return it as a `data:` URL (for project icons).
#[tauri::command]
pub async fn read_image_data_url(path: String) -> CommandResult<String> {
    run_blocking(move || CommandResult::from_result(image_data_url(&path))).await
}

fn image_data_url(path: &str) -> AppResult<String> {
    // Icons are embedded in settings.json, so keep the source small.
    const MAX_BYTES: u64 = 1024 * 1024;
    let size = fs::metadata(path)?.len();
    if size > MAX_BYTES {
        return Err(AppError::FileParse {
            file: path.to_string(),
            reason: "image is larger than 1 MB".to_string(),
        });
    }
    let bytes = fs::read(path)?;
    let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:{};base64,{}", image_mime(path), encoded))
}

fn image_mime(path: &str) -> &'static str {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "bmp" => "image/bmp",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_mime_maps_extensions() {
        assert_eq!(image_mime("a/b/icon.png"), "image/png");
        assert_eq!(image_mime("ICON.JPG"), "image/jpeg");
        assert_eq!(image_mime("x.svg"), "image/svg+xml");
        assert_eq!(image_mime("noext"), "application/octet-stream");
    }

    #[test]
    fn image_data_url_encodes_small_file() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("icon.png");
        fs::write(&file, [1u8, 2, 3, 4]).unwrap();
        let url = image_data_url(&file.to_string_lossy()).unwrap();
        assert!(url.starts_with("data:image/png;base64,"));
    }
}
