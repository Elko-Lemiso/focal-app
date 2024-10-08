// src/app_usage.rs

use chrono::{ DateTime, Local };
use serde::{ Serialize };
use regex::Regex;

#[derive(Debug, Clone, Serialize)]
pub struct AppUsage {
    pub app: String,
    pub window: String,
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub duration_secs: u64,
}

impl AppUsage {
    pub fn new(
        previous_app: &str,
        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
        duration: u64
    ) -> Self {
        let (app_name, window_title, bundle_id) = parse_app_info(previous_app);
        let app = interpret_app_name(&app_name, &bundle_id);

        AppUsage {
            app,
            window: window_title,
            start_time,
            end_time,
            duration_secs: duration,
        }
    }
}

fn parse_app_info(app_info: &str) -> (String, String, String) {
    let re = Regex::new(r"^(.*?) - (.*?) \((.*?)\)$").unwrap();
    if let Some(caps) = re.captures(app_info) {
        let app_name = caps.get(1).map_or("", |m| m.as_str());
        let window_title = caps.get(2).map_or("", |m| m.as_str());
        let bundle_id = caps.get(3).map_or("", |m| m.as_str());

        (app_name.to_string(), window_title.to_string(), bundle_id.to_string())
    } else {
        // Handle cases where the regex does not match
        let parts: Vec<&str> = app_info.split(" - ").collect();
        let app_name = parts.get(0).unwrap_or(&"Unknown App").to_string();
        let window_title = parts.get(1).unwrap_or(&"").to_string();
        let bundle_id = "".to_string(); // Bundle ID not available

        (app_name, window_title, bundle_id)
    }
}

fn interpret_app_name(app_name: &str, bundle_id: &str) -> String {
    match bundle_id {
        "com.microsoft.VSCode" => "Visual Studio Code".to_string(),
        "com.something.slack" => "Slack".to_string(),
        _ => app_name.to_string(),
    }
}
