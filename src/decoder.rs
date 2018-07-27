use serde_json;
use serde_json::Value;
use std::path::PathBuf;

use files;

pub fn decode(input: String, path_base: &PathBuf) -> String {
    let data: Value = serde_json::from_str(input.as_str()).unwrap();

    if data["action"] == "requestFilelist" {
        let mut path_end = data["path"].to_string();

        // From the to_string methode the string starts and ends with a double-
        // qoute. Theses two lines are here to remove them.
        path_end.remove(0);
        path_end.pop();

        let mut path: PathBuf = path_base.clone();
        path.push(&path_end);
        path = match path.canonicalize() {
            Ok(e) => e,
            Err(_) => {
                return json!({
                    "action": "sendError",
                    "message": format!("Cannot read the given path: {:?}", path_end)
                }).to_string();
            }
        };

        // Prevent path tranversal attacks
        if path.starts_with(path_base) == false {
            println!("SECURITY: prevented path tranversal attack");
            return json!({
                "action": "sendError",
                "message": format!("Cannot read the given path: {:?}", path_end)
            }).to_string();
        }

        return files::get_file_respond(path, path_end);
    } else {
        return json!({
                "action": "sendError",
                "message": format!("Unknown action from client: {:?}", data["action"]) 
        }).to_string();
    }
}
