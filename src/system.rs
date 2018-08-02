use std::time::SystemTime;

pub fn get_os<'a>() -> &'a str {
    if cfg!(target_os = "windows") {
        return "Windows";
    } else if cfg!(target_os = "macos") {
        return "macOS";
    } else if cfg!(target_os = "ios") {
        return "iOS";
    } else if cfg!(target_os = "linux") {
        return "Linux";
    } else if cfg!(target_os = "android") {
        return "Android";
    } else if cfg!(target_os = "freebsd") {
        return "FreeBSD";
    } else if cfg!(target_os = "dragonfly") {
        return "DragonFly BSD";
    } else if cfg!(target_os = "bitrig") {
        return "Bitrig";
    } else if cfg!(target_os = "openbsd") {
        return "OpenBSD";
    } else if cfg!(target_os = "netbsd") {
        return "NetBSD";
    } else {
        return "Unknown";
    }
}

pub fn get_uptime_respond(start_time: &SystemTime) -> String {
    match start_time.elapsed() {
        Ok(e) => json!({
                "action": "sendUptime",
                "uptime": e.as_secs(),
            }).to_string(),
        Err(_) => json!({
                "action": "sendError",
                "message": "Unable to calculate uptime",
            }).to_string(),
    }
}
