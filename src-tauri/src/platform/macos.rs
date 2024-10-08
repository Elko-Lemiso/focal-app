// src/platform.rs

use std::error::Error;
use std::process::Command;

pub fn get_active_window_title() -> Result<String, Box<dyn Error>> {
    let script =
        r#"
    global frontAppName, windowTitle, appBundleId

    set windowTitle to ""
    tell application "System Events"
        set frontApp to first application process whose frontmost is true
        set frontAppName to name of frontApp
        set appBundleId to bundle identifier of frontApp
    end tell

    if frontAppName = "Google Chrome" then
        tell application "Google Chrome"
            set windowTitle to title of active tab of front window
        end tell
    else if frontAppName = "Safari" then
        tell application "Safari"
            set windowTitle to name of front document
        end tell
    else
        tell application "System Events"
            tell process frontAppName
                try
                    if exists (1st window whose value of attribute "AXMain" is true) then
                        set windowTitle to title of (1st window whose value of attribute "AXMain" is true)
                    end if
                on error
                    set windowTitle to ""
                end try
            end tell
        end tell
    end if

    return frontAppName & " - " & windowTitle & " (" & appBundleId & ")"
    "#;

    let output = Command::new("osascript").arg("-e").arg(script).output()?;

    if !output.status.success() {
        return Err(format!("osascript failed with status: {}", output.status).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

    Ok(stdout)
}
