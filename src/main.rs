mod config;
mod parsing;
mod utils;

use std::process::Command;

use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG, WM_HOTKEY};

use crate::config::load_config;
use crate::utils::register_from_config;

fn main() -> windows::core::Result<()> {
    let config = load_config();
    register_from_config(&config)?;

    unsafe {
        let mut msg = MSG::default();
        loop {
            let ok = GetMessageW(&mut msg, None, 0, 0);
            if !ok.as_bool() {
                break;
            }
            if msg.message == WM_HOTKEY {
                let _ = Command::new(r"C:\Program Files\WezTerm\wezterm-gui.exe")
                    .args(["start", "--", "wsl.exe"])
                    .spawn();
            }
        }
    }
    Ok(())
}
