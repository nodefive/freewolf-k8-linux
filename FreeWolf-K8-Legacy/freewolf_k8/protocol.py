"""
FREE WOLF K8 USB HID Protocol Definitions
"""
from dataclasses import dataclass
from typing import Optional, List, Dict

VID = 0x1A2C
PID_WIRED = 0x7C80
PID_WIRELESS = 0x7FFF

REPORT_ID = 0x07
HEADER_LIGHT = 0xFFFF  # bytes 1-2: 0xFF, 0xFF

ATTR_BRIGHTNESS = 0x01
ATTR_SPEED = 0x02
ATTR_MUSIC = 0x200  # 512

@dataclass
class LightMode:
    id: int
    wire_id: int
    name: str
    attribute: int
    description: str

    @property
    def has_speed(self) -> bool:
        return bool(self.attribute & ATTR_SPEED)

    @property
    def has_brightness(self) -> bool:
        return bool(self.attribute & ATTR_BRIGHTNESS)

    @property
    def is_music(self) -> bool:
        return bool(self.attribute & ATTR_MUSIC)

# All lighting modes (Off + 20 effects)
LIGHT_MODES: List[LightMode] = [
    LightMode(0,  0x01, "Off",                   0,   "All keyboard backlights turned off"),
    LightMode(1,  0x00, "Steady",                1,   "Static backlight with adjustable brightness"),
    LightMode(2,  0x02, "Breathing",             3,   "Pulsing breathing light rhythm"),
    LightMode(3,  0x03, "Windmill",              3,   "Rotating pinwheel light pattern"),
    LightMode(4,  0x04, "Neon Stream",           3,   "Smooth flowing neon gradient across keyboard"),
    LightMode(5,  0x05, "Streamer",              3,   "Linear streaming rainbow wave"),
    LightMode(6,  0x06, "Flowing Light",         3,   "Multi-directional flowing light waves"),
    LightMode(7,  0x07, "Dripping Ripples",      3,   "Ripples expanding outward from pressed keys"),
    LightMode(8,  0x08, "Brilliant Point",       3,   "Single key illumination upon strike"),
    LightMode(9,  0x09, "Flash Away",            3,   "Keys flash instantly and fade out slowly"),
    LightMode(10, 0x0A, "Shadow Disappear",      3,   "Trailing shadow fading behind keystrokes"),
    LightMode(11, 0x0B, "Ripples Shining",       3,   "Interlocking concentric ripple rings"),
    LightMode(12, 0x0C, "Rich and Honored",      3,   "Lush harmonic multi-color transitions"),
    LightMode(13, 0x0D, "Marquee Effect",        3,   "Edge perimeter chasing marquee lights"),
    LightMode(14, 0x0E, "Rotating Storm",        3,   "Centrifugal cyclone rotating vortex"),
    LightMode(15, 0x0F, "Serpentine Horse",      3,   "Chasing horse-race serpentine sequence"),
    LightMode(16, 0x10, "Stars Twinkle",         3,   "Random celestial twinkling starfield"),
    LightMode(17, 0x11, "Retro Snake",           3,   "Classic retro arcade snake pattern"),
    LightMode(18, 0x12, "Diagonal Transform",    3,   "Diagonal sweeping wavefront"),
    LightMode(19, 0x13, "Sine Wave",             3,   "Smooth sinusoidal undulating wave"),
    LightMode(20, 0x13, "Music",                 512, "Audio visualizer frequency equalizer reactive mode"),
]

MODE_BY_NAME: Dict[str, LightMode] = {}
for m in LIGHT_MODES:
    MODE_BY_NAME[m.name.lower()] = m
    MODE_BY_NAME[m.name.lower().replace(" ", "_")] = m

# Legacy aliases for CLI backward compatibility
MODE_BY_NAME["neon_stream"] = LIGHT_MODES[4]
MODE_BY_NAME["flowing light and"] = LIGHT_MODES[6]
MODE_BY_NAME["flowing_light_and"] = LIGHT_MODES[6]
MODE_BY_NAME["shadow_disappea"] = LIGHT_MODES[10]
MODE_BY_NAME["shadow disappea"] = LIGHT_MODES[10]
MODE_BY_NAME["retro_snake"] = LIGHT_MODES[17]
MODE_BY_NAME["diagonal transfor"] = LIGHT_MODES[18]
MODE_BY_NAME["diagonal_transfor"] = LIGHT_MODES[18]

MODE_BY_ID: Dict[int, LightMode] = {m.id: m for m in LIGHT_MODES}

def build_lighting_packet(wire_id: int, brightness: int, speed: int) -> bytes:
    """
    Builds the 8-byte USB Feature Report packet for standard lighting modes.
    brightness: 0 to 4 (mapped to wire 0x00 to 0x04, where 0 is LEDs off)
    speed: 0 to 4 (mapped to wire 0x00 to 0x04)
    For Off mode (wire_id 0x01), brightness and speed are zeroed.
    """
    if wire_id == 0x01:
        b_val = 0
        s_val = 0
    else:
        b_val = max(0, min(4, brightness))
        s_val = max(0, min(4, speed))
    return bytes([
        REPORT_ID,
        0xFF,
        0xFF,
        wire_id & 0xFF,
        b_val,
        s_val,
        0x00,
        0x00
    ])

def build_music_packet(submode: int = 2, eq_data: bytes = b'\x23\x45\x67\x89') -> bytes:
    """
    Builds the 8-byte streaming packet for Music Mode visualizer frames.
    submode: 1 (Music mode 1) or 2 (Music mode 2)
    eq_data: 4 bytes representing frequency band amplitudes
    """
    if len(eq_data) < 4:
        eq_data = (eq_data + b'\x00' * 4)[:4]
    return bytes([
        REPORT_ID,
        eq_data[0],
        eq_data[1],
        eq_data[2],
        eq_data[3],
        submode & 0xFF,
        0x00,
        0x00
    ])
