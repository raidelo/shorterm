use std::process::Command;

use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, VK_F9, HOT_KEY_MODIFIERS};
use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG, WM_HOTKEY};

fn main() -> windows::core::Result<()> {
    unsafe {
        RegisterHotKey(None, 1, HOT_KEY_MODIFIERS(0), VK_F9.0 as u32)?;

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
