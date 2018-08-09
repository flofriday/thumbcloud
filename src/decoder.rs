use serde_json;
use serde_json::Value;

use config::Config;
use files;
use system;

pub fn decode(input: String, config: &Config) -> String {
    let data: Value = serde_json::from_str(input.as_str()).unwrap();

    if data["action"] == "requestFilelist" {
        let mut path_end = data["path"].to_string();

        // From the to_string methode the string starts and ends with a double-
        // qoute. Theses two lines are here to remove them.
        path_end.remove(0);
        path_end.pop();

        return files::get_file_respond(path_end, config);
    } else if data["action"] == "requestNewFolder" {
        let mut path_end = data["path"].to_string();

        // From the to_string methode the string starts and ends with a double-
        // qoute. Theses two lines are here to remove them.
        path_end.remove(0);
        path_end.pop();

        return files::get_new_folder_respond(path_end, &config);
    } else if data["action"] == "requestUptime" {
        return system::get_uptime_respond(&config.start_time);
    } else {
        return json!({
            "action": "sendError",
            "message": format!("Unknown action from client: {}", data["action"]) 
        }).to_string();
    }
}
