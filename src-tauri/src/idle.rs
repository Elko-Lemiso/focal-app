use std::error::Error;

#[cfg(target_os = "macos")]
pub fn get_idle_time() -> Result<f64, Box<dyn Error>> {
    let output = std::process::Command::new("ioreg").args(&["-c", "IOHIDSystem"]).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("HIDIdleTime") {
            let parts: Vec<&str> = line.trim().split("=").collect();
            if let Some(idle_time_str) = parts.get(1) {
                let idle_nanos: f64 = idle_time_str.trim().parse()?;
                return Ok(idle_nanos / 1_000_000_000.0);
            }
        }
    }
    Err("Could not retrieve idle time".into())
}

#[cfg(target_os = "windows")]
pub fn get_idle_time() -> Result<f64, Box<dyn Error>> {
    use winapi::shared::minwindef::{ DWORD, UINT };
    use winapi::shared::ntdef::NULL;
    use winapi::um::winuser::GetLastInputInfo;

    unsafe {
        let mut lii = winapi::um::winuser::LASTINPUTINFO {
            cbSize: std::mem::size_of::<winapi::um::winuser::LASTINPUTINFO>() as UINT,
            dwTime: 0,
        };

        if GetLastInputInfo(&mut lii) == 0 {
            return Err("GetLastInputInfo failed".into());
        }

        let tick_count = winapi::um::sysinfoapi::GetTickCount();
        let idle_time = tick_count - lii.dwTime;

        Ok((idle_time as f64) / 1000.0)
    }
}

#[cfg(target_os = "linux")]
pub fn get_idle_time() -> Result<f64, Box<dyn Error>> {
    // This is a placeholder implementation.
    // You can use the `xdotool` command or `X11` libraries to get idle time.
    Err("Idle time retrieval not implemented for Linux yet.".into())
}
