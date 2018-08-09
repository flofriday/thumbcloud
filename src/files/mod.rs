mod category;

use htmlescape;
use pretty_bytes::converter::convert;
use serde_json;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use config::Config;

// This function is a secure version of the join method for PathBuf. The standart join method can
// allow path tranversal, this function doesn't.
pub fn secure_join<P: AsRef<Path>>(first: PathBuf, second: P) -> Result<PathBuf, io::Error> {
    let mut result = first.clone();
    result = result.join(second);
    result = result.canonicalize()?;

    // Check if first is still a parent of result
    if result.starts_with(first) {
        Ok(result)
    } else {
        println!("SECURITY: prevented path traversal attack");
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Paths are not securely joinable",
        ))
    }
}

#[derive(Serialize, Deserialize)]
struct FolderItem {
    name: String,
}

impl FolderItem {
    fn from_name(folder_name: String) -> FolderItem {
        FolderItem {
            name: htmlescape::encode_minimal(&folder_name),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct FileItem {
    name: String,
    size: String,
    category: String,
}

impl FileItem {
    fn from(file_name: String, bytes: u64) -> FileItem {
        FileItem {
            category: category::get_from_name(&file_name),
            name: htmlescape::encode_minimal(&file_name),
            size: convert(bytes as f64).replace(" B", " bytes"),
        }
    }

    fn from_name(file_name: String) -> FileItem {
        FileItem {
            category: category::get_from_name(&file_name),
            name: htmlescape::encode_minimal(&file_name),
            size: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct FileRespond {
    action: String,
    path: String,
    folders: Vec<FolderItem>,
    files: Vec<FileItem>,
}

impl FileRespond {
    fn from_path(path_name: String) -> FileRespond {
        FileRespond {
            action: "sendFilelist".to_string(),
            path: htmlescape::encode_minimal(&path_name),
            folders: Vec::new(),
            files: Vec::new(),
        }
    }
}

pub fn get_file_respond(path_end: String, config: &Config) -> String {

    let path = match secure_join(config.path.clone(), path_end.clone()) {
        Ok(path) => path,
        Err(_) => {
            return json!({
                "action": "sendError",
                "message": format!("Cannot read the given path: {:?}", path_end)
            }).to_string();
        }
    };

    let entries = match fs::read_dir(&path) {
        Ok(e) => e,
        Err(_) => {
            return json!({
                "action": "sendError",
                "message": format!("Cannot read the given path: {:?}", path_end)
            }).to_string();
        }
    };

    println!("Open path: {:?}", path_end);
    let mut respond = FileRespond::from_path(path_end);

    for entry in entries {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if file_type.is_dir() {
                        respond.folders.push(FolderItem::from_name(file_name));
                    } else {
                        let item: FileItem;

                        if let Ok(meta) = entry.metadata() {
                            item = FileItem::from(file_name, meta.len())
                        } else {
                            item = FileItem::from_name(file_name);
                        }

                        respond.files.push(item);
                    }
                }
            }
        }
    }

    serde_json::to_string(&respond).unwrap_or(
        json!({
                "action": "sendError",
                "message": "Cannot parse content"
            }).to_string(),
    )
}
