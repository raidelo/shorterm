use std::fs;
use std::thread;
use std::time::{Duration, SystemTime};

use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{PostThreadMessageW, WM_APP};

use crate::config::config_path;

pub const WM_CONFIG_CHANGED: u32 = WM_APP + 1;

pub fn spawn_config_watcher(main_thread_id: u32) {
    thread::spawn(move || {
        let path = config_path();
        let mut last_modified: Option<SystemTime> =
            fs::metadata(&path).and_then(|m| m.modified()).ok();

        loop {
            thread::sleep(Duration::from_millis(500));

            let current_modified = match fs::metadata(&path).and_then(|m| m.modified()) {
                Ok(t) => t,
                Err(_) => continue, // el archivo puede no existir momentáneamente (ej. mientras se guarda)
            };

            if Some(current_modified) != last_modified {
                last_modified = Some(current_modified);
                unsafe {
                    let _ =
                        PostThreadMessageW(main_thread_id, WM_CONFIG_CHANGED, WPARAM(0), LPARAM(0));
                }
            }
        }
    });
}
