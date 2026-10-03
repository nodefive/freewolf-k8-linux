//! FREE WOLF K8 USB HID Protocol Definitions

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LightMode {
    pub id: u8,
    pub wire_id: u8,
    pub name: &'static str,
    pub attribute: u16,
    pub description: &'static str,
}

pub const ATTR_BRIGHTNESS: u16 = 0x01;
pub const ATTR_SPEED: u16 = 0x02;
pub const ATTR_MUSIC: u16 = 0x200;

impl LightMode {
    pub fn has_speed(&self) -> bool {
        (self.attribute & ATTR_SPEED) != 0
    }

    pub fn has_brightness(&self) -> bool {
        (self.attribute & ATTR_BRIGHTNESS) != 0
    }

    pub fn is_music(&self) -> bool {
        (self.attribute & ATTR_MUSIC) != 0
    }
}

pub const VID: u16 = 0x1A2C;
pub const PID_WIRED: u16 = 0x7C80;
#[allow(dead_code)]
pub const PID_WIRELESS: u16 = 0x7FFF;
pub const REPORT_ID: u8 = 0x07;

pub const LIGHT_MODES: &[LightMode] = &[
    LightMode { id: 0,  wire_id: 0x01, name: "Off",                attribute: 0,   description: "All keyboard backlights turned off" },
    LightMode { id: 1,  wire_id: 0x00, name: "Steady",             attribute: 1,   description: "Static backlight with adjustable brightness" },
    LightMode { id: 2,  wire_id: 0x02, name: "Breathing",          attribute: 3,   description: "Pulsing breathing light rhythm" },
    LightMode { id: 3,  wire_id: 0x03, name: "Windmill",           attribute: 3,   description: "Rotating pinwheel light pattern" },
    LightMode { id: 4,  wire_id: 0x04, name: "Neon Stream",        attribute: 3,   description: "Smooth flowing neon gradient across keyboard" },
    LightMode { id: 5,  wire_id: 0x05, name: "Streamer",           attribute: 3,   description: "Linear streaming rainbow wave" },
    LightMode { id: 6,  wire_id: 0x06, name: "Flowing Light",      attribute: 3,   description: "Multi-directional flowing light waves" },
    LightMode { id: 7,  wire_id: 0x07, name: "Dripping Ripples",   attribute: 3,   description: "Ripples expanding outward from pressed keys" },
    LightMode { id: 8,  wire_id: 0x08, name: "Brilliant Point",    attribute: 3,   description: "Single key illumination upon strike" },
    LightMode { id: 9,  wire_id: 0x09, name: "Flash Away",         attribute: 3,   description: "Keys flash instantly and fade out slowly" },
    LightMode { id: 10, wire_id: 0x0A, name: "Shadow Disappear",   attribute: 3,   description: "Trailing shadow fading behind keystrokes" },
    LightMode { id: 11, wire_id: 0x0B, name: "Ripples Shining",    attribute: 3,   description: "Interlocking concentric ripple rings" },
    LightMode { id: 12, wire_id: 0x0C, name: "Rich and Honored",   attribute: 3,   description: "Lush harmonic multi-color transitions" },
    LightMode { id: 13, wire_id: 0x0D, name: "Marquee Effect",     attribute: 3,   description: "Edge perimeter chasing marquee lights" },
    LightMode { id: 14, wire_id: 0x0E, name: "Rotating Storm",     attribute: 3,   description: "Centrifugal cyclone rotating vortex" },
    LightMode { id: 15, wire_id: 0x0F, name: "Serpentine Horse",   attribute: 3,   description: "Chasing horse-race serpentine sequence" },
    LightMode { id: 16, wire_id: 0x10, name: "Stars Twinkle",      attribute: 3,   description: "Random celestial twinkling starfield" },
    LightMode { id: 17, wire_id: 0x11, name: "Retro Snake",        attribute: 3,   description: "Classic retro arcade snake pattern" },
    LightMode { id: 18, wire_id: 0x12, name: "Diagonal Transform", attribute: 3,   description: "Diagonal sweeping wavefront" },
    LightMode { id: 19, wire_id: 0x13, name: "Sine Wave",          attribute: 3,   description: "Smooth sinusoidal undulating wave" },
    LightMode { id: 20, wire_id: 0x13, name: "Music",              attribute: 512, description: "Audio visualizer frequency equalizer reactive mode" },
];

pub fn build_lighting_packet(wire_id: u8, brightness: u8, speed: u8) -> [u8; 8] {
    if wire_id == 0x01 {
        // Off
        [REPORT_ID, 0xFF, 0xFF, wire_id, 0, 0, 0, 0]
    } else {
        let b = brightness.min(4);
        let s = speed.min(4);
        [REPORT_ID, 0xFF, 0xFF, wire_id, b, s, 0, 0]
    }
}

pub fn build_music_packet(submode: u8, eq_data: &[u8; 4]) -> [u8; 8] {
    [
        REPORT_ID,
        eq_data[0],
        eq_data[1],
        eq_data[2],
        eq_data[3],
        submode,
        0,
        0,
    ]
}

pub fn find_mode(mode_arg: &str) -> Option<&'static LightMode> {
    if let Ok(id) = mode_arg.parse::<u8>() {
        if let Some(m) = LIGHT_MODES.iter().find(|m| m.id == id) {
            return Some(m);
        }
    }

    let q = mode_arg.to_lowercase().replace(' ', "_");
    for m in LIGHT_MODES {
        let name_clean = m.name.to_lowercase().replace(' ', "_");
        if name_clean == q || name_clean.contains(&q) {
            return Some(m);
        }
    }

    // Aliases
    match q.as_str() {
        "neon_stream" => Some(&LIGHT_MODES[4]),
        "flowing_light_and" => Some(&LIGHT_MODES[6]),
        "shadow_disappea" => Some(&LIGHT_MODES[10]),
        "retro_snake" => Some(&LIGHT_MODES[17]),
        "diagonal_transfor" => Some(&LIGHT_MODES[18]),
        _ => None,
    }
}
