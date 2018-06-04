use std::fs;
use std::path::PathBuf;
use serde_json;
use serde_json::*;

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

    let mut fl = FileRespond::new();

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let file_type = match entry.file_type() {
            Ok(e) => e,
            Err(_) => continue,
        };

        let file_name = match entry.file_name().into_string() {
            Ok(e) => e,
            Err(_) => continue,
        };

        if file_type.is_dir() {
            fl.folders.push(file_name);
        } else {
            fl.files.push(file_name);
        }
    }
    
    /*for entry in entries {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if file_type.is_dir() {
                        fl.folders.push(file_name);
                    } else {
                        fl.files.push(file_name);
                    }
                }
            }
        }
    }*/

    serde_json::to_string(&fl).unwrap()
}
