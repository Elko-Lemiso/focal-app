#[cfg(target_os = "windows")]
use winapi::um::winuser::{GetForegroundWindow, GetWindowTextW};
use std::error::Error;
use std::ptr::null_mut;

pub fn get_active_window_title() -> Result<String, Box<dyn Error>> {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_null() {
            return Err("No active window".into());
        }

        let mut buffer = [0u16; 512];
        let length = GetWindowTextW(hwnd, buffer.as_mut_ptr(), 512);

        if length > 0 {
            let title =
                String::from_utf16(&buffer[..length as usize])?;
            Ok(title)
        } else {
            Err("Could not get window title".into())
        }
    }
}
