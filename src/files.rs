use pretty_bytes::converter::convert;
use serde_json;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct FolderItem {
    name: String,
}

impl FolderItem {
    fn from_name(folder_name: String) -> FolderItem {
        FolderItem { name: folder_name }
    }
}

#[derive(Serialize, Deserialize)]
struct FileItem {
    name: String,
    size: String,
    icon: String,
}

impl FileItem {
    fn from(file_name: String, bytes: u64) -> FileItem {
        FileItem {
            icon: get_icon(&file_name),
            name: file_name,
            size: convert(bytes as f64).replace(" B", " bytes"),
        }
    }

    fn from_name(file_name: String) -> FileItem {
        FileItem {
            name: file_name,
            size: String::new(),
            icon: String::from("default"),
        }
    }
}

// This function detects a simple file-type from the file name. This step is
// needed so the frontend knows which icon it should use.
// Possible answers of this function are:
// audio, archive, code, default, document, image, presentation, pdf,
// spreedsheet, video
fn get_icon(file_name: &String) -> String {
    let extension_lists = [
        ("audio", ["test"]),
        ("archive", ["test"]),
        ("code", ["test"]),
        ("default", ["test"]),
        ("document", ["test"]),
        ("image", ["png"]),
        ("presentation", ["test"]),
        ("pdf", ["test"]),
        ("spreedsheet", ["test"]),
        ("video", ["test"]),
    ];

    // Start with the actual detection
    if let Some(mut index) = file_name.rfind('.') {
        index += 1; // To exclude the point
        let extension = &file_name[index..].to_lowercase();

        for list in extension_lists.iter() {
            for entry in list.1.iter() {
                if extension == entry {
                    return String::from(list.0);
                }
            }
        }
    }

    String::from("default")
}

#[derive(Serialize, Deserialize)]
struct FileRespond {
    action: String,
    path: String,
    folders: Vec<FolderItem>,
    files: Vec<FileItem>,
}

impl FileRespond {
    fn new() -> FileRespond {
        FileRespond {
            action: "sendFilelist".to_string(),
            path: String::new(),
            folders: Vec::new(),
            files: Vec::new(),
        }
    }
}

pub fn get_file_respond(path: PathBuf, path_name: String) -> String {
    let entries = match fs::read_dir(&path) {
        Ok(e) => e,
        Err(_) => {
            return json!({
                "action": "sendError",
                "message": format!("Cannot read the given path: {:?}", path_name)
            }).to_string();
        }
    };

    let mut respond = FileRespond::new();
    respond.path = path_name;

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
