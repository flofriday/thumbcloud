use std::fs;
use std::path::PathBuf;
use serde_json;

#[derive(Serialize, Deserialize)]
struct FileRespond {
    action: String,
    folders: Vec<String>,
    files: Vec<String>,
}

impl FileRespond {
    fn new() -> FileRespond {
        FileRespond {
            action: "sendFilelist".to_string(), folders: Vec::new(), files: Vec::new() 
        }
    }
}

pub fn get_file_respond(path: PathBuf) -> String {
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => {
            return json!({
                "action": "sendError",
                "message": "Cannot read the given path"
            }).to_string();
        }
    };

    let mut respond = FileRespond::new();
    
    for entry in entries {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if file_type.is_dir() {
                        respond.folders.push(file_name);
                    } else {
                        respond.files.push(file_name);
                    }
                }
            }
        }
    }

    serde_json::to_string(&respond).unwrap_or(json!({
                "action": "sendError",
                "message": "Cannot parse content"
            }).to_string())
}
