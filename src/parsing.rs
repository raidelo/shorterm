use windows::Win32::UI::Input::KeyboardAndMouse::{
    HOT_KEY_MODIFIERS, MOD_ALT, MOD_CONTROL, MOD_SHIFT, MOD_WIN,
    VK_F1, VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_F10, VK_F11, VK_F12,
};

pub fn parse_key(key: &str) -> Option<u32> {
    let vk = match key.to_uppercase().as_str() {
        "F1" => VK_F1,
        "F2" => VK_F2,
        "F3" => VK_F3,
        "F4" => VK_F4,
        "F5" => VK_F5,
        "F6" => VK_F6,
        "F7" => VK_F7,
        "F8" => VK_F8,
        "F9" => VK_F9,
        "F10" => VK_F10,
        "F11" => VK_F11,
        "F12" => VK_F12,
        _ => return None,
    };
    Some(vk.0 as u32)
}

pub fn parse_modifiers(mods: &[String]) -> HOT_KEY_MODIFIERS {
    let mut flags = HOT_KEY_MODIFIERS(0);
    for m in mods {
        flags |= match m.to_uppercase().as_str() {
            "CTRL" | "CONTROL" => MOD_CONTROL,
            "SHIFT" => MOD_SHIFT,
            "ALT" => MOD_ALT,
            "WIN" | "SUPER" => MOD_WIN,
            other => {
                eprintln!("modificador desconocido en config: {other}, se ignora");
                continue;
            }
        };
    }
    flags
}
