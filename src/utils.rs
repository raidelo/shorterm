use windows::Win32::UI::Input::KeyboardAndMouse::RegisterHotKey;

use crate::{config::Config, parsing::{parse_key, parse_modifiers}};

pub fn register_from_config(config: &Config) -> windows::core::Result<()> {
    let vk = parse_key(&config.key)
        .unwrap_or_else(|| panic!("tecla inválida en config: {}", config.key));
    let modifiers = parse_modifiers(&config.modifier);

    unsafe { RegisterHotKey(None, 1, modifiers, vk) }
}
