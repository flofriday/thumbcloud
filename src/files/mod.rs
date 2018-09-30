pub mod category;

use htmlescape;
use pretty_bytes::converter::convert;
use serde_json;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use config::Config;

// TODO: This function should work with paths that do not exist
/// This function is a secure version of the join method for PathBuf. The standart join method can
/// allow path tranversal, this function does not.
///
/// # Errors
///
/// * The paths are not secure joinable (path tranversal)
/// * The joined path does not exist
///
/// # Examples
///
/// ```
/// let path1 = Pathbuf::from("/home/");
/// secure_join(path1, "flo"); // Returns Ok("/home/flo")
///
/// let path2 = Pathbuf::from("/home/");
/// secure_join(path2, "../bin"); // Returns Err()
///
/// ```
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
    /// Create a new FolderItem object from the name
    fn from_name(folder_name: &str) -> FolderItem {
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
    /// Create a new FileItem object from a name and size
    fn from(file_name: &str, simple_icons: bool, bytes: u64) -> FileItem {
        FileItem {
            category: category::get_from_name(&file_name, simple_icons),
            name: htmlescape::encode_minimal(&file_name),
            size: convert(bytes as f64).replace(" B", " bytes"),
        }
    }

    /// Create a new FileItem object just fron the name. (Can be used when it is impossible to
    /// read the size of the file.)
    fn from_name(file_name: &str, simple_icons: bool) -> FileItem {
        FileItem {
            category: category::get_from_name(&file_name, simple_icons),
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
    /// Create an empty FileRespond with the action and path already set.
    fn from_path(path_name: &str) -> FileRespond {
        FileRespond {
            action: "sendFilelist".to_string(),
            path: htmlescape::encode_minimal(&path_name),
            folders: Vec::new(),
            files: Vec::new(),
        }
    }
}

/// Returns a JSON object with the all folders and all files inside the requested path.
pub fn get_file_respond(path_end: &str, config: &Config) -> String {
    let path = match secure_join(config.path.clone(), PathBuf::from(path_end)) {
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
                        respond.folders.push(FolderItem::from_name(&file_name));
                    } else {
                        let item: FileItem;

                        if let Ok(meta) = entry.metadata() {
                            item = FileItem::from(&file_name, config.simple_icons, meta.len())
                        } else {
                            item = FileItem::from_name(&file_name, config.simple_icons);
                        }

                        respond.files.push(item);
                    }
                }
            }
        }
    }

    serde_json::to_string(&respond).unwrap_or_else(|_| {
        json!({
            "action": "sendError",
            "message": "Cannot parse content"
        }).to_string()
    })
}

/// Returns a JSON object, which tells if the requested folder could have been created.
/// ```
/// // successfull
/// {
///     action: "sendNewFolder",
///     created: true
/// }
///
/// // failure
/// {
///     action: "sendNewFolder",
///     created: false,
///     message: "A message for the frontend"
/// }
/// ```
pub fn get_new_folder_respond(path_end: &str, config: &Config) -> String {
    let path_end = PathBuf::from(path_end);
    let path_end_parent = match path_end.parent() {
        Some(path) => path.to_path_buf(),
        None => PathBuf::from(""),
    };

    match secure_join(config.path.clone(), path_end_parent) {
        Ok(_) => (),
        Err(_) => {
            return json!({
                "action": "sendNewFolder",
                "created": false,
                "message": "Cannot create new folder, because the path is invalid"
            }).to_string()
        }
    };

    let path = config.path.clone().join(path_end.clone());

    match fs::create_dir(path) {
        Ok(_) => (),
        Err(e) => {
            return json!({
                "action": "sendNewFolder",
                "created": false,
                "message": format!("Cannot create new folder.<br<br>Exact Error: {}", e)
            }).to_string();
        }
    }

    println!("Creat Folder: {:?}", path_end);

    json!({
        "action": "sendNewFolder",
        "created": true,
        "message": ""
    }).to_string()
}
