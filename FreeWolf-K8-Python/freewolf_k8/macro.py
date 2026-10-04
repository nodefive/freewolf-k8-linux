"""
Macro Engine and Data Management for FREE WOLF K8
"""
import os
import json
import time
from dataclasses import dataclass, field, asdict
from typing import List, Optional, Dict

DELAY_RECORD = 0   # Record Delay (actual elapsed ms)
DELAY_NONE = 1     # No Delay (0 ms)
DELAY_DEFAULT = 2  # Default Delay (fixed ms, e.g. 10ms)

CONFIG_DIR = os.path.expanduser("~/.config/freewolf-k8")
MACRO_FILE = os.path.join(CONFIG_DIR, "macros.json")

# Standard HID key mapping for common keys
KEYSYM_TO_HID = {
    "a": (0x04, "Key A"), "b": (0x05, "Key B"), "c": (0x06, "Key C"),
    "d": (0x07, "Key D"), "e": (0x08, "Key E"), "f": (0x09, "Key F"),
    "g": (0x0A, "Key G"), "h": (0x0B, "Key H"), "i": (0x0C, "Key I"),
    "j": (0x0D, "Key J"), "k": (0x0E, "Key K"), "l": (0x0F, "Key L"),
    "m": (0x10, "Key M"), "n": (0x11, "Key N"), "o": (0x12, "Key O"),
    "p": (0x13, "Key P"), "q": (0x14, "Key Q"), "r": (0x15, "Key R"),
    "s": (0x16, "Key S"), "t": (0x17, "Key T"), "u": (0x18, "Key U"),
    "v": (0x19, "Key V"), "w": (0x1A, "Key W"), "x": (0x1B, "Key X"),
    "y": (0x1C, "Key Y"), "z": (0x1D, "Key Z"),
    "1": (0x1E, "Key 1"), "2": (0x1F, "Key 2"), "3": (0x20, "Key 3"),
    "4": (0x21, "Key 4"), "5": (0x22, "Key 5"), "6": (0x23, "Key 6"),
    "7": (0x24, "Key 7"), "8": (0x25, "Key 8"), "9": (0x26, "Key 9"),
    "0": (0x27, "Key 0"),
    "Return": (0x28, "Enter"), "Escape": (0x29, "Esc"),
    "BackSpace": (0x2A, "Backspace"), "Tab": (0x2B, "Tab"),
    "space": (0x2C, "Space"), "minus": (0x2D, "-"), "equal": (0x2E, "="),
    "bracketleft": (0x2F, "["), "bracketright": (0x30, "]"),
    "backslash": (0x31, "\\"), "semicolon": (0x33, ";"),
    "apostrophe": (0x34, "'"), "grave": (0x35, "`"),
    "comma": (0x36, ","), "period": (0x37, "."), "slash": (0x38, "/"),
    "Caps_Lock": (0x39, "Caps Lock"),
    "F1": (0x3A, "F1"), "F2": (0x3B, "F2"), "F3": (0x3C, "F3"),
    "F4": (0x3D, "F4"), "F5": (0x3E, "F5"), "F6": (0x3F, "F6"),
    "F7": (0x40, "F7"), "F8": (0x41, "F8"), "F9": (0x42, "F9"),
    "F10": (0x43, "F10"), "F11": (0x44, "F11"), "F12": (0x45, "F12"),
    "Print": (0x46, "Print Screen"), "Scroll_Lock": (0x47, "Scroll Lock"),
    "Pause": (0x48, "Pause"), "Insert": (0x49, "Insert"),
    "Home": (0x4A, "Home"), "Prior": (0x4B, "Page Up"),
    "Delete": (0x4C, "Delete"), "End": (0x4D, "End"), "Next": (0x4E, "Page Down"),
    "Right": (0x4F, "Right"), "Left": (0x50, "Left"),
    "Down": (0x51, "Down"), "Up": (0x52, "Up"),
    "Control_L": (0xE0, "Ctrl"), "Shift_L": (0xE1, "Shift"),
    "Alt_L": (0xE2, "Alt"), "Super_L": (0xE3, "Win"),
    "Control_R": (0xE4, "Right Ctrl"), "Shift_R": (0xE5, "Right Shift"),
    "Alt_R": (0xE6, "Right Alt"), "Super_R": (0xE7, "Right Win"),
    "Menu": (0x65, "App"),
}

def get_key_info(keysym: str, keycode: int = 0):
    """
    Translates a Tkinter keysym and keycode into (hid_code, display_name).
    E.g. ('q', 24) -> (0x14, 'Key Q')
    """
    if keysym in KEYSYM_TO_HID:
        code, name = KEYSYM_TO_HID[keysym]
        desc = name if name.startswith("Key ") else f"Key {name}"
        return code, desc

    low = keysym.lower()
    if low in KEYSYM_TO_HID:
        code, name = KEYSYM_TO_HID[low]
        desc = name if name.startswith("Key ") else f"Key {name}"
        return code, desc

    shifted_map = {
        "exclam": (0x1E, "Key 1"), "at": (0x1F, "Key 2"), "numbersign": (0x20, "Key 3"),
        "dollar": (0x21, "Key 4"), "percent": (0x22, "Key 5"), "asciicircum": (0x23, "Key 6"),
        "ampersand": (0x24, "Key 7"), "asterisk": (0x25, "Key 8"), "parenleft": (0x26, "Key 9"),
        "parenright": (0x27, "Key 0"), "underscore": (0x2D, "Key -"), "plus": (0x2E, "Key ="),
        "braceleft": (0x2F, "Key ["), "braceright": (0x30, "Key ]"), "bar": (0x31, "Key \\"),
        "colon": (0x33, "Key ;"), "quotedbl": (0x34, "Key '"), "asciitilde": (0x35, "Key `"),
        "less": (0x36, "Key ,"), "greater": (0x37, "Key ."), "question": (0x38, "Key /"),
    }
    if keysym in shifted_map:
        return shifted_map[keysym]

    if keysym.startswith("KP_"):
        kp_suffix = keysym[3:]
        kp_map = {
            "0": (0x62, "Key Num 0"), "1": (0x59, "Key Num 1"), "2": (0x5A, "Key Num 2"),
            "3": (0x5B, "Key Num 3"), "4": (0x5C, "Key Num 4"), "5": (0x5D, "Key Num 5"),
            "6": (0x5E, "Key Num 6"), "7": (0x5F, "Key Num 7"), "8": (0x60, "Key Num 8"),
            "9": (0x61, "Key Num 9"), "Enter": (0x58, "Key Num Enter"), "Add": (0x57, "Key Num +"),
            "Subtract": (0x56, "Key Num -"), "Multiply": (0x55, "Key Num *"),
            "Divide": (0x54, "Key Num /"), "Decimal": (0x63, "Key Num .")
        }
        if kp_suffix in kp_map:
            return kp_map[kp_suffix]

    clean_name = keysym.replace("_L", "").replace("_R", "")
    return keycode, f"Key {clean_name}"

@dataclass
class MacroAction:
    desc: str
    action: str  # "Down" or "Up"
    delay_ms: int
    keycode: int = 0

@dataclass
class Macro:
    id: int
    name: str
    repeat_time: int = 1
    delay_type: int = DELAY_DEFAULT
    default_delay: int = 10
    actions: List[MacroAction] = field(default_factory=list)

class MacroManager:
    def __init__(self):
        self.macros: List[Macro] = []
        self._load()

    def _ensure_dir(self):
        os.makedirs(CONFIG_DIR, exist_ok=True)

    def _load(self):
        if os.path.exists(MACRO_FILE):
            try:
                with open(MACRO_FILE, "r") as f:
                    data = json.load(f)
                self.macros = []
                for m_dict in data:
                    actions = [MacroAction(**a) for a in m_dict.get("actions", [])]
                    macro = Macro(
                        id=m_dict["id"],
                        name=m_dict["name"],
                        repeat_time=m_dict.get("repeat_time", 1),
                        delay_type=m_dict.get("delay_type", DELAY_DEFAULT),
                        default_delay=m_dict.get("default_delay", 10),
                        actions=actions
                    )
                    self.macros.append(macro)
                return
            except Exception as e:
                print(f"Warning: Failed to load {MACRO_FILE}: {e}")

        # Default sample macros if none exist
        self.macros = [
            Macro(
                id=1,
                name="Macro 1",
                repeat_time=1,
                delay_type=DELAY_DEFAULT,
                default_delay=10,
                actions=[
                    MacroAction(desc="Key Q", action="Down", delay_ms=10, keycode=0x14),
                    MacroAction(desc="Key Q", action="Up", delay_ms=10, keycode=0x14),
                    MacroAction(desc="Key W", action="Down", delay_ms=10, keycode=0x1A),
                    MacroAction(desc="Key W", action="Up", delay_ms=10, keycode=0x1A),
                    MacroAction(desc="Key E", action="Down", delay_ms=10, keycode=0x08),
                    MacroAction(desc="Key E", action="Up", delay_ms=10, keycode=0x08),
                ]
            )
        ]
        self.save()

    def save(self):
        self._ensure_dir()
        data = []
        for m in self.macros:
            d = asdict(m)
            data.append(d)
        with open(MACRO_FILE, "w") as f:
            json.dump(data, f, indent=2)

    def new_macro(self, name: Optional[str] = None) -> Macro:
        next_id = max([m.id for m in self.macros], default=0) + 1
        if not name:
            name = f"Macro {next_id}"
        macro = Macro(id=next_id, name=name, repeat_time=1, delay_type=DELAY_DEFAULT, default_delay=10)
        self.macros.append(macro)
        self.save()
        return macro

    def delete_macro(self, macro_id: int):
        self.macros = [m for m in self.macros if m.id != macro_id]
        self.save()

    def clear_all(self):
        """Deletes all macros and saves an empty macro list to disk."""
        self.macros.clear()
        self.save()

    def copy_macro(self, macro_id: int) -> Optional[Macro]:
        source = self.get_macro(macro_id)
        if not source:
            return None
        next_id = max([m.id for m in self.macros], default=0) + 1
        new_actions = [MacroAction(**asdict(a)) for a in source.actions]
        copied = Macro(
            id=next_id,
            name=f"{source.name} (Copy)",
            repeat_time=source.repeat_time,
            delay_type=source.delay_type,
            default_delay=source.default_delay,
            actions=new_actions
        )
        self.macros.append(copied)
        self.save()
        return copied

    def rename_macro(self, macro_id: int, new_name: str):
        macro = self.get_macro(macro_id)
        if macro:
            macro.name = new_name.strip()
            self.save()

    def get_macro(self, macro_id: int) -> Optional[Macro]:
        for m in self.macros:
            if m.id == macro_id:
                return m
        return None

    def export_macro(self, macro_id: int, filepath: str):
        macro = self.get_macro(macro_id)
        if macro:
            with open(filepath, "w") as f:
                json.dump(asdict(macro), f, indent=2)

    def import_macro(self, filepath: str) -> Optional[Macro]:
        with open(filepath, "r") as f:
            data = json.load(f)
        next_id = max([m.id for m in self.macros], default=0) + 1
        actions = [MacroAction(**a) for a in data.get("actions", [])]
        macro = Macro(
            id=next_id,
            name=data.get("name", f"Imported Macro {next_id}"),
            repeat_time=data.get("repeat_time", 1),
            delay_type=data.get("delay_type", DELAY_DEFAULT),
            default_delay=data.get("default_delay", 10),
            actions=actions
        )
        self.macros.append(macro)
        self.save()
        return macro

    def reorder_action(self, macro_id: int, old_index: int, new_index: int) -> bool:
        macro = self.get_macro(macro_id)
        if not macro:
            return False
        if 0 <= old_index < len(macro.actions) and 0 <= new_index < len(macro.actions):
            action = macro.actions.pop(old_index)
            macro.actions.insert(new_index, action)
            self.save()
            return True
        return False

    def delete_action(self, macro_id: int, index: int) -> bool:
        macro = self.get_macro(macro_id)
        if not macro:
            return False
        if 0 <= index < len(macro.actions):
            macro.actions.pop(index)
            self.save()
            return True
        return False

    def add_action(self, macro_id: int, action: MacroAction):
        macro = self.get_macro(macro_id)
        if macro:
            macro.actions.append(action)

DESC_TO_EVKEY_NAME = {
    "Key A": "KEY_A", "Key B": "KEY_B", "Key C": "KEY_C", "Key D": "KEY_D",
    "Key E": "KEY_E", "Key F": "KEY_F", "Key G": "KEY_G", "Key H": "KEY_H",
    "Key I": "KEY_I", "Key J": "KEY_J", "Key K": "KEY_K", "Key L": "KEY_L",
    "Key M": "KEY_M", "Key N": "KEY_N", "Key O": "KEY_O", "Key P": "KEY_P",
    "Key Q": "KEY_Q", "Key R": "KEY_R", "Key S": "KEY_S", "Key T": "KEY_T",
    "Key U": "KEY_U", "Key V": "KEY_V", "Key W": "KEY_W", "Key X": "KEY_X",
    "Key Y": "KEY_Y", "Key Z": "KEY_Z",
    "Key 1": "KEY_1", "Key 2": "KEY_2", "Key 3": "KEY_3", "Key 4": "KEY_4",
    "Key 5": "KEY_5", "Key 6": "KEY_6", "Key 7": "KEY_7", "Key 8": "KEY_8",
    "Key 9": "KEY_9", "Key 0": "KEY_0",
    "Key Enter": "KEY_ENTER", "Key Esc": "KEY_ESC", "Key Backspace": "KEY_BACKSPACE",
    "Key Tab": "KEY_TAB", "Key Space": "KEY_SPACE", "Key -": "KEY_MINUS",
    "Key =": "KEY_EQUAL", "Key [": "KEY_LEFTBRACE", "Key ]": "KEY_RIGHTBRACE",
    "Key \\": "KEY_BACKSLASH", "Key ;": "KEY_SEMICOLON", "Key '": "KEY_APOSTROPHE",
    "Key `": "KEY_GRAVE", "Key ,": "KEY_COMMA", "Key .": "KEY_DOT", "Key /": "KEY_SLASH",
    "Key Caps Lock": "KEY_CAPSLOCK",
    "Key F1": "KEY_F1", "Key F2": "KEY_F2", "Key F3": "KEY_F3", "Key F4": "KEY_F4",
    "Key F5": "KEY_F5", "Key F6": "KEY_F6", "Key F7": "KEY_F7", "Key F8": "KEY_F8",
    "Key F9": "KEY_F9", "Key F10": "KEY_F10", "Key F11": "KEY_F11", "Key F12": "KEY_F12",
    "Key Up": "KEY_UP", "Key Down": "KEY_DOWN", "Key Left": "KEY_LEFT", "Key Right": "KEY_RIGHT",
    "Key Ctrl": "KEY_LEFTCTRL", "Key Shift": "KEY_LEFTSHIFT", "Key Alt": "KEY_LEFTALT", "Key Win": "KEY_LEFTMETA",
    "Key Right Ctrl": "KEY_RIGHTCTRL", "Key Right Shift": "KEY_RIGHTSHIFT", "Key Right Alt": "KEY_RIGHTALT", "Key Right Win": "KEY_RIGHTMETA",
    "Key Delete": "KEY_DELETE", "Key Home": "KEY_HOME", "Key End": "KEY_END",
    "Key Page Up": "KEY_PAGEUP", "Key Page Down": "KEY_PAGEDOWN", "Key Insert": "KEY_INSERT",
    "Key Num 0": "KEY_KP0", "Key Num 1": "KEY_KP1", "Key Num 2": "KEY_KP2",
    "Key Num 3": "KEY_KP3", "Key Num 4": "KEY_KP4", "Key Num 5": "KEY_KP5",
    "Key Num 6": "KEY_KP6", "Key Num 7": "KEY_KP7", "Key Num 8": "KEY_KP8",
    "Key Num 9": "KEY_KP9", "Key Num Enter": "KEY_KPENTER", "Key Num +": "KEY_KPPLUS",
    "Key Num -": "KEY_KPMINUS", "Key Num *": "KEY_KPASTERISK", "Key Num /": "KEY_KPSLASH",
    "Key Num .": "KEY_KPDOT",
}

KEYSYM_MAP = {
    "Key Enter": 0xFF0D, "Key Esc": 0xFF1B, "Key Backspace": 0xFF08,
    "Key Tab": 0xFF09, "Key Space": 0x0020, "Key Delete": 0xFFFF,
    "Key Home": 0xFF50, "Key End": 0xFF57, "Key Page Up": 0xFF55,
    "Key Page Down": 0xFF56, "Key Up": 0xFF52, "Key Down": 0xFF54,
    "Key Left": 0xFF51, "Key Right": 0xFF53, "Key Insert": 0xFF63,
    "Key Caps Lock": 0xFFE5, "Key Shift": 0xFFE1, "Key Ctrl": 0xFFE3,
    "Key Alt": 0xFFE9, "Key Win": 0xFFEB, "Key Right Shift": 0xFFE2,
    "Key Right Ctrl": 0xFFE4, "Key Right Alt": 0xFFEA, "Key Right Win": 0xFFEC,
    "Key F1": 0xFFBE, "Key F2": 0xFFBF, "Key F3": 0xFFC0, "Key F4": 0xFFC1,
    "Key F5": 0xFFC2, "Key F6": 0xFFC3, "Key F7": 0xFFC4, "Key F8": 0xFFC5,
    "Key F9": 0xFFC6, "Key F10": 0xFFC7, "Key F11": 0xFFC8, "Key F12": 0xFFC9,
    "Key Num 0": 0xFFB0, "Key Num 1": 0xFFB1, "Key Num 2": 0xFFB2,
    "Key Num 3": 0xFFB3, "Key Num 4": 0xFFB4, "Key Num 5": 0xFFB5,
    "Key Num 6": 0xFFB6, "Key Num 7": 0xFFB7, "Key Num 8": 0xFFB8,
    "Key Num 9": 0xFFB9, "Key Num Enter": 0xFF8D, "Key Num +": 0xFFAB,
    "Key Num -": 0xFFAD, "Key Num *": 0xFFAA, "Key Num /": 0xFFAF, "Key Num .": 0xFFAE,
}

def desc_to_keysym(desc: str) -> int:
    if desc in KEYSYM_MAP:
        return KEYSYM_MAP[desc]
    raw = desc.replace("Key ", "").strip()
    if len(raw) == 1:
        return ord(raw.lower())
    return ord(raw[0].lower())

class MacroPlayer:
    """
    Simulates macro keystrokes into the active Linux window/desktop session.
    Automatically chooses the best available backend:
      1. GNOME Wayland Mutter RemoteDesktop (Zero-config, no root/module required)
      2. Linux /dev/uinput virtual keyboard (Universal)
    """
    def __init__(self):
        self.backend = None
        self.uinput = None
        self._init_error = None
        self._detect_backend()

    def _detect_backend(self):
        # 1. Try GNOME Mutter RemoteDesktop
        try:
            import dbus
            bus = dbus.SessionBus()
            obj = bus.get_object('org.gnome.Mutter.RemoteDesktop', '/org/gnome/Mutter/RemoteDesktop')
            iface = dbus.Interface(obj, 'org.gnome.Mutter.RemoteDesktop')
            path = iface.CreateSession()
            sess_obj = bus.get_object('org.gnome.Mutter.RemoteDesktop', path)
            sess_iface = dbus.Interface(sess_obj, 'org.gnome.Mutter.RemoteDesktop.Session')
            sess_iface.Start()
            sess_iface.Stop()
            self.backend = "mutter"
            self._init_error = None
            return
        except Exception as e:
            mutter_err = str(e)

        # 2. Try Linux uinput
        try:
            import libevdev
            dev = libevdev.Device()
            dev.name = "FREE WOLF K8 Virtual Macro Keyboard"
            for k in dir(libevdev.EV_KEY):
                if k.startswith("KEY_"):
                    try:
                        dev.enable(getattr(libevdev.EV_KEY, k))
                    except Exception:
                        pass
            self.uinput = dev.create_uinput_device()
            self.backend = "uinput"
            self._init_error = None
            return
        except Exception as e:
            uinput_err = str(e)

        self.backend = None
        self._init_error = f"uinput: {uinput_err}; mutter: {mutter_err}"

    def is_available(self) -> bool:
        if not self.backend:
            self._detect_backend()
        return self.backend is not None

    def play(
        self,
        macro: Macro,
        repeat_count: Optional[int] = None,
        delay_start: float = 0.0,
        on_step=None
    ) -> Tuple[bool, str]:
        if not self.is_available():
            return False, f"Keystroke simulation backend unavailable: {self._init_error}"

        if delay_start > 0:
            time.sleep(delay_start)

        repeats = repeat_count if repeat_count is not None else max(1, macro.repeat_time)

        if self.backend == "mutter":
            return self._play_mutter(macro, repeats, on_step)
        elif self.backend == "uinput":
            return self._play_uinput(macro, repeats, on_step)

        return False, "No playback backend selected."

    def _play_mutter(self, macro: Macro, repeats: int, on_step=None) -> Tuple[bool, str]:
        import dbus
        bus = dbus.SessionBus()
        obj = bus.get_object('org.gnome.Mutter.RemoteDesktop', '/org/gnome/Mutter/RemoteDesktop')
        iface = dbus.Interface(obj, 'org.gnome.Mutter.RemoteDesktop')
        session_path = iface.CreateSession()
        sess_obj = bus.get_object('org.gnome.Mutter.RemoteDesktop', session_path)
        sess_iface = dbus.Interface(sess_obj, 'org.gnome.Mutter.RemoteDesktop.Session')
        sess_iface.Start()

        try:
            for rep in range(repeats):
                for idx, action in enumerate(macro.actions):
                    if action.delay_ms > 0:
                        time.sleep(action.delay_ms / 1000.0)

                    ks = desc_to_keysym(action.desc)
                    is_down = (action.action == "Down")
                    sess_iface.NotifyKeyboardKeysym(dbus.UInt32(ks), is_down)

                    if on_step:
                        on_step(rep + 1, idx + 1, action)
        finally:
            sess_iface.Stop()

        return True, f"Completed playback of '{macro.name}' ({repeats} repeats via GNOME Mutter)."

    def _play_uinput(self, macro: Macro, repeats: int, on_step=None) -> Tuple[bool, str]:
        import libevdev
        for rep in range(repeats):
            for idx, action in enumerate(macro.actions):
                if action.delay_ms > 0:
                    time.sleep(action.delay_ms / 1000.0)

                ev_name = DESC_TO_EVKEY_NAME.get(action.desc)
                if not ev_name:
                    clean = action.desc.replace("Key ", "").upper()
                    ev_name = f"KEY_{clean}"

                ev_code = getattr(libevdev.EV_KEY, ev_name, None)
                if ev_code:
                    val = 1 if action.action == "Down" else 0
                    self.uinput.send_events([
                        libevdev.InputEvent(ev_code, val),
                        libevdev.InputEvent(libevdev.EV_SYN.SYN_REPORT, 0)
                    ])

                if on_step:
                    on_step(rep + 1, idx + 1, action)

        return True, f"Completed playback of '{macro.name}' ({repeats} repeats via uinput)."


