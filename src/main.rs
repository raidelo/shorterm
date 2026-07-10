mod config;
mod constants;
mod parsing;
mod utils;
mod watchdog;

use std::process::Command;

use windows::Win32::UI::Input::KeyboardAndMouse::UnregisterHotKey;
use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG, WM_HOTKEY};
use windows::Win32::System::Threading::GetCurrentThreadId;

use crate::config::load_config;
use crate::utils::register_from_config;
use crate::watchdog::{spawn_config_watcher, WM_CONFIG_CHANGED};
use crate::constants::HK_ID;

fn main() -> windows::core::Result<()> {
    let mut config = load_config();
    unsafe { register_from_config(&config)? };

    let main_thread_id = unsafe { GetCurrentThreadId() };
    spawn_config_watcher(main_thread_id);

    let mut msg = MSG::default();
    unsafe {
        loop {
            let ok = GetMessageW(&mut msg, None, 0, 0);
            if !ok.as_bool() {
                break;
            }

            match msg.message {
                WM_HOTKEY => {
                    let _ = Command::new(r"C:\Program Files\WezTerm\wezterm-gui.exe")
                        .args(["start"])
                        .spawn();
                }
                WM_CONFIG_CHANGED => {
                    println!("config.toml cambió, recargando...");
                    let _ = UnregisterHotKey(None, HK_ID);
                    config = load_config();
                    if let Err(e) = register_from_config(&config) {
                        eprintln!("no se pudo registrar el nuevo hotkey: {e:?}");
                    } else {
                        println!("nuevo hotkey activo: {:?}", config);
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
