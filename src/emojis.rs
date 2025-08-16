// In a separate file, e.g., emojis.rs
use phf::phf_map; // Static, Compile-time map.

pub static EMOJIS: phf::Map<&'static str, &'static str> = phf_map! {
    "dynamite" => "🧨",
    "pen" => "🖊️",
    "eye" => "👁️",
    "joystick" => "🕹️",
    "computer" => "🖥️",
    "runner" => "🏃",
    "handwave" => "👋",
};

// Old EMOJI[n] static
// pub static EMOJIS: &[&str] = &["🧨", "🖊️", "👁️", "🕹️", "🖥️", "🏃"];
