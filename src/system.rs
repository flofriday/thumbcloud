use std::time::SystemTime;

// TODO: this can/should be a macro
/// Returns the name of the OS
pub fn get_os<'a>() -> &'a str {
    if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "macos") {
        "macOS"
    } else if cfg!(target_os = "ios") {
        "iOS"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else if cfg!(target_os = "android") {
        "Android"
    } else if cfg!(target_os = "freebsd") {
        "FreeBSD"
    } else if cfg!(target_os = "dragonfly") {
        "DragonFly BSD"
    } else if cfg!(target_os = "bitrig") {
        "Bitrig"
    } else if cfg!(target_os = "openbsd") {
        "OpenBSD"
    } else if cfg!(target_os = "netbsd") {
        "NetBSD"
    } else {
        "Unknown"
    }
}

/// Returns a JSON object with the uptime in seconds of Thumbcloud
/// ```
/// {
///     action: "sendUptime",
///     uptime: "60"
/// }
/// ```
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
