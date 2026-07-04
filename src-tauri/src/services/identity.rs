//! App Identity: detect a project's framework(s) and read/write the fields that
//! identify the app (name, identifier, version) across their config files.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

use crate::error::{AppError, AppResult};

/// Where a field lives and how to edit it.
enum Location {
    /// A dotted path inside a JSON file.
    Json {
        file: &'static str,
        path: &'static [&'static str],
    },
    /// A dotted path inside a TOML file.
    Toml {
        file: &'static str,
        path: &'static [&'static str],
    },
    /// A top-level `key: value` line in a YAML file.
    YamlLine {
        file: &'static str,
        key: &'static str,
    },
}

struct FieldDef {
    id: &'static str,
    label: &'static str,
    framework: &'static str,
    loc: Location,
}

const FIELDS: &[FieldDef] = &[
    FieldDef {
        id: "node.name",
        label: "Package name",
        framework: "node",
        loc: Location::Json {
            file: "package.json",
            path: &["name"],
        },
    },
    FieldDef {
        id: "node.version",
        label: "Version",
        framework: "node",
        loc: Location::Json {
            file: "package.json",
            path: &["version"],
        },
    },
    FieldDef {
        id: "tauri.productName",
        label: "Product name",
        framework: "tauri",
        loc: Location::Json {
            file: "src-tauri/tauri.conf.json",
            path: &["productName"],
        },
    },
    FieldDef {
        id: "tauri.identifier",
        label: "Identifier",
        framework: "tauri",
        loc: Location::Json {
            file: "src-tauri/tauri.conf.json",
            path: &["identifier"],
        },
    },
    FieldDef {
        id: "tauri.cargoName",
        label: "Cargo package name",
        framework: "tauri",
        loc: Location::Toml {
            file: "src-tauri/Cargo.toml",
            path: &["package", "name"],
        },
    },
    FieldDef {
        id: "electron.productName",
        label: "Product name (build)",
        framework: "electron",
        loc: Location::Json {
            file: "package.json",
            path: &["build", "productName"],
        },
    },
    FieldDef {
        id: "electron.appId",
        label: "App ID",
        framework: "electron",
        loc: Location::YamlLine {
            file: "electron-builder.yml",
            key: "appId",
        },
    },
    FieldDef {
        id: "flutter.name",
        label: "Name",
        framework: "flutter",
        loc: Location::YamlLine {
            file: "pubspec.yaml",
            key: "name",
        },
    },
    FieldDef {
        id: "flutter.version",
        label: "Version",
        framework: "flutter",
        loc: Location::YamlLine {
            file: "pubspec.yaml",
            key: "version",
        },
    },
];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityField {
    pub id: String,
    pub label: String,
    pub file: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IdentityFieldValue {
    pub id: String,
    pub value: String,
}

/// Detect which frameworks a project uses (a project may match several).
pub fn detect_frameworks(root: &str) -> Vec<String> {
    let root = Path::new(root);
    let mut frameworks = Vec::new();
    if root.join("src-tauri/tauri.conf.json").exists() {
        frameworks.push("tauri".to_string());
    }
    let package_json = root.join("package.json");
    if package_json.exists() {
        if package_has_electron(&package_json) {
            frameworks.push("electron".to_string());
        }
        frameworks.push("node".to_string());
    }
    if root.join("pubspec.yaml").exists() {
        frameworks.push("flutter".to_string());
    }
    frameworks
}

fn package_has_electron(package_json: &Path) -> bool {
    let Ok(text) = fs::read_to_string(package_json) else {
        return false;
    };
    let Ok(json) = serde_json::from_str::<Json>(&text) else {
        return false;
    };
    ["dependencies", "devDependencies"]
        .iter()
        .any(|section| json.get(section).and_then(|d| d.get("electron")).is_some())
}

/// Read every identity field belonging to the detected frameworks.
pub fn read_identity(root: &str) -> Vec<IdentityField> {
    let frameworks = detect_frameworks(root);
    FIELDS
        .iter()
        .filter(|f| frameworks.iter().any(|fw| fw == f.framework))
        .map(|f| IdentityField {
            id: f.id.to_string(),
            label: f.label.to_string(),
            file: location_file(&f.loc).to_string(),
            value: read_field(root, &f.loc),
        })
        .collect()
}

/// Write the given field values, backing up each touched file to `<file>.bak`.
pub fn write_identity(root: &str, values: &[IdentityFieldValue]) -> AppResult<()> {
    for v in values {
        let def = FIELDS
            .iter()
            .find(|f| f.id == v.id)
            .ok_or_else(|| AppError::UnknownField(v.id.clone()))?;
        write_field(root, &def.loc, &v.value)?;
    }
    Ok(())
}

fn location_file(loc: &Location) -> &'static str {
    match loc {
        Location::Json { file, .. }
        | Location::Toml { file, .. }
        | Location::YamlLine { file, .. } => file,
    }
}

fn read_field(root: &str, loc: &Location) -> Option<String> {
    match loc {
        Location::Json { file, path } => {
            let text = fs::read_to_string(Path::new(root).join(file)).ok()?;
            let json: Json = serde_json::from_str(&text).ok()?;
            json_get(&json, path)
        }
        Location::Toml { file, path } => {
            let text = fs::read_to_string(Path::new(root).join(file)).ok()?;
            let doc: toml_edit::DocumentMut = text.parse().ok()?;
            let mut item = doc.as_item();
            for key in *path {
                item = item.get(key)?;
            }
            item.as_str().map(String::from)
        }
        Location::YamlLine { file, key } => {
            let text = fs::read_to_string(Path::new(root).join(file)).ok()?;
            read_yaml_line(&text, key)
        }
    }
}

fn write_field(root: &str, loc: &Location, value: &str) -> AppResult<()> {
    match loc {
        Location::Json { file, path } => {
            let full = Path::new(root).join(file);
            let text = read_and_backup(&full)?;
            let mut json: Json = serde_json::from_str(&text).map_err(|e| AppError::FileParse {
                file: file.to_string(),
                reason: e.to_string(),
            })?;
            json_set(&mut json, path, value);
            let out = serde_json::to_string_pretty(&json).map_err(|e| AppError::FileParse {
                file: file.to_string(),
                reason: e.to_string(),
            })?;
            fs::write(&full, out + "\n")?;
        }
        Location::Toml { file, path } => {
            let full = Path::new(root).join(file);
            let text = read_and_backup(&full)?;
            let mut doc: toml_edit::DocumentMut =
                text.parse()
                    .map_err(|e: toml_edit::TomlError| AppError::FileParse {
                        file: file.to_string(),
                        reason: e.to_string(),
                    })?;
            let mut item = doc.as_item_mut();
            for key in *path {
                item = &mut item[key];
            }
            *item = toml_edit::value(value);
            fs::write(&full, doc.to_string())?;
        }
        Location::YamlLine { file, key } => {
            let full = Path::new(root).join(file);
            let text = read_and_backup(&full)?;
            fs::write(&full, replace_yaml_line(&text, key, value))?;
        }
    }
    Ok(())
}

/// Read a file and write a `.bak` copy beside it before it is modified.
fn read_and_backup(path: &Path) -> AppResult<String> {
    let text = fs::read_to_string(path)?;
    let bak = path.with_extension(format!(
        "{}.bak",
        path.extension().and_then(|e| e.to_str()).unwrap_or("")
    ));
    fs::write(bak, &text)?;
    Ok(text)
}

fn json_get(value: &Json, path: &[&str]) -> Option<String> {
    let mut cur = value;
    for key in path {
        cur = cur.get(key)?;
    }
    cur.as_str().map(String::from)
}

fn json_set(value: &mut Json, path: &[&str], new: &str) {
    let mut cur = value;
    for key in &path[..path.len() - 1] {
        if !cur.get(*key).map(Json::is_object).unwrap_or(false) {
            cur[*key] = Json::Object(Default::default());
        }
        cur = &mut cur[*key];
    }
    cur[path[path.len() - 1]] = Json::String(new.to_string());
}

fn read_yaml_line(content: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}:");
    content.lines().find_map(|line| {
        let trimmed = line.trim_start();
        trimmed
            .strip_prefix(&prefix)
            .map(|rest| rest.trim().trim_matches(['"', '\'']).to_string())
    })
}

/// Replace the value of a top-level `key:` line, preserving indentation. If the
/// key is absent, append it.
fn replace_yaml_line(content: &str, key: &str, value: &str) -> String {
    let prefix = format!("{key}:");
    let mut replaced = false;
    let mut out: Vec<String> = content
        .lines()
        .map(|line| {
            let indent = &line[..line.len() - line.trim_start().len()];
            if line.trim_start().starts_with(&prefix) {
                replaced = true;
                format!("{indent}{key}: {value}")
            } else {
                line.to_string()
            }
        })
        .collect();
    if !replaced {
        out.push(format!("{key}: {value}"));
    }
    let trailing_newline = content.ends_with('\n');
    let joined = out.join("\n");
    if trailing_newline {
        joined + "\n"
    } else {
        joined
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_get_and_set_nested() {
        let mut v: Json = serde_json::json!({ "name": "old", "build": { "productName": "App" } });
        assert_eq!(json_get(&v, &["name"]).as_deref(), Some("old"));
        assert_eq!(
            json_get(&v, &["build", "productName"]).as_deref(),
            Some("App")
        );

        json_set(&mut v, &["name"], "new");
        json_set(&mut v, &["build", "productName"], "Renamed");
        json_set(&mut v, &["author"], "Ada"); // create missing key
        assert_eq!(json_get(&v, &["name"]).as_deref(), Some("new"));
        assert_eq!(
            json_get(&v, &["build", "productName"]).as_deref(),
            Some("Renamed")
        );
        assert_eq!(json_get(&v, &["author"]).as_deref(), Some("Ada"));
    }

    #[test]
    fn yaml_line_read_and_replace() {
        let content = "name: old_app\nversion: 1.0.0\ndescription: hi\n";
        assert_eq!(read_yaml_line(content, "name").as_deref(), Some("old_app"));

        let out = replace_yaml_line(content, "name", "new_app");
        assert!(out.contains("name: new_app"));
        assert!(out.contains("version: 1.0.0"));
        assert!(out.ends_with('\n'));
    }

    #[test]
    fn yaml_replace_appends_missing_key() {
        let out = replace_yaml_line("name: app\n", "version", "2.0.0");
        assert!(out.contains("version: 2.0.0"));
    }
}
