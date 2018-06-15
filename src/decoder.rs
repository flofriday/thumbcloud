use std::path::PathBuf;
use files;
use serde_json;
use serde_json::Value;

pub fn decode(input: String) -> String {
    let data: Value = serde_json::from_str(input.as_str()).unwrap();

    if data["action"] == "requestFilelist" {
        let mut path = data["path"].to_string();

        // From the to_string methode the string starts and ends with a double-
        // qoute. Theses two lines are here to remove them.
        path.remove(0);
        path.pop();

        println!("Open path: {}", path); //TODO: remove later
        return files::get_file_respond(PathBuf::from(path.as_str()));
    } else {
        return json!({
                "action": "sendError",
                "message": format!("Unknown action from client: {:?}", data["action"]) 
        }).to_string();
    }
}
