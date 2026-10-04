"""
GUI Application for FREE WOLF K8 Linux Configuration
Replicating the official Windows application visual layout, capabilities, and settings.
Supports dynamic multi-language localization (EN, PT, ES, FR, DE).
"""
import os
import sys
import time
import tkinter as tk
from tkinter import ttk, messagebox, filedialog, simpledialog
import subprocess
from typing import Optional, List, Dict, Tuple

from .protocol import LIGHT_MODES, LightMode
from .driver import FreeWolfK8Driver, DeviceState
from .music import MusicVisualizerEngine
from .macro import (
    MacroManager, Macro, MacroAction, MacroPlayer,
    DELAY_RECORD, DELAY_NONE, DELAY_DEFAULT,
    get_key_info
)
from .config import ConfigManager
from .i18n import t, get_mode_name, LANGUAGES, LANG_CODE_BY_NAME
from .manual import get_topic_titles, get_topic_content
from .udev import check_access, prompt_password_dialog

ASSETS_DIR = os.path.join(os.path.dirname(os.path.dirname(__file__)), "assets")


class DarkAutoScrollbar(ttk.Scrollbar):
    """A vertical scrollbar with dark theme styling that only shows when content overflows."""
    def __init__(self, master=None, **kwargs):
        super().__init__(master, orient="vertical", **kwargs)
        self._is_packed = False

    def set(self, lo, hi):
        flo, fhi = float(lo), float(hi)
        if flo <= 0.0 and fhi >= 1.0:
            if self._is_packed:
                self.pack_forget()
                self._is_packed = False
        else:
            if not self._is_packed:
                self.pack(side="right", fill="y")
                self._is_packed = True
# =============================================================================
# ARGONAUT GNOME 50 THEME PALETTE & DESIGN TOKENS
# =============================================================================
C_WIN_BG = "#0e1019"          # Base window & canvas background
C_VIEW_BG = "#101321"         # Elevated view background (treeview, listbox, entries)
C_CARD_BG = "#151829"         # Card/panel surface background
C_CARD_BORDER = "#232840"     # Card border / separator line
C_SIDEBAR_BG = "#0e1019"      # Left navigation sidebar
C_SIDEBAR_BORDER = "#181c2e"  # Sidebar vertical border
C_SIDEBAR_ACTIVE = "#132238"  # Active tab background tint

C_ACCENT = "#027ad7"          # Argonaut vibrant blue accent
C_ACCENT_HOVER = "#1a8fe5"    # Accent hover
C_ACCENT_ACTIVE = "#0266b5"   # Accent pressed
C_ACCENT_FG = "#ffffff"

C_DESTRUCTIVE = "#ED5F5D"     # Argonaut coral red
C_DESTRUCTIVE_HOVER = "#f0716f"
C_SUCCESS = "#8ce10b"         # Argonaut lime green
C_WARNING = "#ffb900"         # Argonaut gold / amber

C_FG_PRIMARY = "#ffffff"      # High contrast titles / active text
C_FG_BODY = "#d8dee9"         # Standard Argonaut body text (Nord light)
C_FG_MUTED = "#7e88a0"        # Subtle secondary hints & units
C_FG_DISABLED = "#454b66"     # Disabled buttons & text

C_BTN_BG = "#1a1e32"          # Regular button background
C_BTN_HOVER = "#222842"       # Hovered button background
C_BTN_BORDER = "#262c45"      # Button border
C_BTN_DISABLED_BG = "#131626"

FONT_FAMILY = "Adwaita Sans"
FONT_REGULAR = (FONT_FAMILY, 10)
FONT_BOLD = (FONT_FAMILY, 10, "bold")
FONT_TITLE = (FONT_FAMILY, 11, "bold")
FONT_HEADER = (FONT_FAMILY, 16, "bold")
FONT_APP_HEADER = (FONT_FAMILY, 17, "bold")
FONT_SMALL = (FONT_FAMILY, 9)
FONT_SMALL_BOLD = (FONT_FAMILY, 9, "bold")
FONT_BADGE = (FONT_FAMILY, 11, "bold")
FONT_STATUS = (FONT_FAMILY, 9)


class FreeWolfK8App(tk.Tk):
    def __init__(self):
        super().__init__()
        self.configure(bg=C_WIN_BG)
        
        # Load user settings & configuration
        self.config_mgr = ConfigManager()
        self.current_lang = self.config_mgr.settings.language
        if self.current_lang not in LANGUAGES:
            self.current_lang = "en"

        self.title(t(self.current_lang, "app_title"))
        self.geometry("835x524")
        self.minsize(835, 524)
        self.resizable(False, False)

        # Set window icon
        icon_path = os.path.join(ASSETS_DIR, "DeviceDriver.png")
        if os.path.exists(icon_path):
            try:
                img = tk.PhotoImage(file=icon_path)
                self.iconphoto(True, img)
            except Exception:
                pass

        # Initialize hardware driver & engines
        self.driver = FreeWolfK8Driver()
        self.music_engine = MusicVisualizerEngine(self.driver)
        self.macro_mgr = MacroManager()

        # Lighting State from saved settings
        mode_id = self.config_mgr.settings.mode_id
        matched_modes = [m for m in LIGHT_MODES if m.id == mode_id]
        self.current_mode: LightMode = matched_modes[0] if matched_modes else LIGHT_MODES[3]
        self.brightness = min(4, max(0, self.config_mgr.settings.brightness))
        self.speed = min(4, max(0, self.config_mgr.settings.speed))
        self.music_submode = self.config_mgr.settings.music_submode
        self.music_delay = self.config_mgr.settings.music_delay
        self.is_visualizer_active = False

        # Macro State
        self.current_macro: Optional[Macro] = None
        self.is_recording = False
        self.record_last_time: Optional[float] = None

        # Tab Navigation State ("Light", "Macro", "Help")
        self.current_tab = "Light"

        # Canvas item tracking for view switching
        self.light_canvas_items: List[int] = []
        self.macro_canvas_items: List[int] = []
        self.help_canvas_items: List[int] = []
        self.current_help_topic: int = 0

        # Load graphical assets
        self._load_assets()

        # Build UI layout
        self._build_ui()

        # Apply localized strings to all widgets
        self._apply_language()

        # Start background hardware poller
        self._poll_hardware()

        # Check udev & device permissions on startup
        self.after(350, self._check_startup_permissions)

    def _load_assets(self):
        from PIL import Image, ImageTk
        import io, base64

        def load_img(rel_path, size=None):
            full_path = os.path.join(ASSETS_DIR, rel_path)
            if os.path.exists(full_path):
                try:
                    im = Image.open(full_path)
                    if size:
                        im = im.resize(size, Image.Resampling.LANCZOS)
                    return ImageTk.PhotoImage(im)
                except Exception:
                    return tk.PhotoImage(file=full_path)
            return None

        def load_sprite_frame(rel_path, frame_idx=0, frame_w=36):
            full_path = os.path.join(ASSETS_DIR, rel_path)
            if os.path.exists(full_path):
                try:
                    im = Image.open(full_path)
                    cropped = im.crop((frame_idx * frame_w, 0, (frame_idx + 1) * frame_w, im.height))
                    buf = io.BytesIO()
                    cropped.save(buf, format="PNG")
                    return tk.PhotoImage(data=base64.b64encode(buf.getvalue()).decode("ascii"))
                except Exception:
                    return None
            return None

        # Overview keyboard image for Help tab
        self.img_kb_overview = load_img("keyboard/kb_102.png", size=(420, 204))

        # Tab icons for left navbar: [0] = inactive, [1] = active
        self.tab_icons = {
            "Light": (load_sprite_frame("icon/tab_config.png", 0), load_sprite_frame("icon/tab_config.png", 2)),
            "Macro": (load_sprite_frame("icon/tab_customkey.png", 0), load_sprite_frame("icon/tab_customkey.png", 2)),
            "Help": (load_sprite_frame("icon/tab_help.png", 0), load_sprite_frame("icon/tab_help.png", 2)),
        }

        # Macro Record table small action buttons (Delete, Up, Down)
        self.img_btn_del = load_sprite_frame("btn_record_del.png", 0, 20)
        self.img_btn_up = load_sprite_frame("btn_record_up.png", 0, 20)
        self.img_btn_down = load_sprite_frame("btn_record_down.png", 0, 20)

    def _build_ui(self):
        # Configure custom TTK styles
        style = ttk.Style()
        try:
            style.theme_use("clam")
        except Exception:
            pass

        try:
            import tkinter.font as tkfont
            tkfont.nametofont("TkCaptionFont").configure(weight="normal")
            tkfont.nametofont("TkHeadingFont").configure(weight="normal")
        except Exception:
            pass

        # Argonaut Slider Style (Clean modern GNOME look)
        style.configure(
            "TScale",
            troughcolor=C_VIEW_BG,
            background=C_ACCENT,
            bordercolor=C_CARD_BORDER,
            lightcolor=C_ACCENT,
            darkcolor=C_ACCENT,
            sliderlength=22,
            sliderthickness=14,
            gripcount=0,
            relief="flat",
            borderwidth=0
        )
        style.map(
            "TScale",
            background=[("active", C_ACCENT_HOVER), ("disabled", C_BTN_DISABLED_BG)],
            lightcolor=[("active", C_ACCENT_HOVER), ("disabled", C_BTN_DISABLED_BG)],
            darkcolor=[("active", C_ACCENT_HOVER), ("disabled", C_BTN_DISABLED_BG)],
            bordercolor=[("disabled", C_CARD_BORDER)]
        )

        # Argonaut Scrollbar Style
        style.configure(
            "Vertical.TScrollbar",
            background=C_BTN_BG,
            troughcolor=C_VIEW_BG,
            bordercolor=C_VIEW_BG,
            arrowcolor=C_FG_MUTED,
            lightcolor=C_BTN_BG,
            darkcolor=C_BTN_BG,
            relief="flat",
            borderwidth=0,
            arrowsize=11
        )
        style.map(
            "Vertical.TScrollbar",
            background=[("pressed", C_ACCENT_ACTIVE), ("active", C_ACCENT)],
            arrowcolor=[("pressed", C_FG_PRIMARY), ("active", C_FG_PRIMARY)]
        )

        # Argonaut Treeview Style for Macro Record Table
        style.configure(
            "Macro.Treeview",
            background=C_VIEW_BG,
            foreground=C_FG_BODY,
            fieldbackground=C_VIEW_BG,
            bordercolor=C_SIDEBAR_BORDER,
            lightcolor=C_VIEW_BG,
            darkcolor=C_VIEW_BG,
            rowheight=24,
            borderwidth=0,
            font=FONT_REGULAR
        )
        style.configure(
            "Macro.Treeview.Heading",
            background=C_CARD_BG,
            foreground=C_FG_BODY,
            bordercolor=C_CARD_BORDER,
            lightcolor=C_CARD_BG,
            darkcolor=C_CARD_BG,
            relief="flat",
            borderwidth=0,
            font=FONT_BOLD
        )
        style.map(
            "Macro.Treeview.Heading",
            background=[("pressed", C_VIEW_BG), ("active", C_BTN_HOVER)],
            foreground=[("pressed", C_FG_PRIMARY), ("active", C_FG_PRIMARY)],
            lightcolor=[("active", C_BTN_HOVER)],
            darkcolor=[("active", C_BTN_HOVER)],
            bordercolor=[("active", C_BTN_HOVER)]
        )
        style.map(
            "Macro.Treeview",
            background=[("selected", C_ACCENT)],
            foreground=[("selected", C_ACCENT_FG)],
            bordercolor=[("focus", C_SIDEBAR_BORDER)]
        )

        # Root option database for Combobox popup Listbox (matching Argonaut UI)
        self.option_add("*TCombobox*Listbox.background", C_CARD_BG)
        self.option_add("*TCombobox*Listbox.foreground", C_FG_BODY)
        self.option_add("*TCombobox*Listbox.selectBackground", C_ACCENT)
        self.option_add("*TCombobox*Listbox.selectForeground", C_ACCENT_FG)
        self.option_add("*TCombobox*Listbox.font", FONT_REGULAR)
        self.option_add("*ComboboxListbox*background", C_CARD_BG)
        self.option_add("*ComboboxListbox*foreground", C_FG_BODY)
        self.option_add("*ComboboxListbox*selectBackground", C_ACCENT)
        self.option_add("*ComboboxListbox*selectForeground", C_ACCENT_FG)
        self.option_add("*ComboboxListbox*font", FONT_REGULAR)

        # Combobox style matching Argonaut UI
        style.map(
            "TCombobox",
            fieldbackground=[("readonly", C_CARD_BG), ("!disabled", C_CARD_BG)],
            background=[("readonly", C_CARD_BG), ("active", C_BTN_HOVER), ("!disabled", C_CARD_BG)],
            foreground=[("readonly", C_FG_BODY), ("!disabled", C_FG_BODY)],
            selectbackground=[("readonly", C_CARD_BG)],
            selectforeground=[("readonly", C_FG_BODY)],
            arrowcolor=[("readonly", C_FG_BODY), ("!disabled", C_FG_BODY)]
        )
        style.configure(
            "TCombobox",
            fieldbackground=C_CARD_BG,
            background=C_CARD_BG,
            foreground=C_FG_BODY,
            darkcolor=C_CARD_BG,
            lightcolor=C_CARD_BG,
            bordercolor=C_CARD_BORDER,
            arrowcolor=C_FG_BODY,
            padding=5,
            relief="solid",
            borderwidth=1
        )

        # 1. Main Canvas displaying background
        self.canvas = tk.Canvas(self, width=835, height=524, highlightthickness=0, bg=C_WIN_BG)
        self.canvas.pack(fill="both", expand=True)

        # 2. Window Header (icons, tab text, and center brand text removed as requested)
        self._build_header()

        # 3. Left Navigation Bar (Tab Strip)
        self._build_navbar()

        # 4. Light View Components
        self._build_light_view()

        # 5. Macro View Components
        self._build_macro_view()

        # 6. Help View Components
        self._build_help_view()

        # Switch to initial view
        self._switch_tab(self.current_tab)

    def _style_combobox_popdown(self, combo: ttk.Combobox):
        """Forces TTK combobox popdown listbox to match Argonaut dark theme on Linux."""
        try:
            popdown = self.tk.eval(f'ttk::combobox::PopdownWindow {combo}')
            self.tk.eval(f'{popdown}.f.l configure -background "{C_CARD_BG}" -foreground "{C_FG_BODY}" -selectbackground "{C_ACCENT}" -selectforeground "{C_ACCENT_FG}" -relief flat -bd 0')
            self.tk.eval(f'{popdown} configure -background "{C_CARD_BG}"')
            self.tk.eval(f'{popdown}.f configure -background "{C_CARD_BG}"')
        except Exception:
            pass

    def _build_header(self):
        """Header bar: big icon, tab text, and FREE WOLF K8 text removed per request."""
        pass

    def _build_navbar(self):
        self.bar_w = 48
        self.canvas.create_rectangle(0, 0, self.bar_w, 524, fill=C_SIDEBAR_BG, outline=C_SIDEBAR_BORDER)

        self.nav_tabs = [
            ("Light", "tab_config"),
            ("Macro", "tab_macro"),
            ("Help", "tab_help"),
        ]
        self._render_navbar()

    def _render_navbar(self):
        self.canvas.delete("nav_item")
        y = 95
        for tab_id, tab_key in self.nav_tabs:
            is_active = (self.current_tab == tab_id)
            icons = self.tab_icons.get(tab_id)
            icon = icons[1] if (is_active and icons) else (icons[0] if icons else None)

            if is_active:
                # Argonaut accent highlight indicator on left edge
                self.canvas.create_rectangle(0, y - 22, 3, y + 22, fill=C_ACCENT, outline="", tags="nav_item")
                self.canvas.create_rectangle(3, y - 22, self.bar_w, y + 22, fill=C_SIDEBAR_ACTIVE, outline="", tags="nav_item")

            if icon:
                self.canvas.create_image(self.bar_w // 2, y, image=icon, anchor="center", tags="nav_item")

            # Clickable bounding box for switching tabs
            hit_box = self.canvas.create_rectangle(
                0, y - 26, self.bar_w, y + 26, fill="", outline="", tags="nav_item"
            )
            self.canvas.tag_bind(hit_box, "<Button-1>", lambda e, tid=tab_id: self._switch_tab(tid))

            y += 70

    def _switch_tab(self, tab_id: str):
        if tab_id not in ("Light", "Macro", "Help"):
            tab_id = "Light"

        # Stop recording if switching away from macro
        if self.is_recording and tab_id != "Macro":
            self._toggle_record()

        self.current_tab = tab_id
        self._render_navbar()

        # Hide all view items first
        all_items = (
            self.light_canvas_items +
            self.macro_canvas_items +
            self.help_canvas_items
        )
        for item in all_items:
            self.canvas.itemconfigure(item, state="hidden")

        # Activate selected view
        if tab_id == "Light":
            for item in self.light_canvas_items:
                self.canvas.itemconfigure(item, state="normal")

        elif tab_id == "Macro":
            for item in self.macro_canvas_items:
                self.canvas.itemconfigure(item, state="normal")
            self._refresh_macro_list()

        elif tab_id == "Help":
            for item in self.help_canvas_items:
                self.canvas.itemconfigure(item, state="normal")
            self._render_help_topic(self.current_help_topic)

    # =========================================================================
    # LIGHT VIEW
    # =========================================================================
    def _build_light_view(self):
        # ---------------------------------------------------------------------
        # 1. LEFT PANEL: Device & System Info (x=56, y=15, w=210, h=494)
        # ---------------------------------------------------------------------
        self.info_panel = tk.Frame(self, bg=C_CARD_BG, bd=1, relief="solid", highlightbackground=C_CARD_BORDER)
        win_info = self.canvas.create_window(56, 15, window=self.info_panel, anchor="nw", width=210, height=494)
        self.light_canvas_items.append(win_info)

        # 1. Versioning anchored at bottom left of info panel
        ver_box = tk.Frame(self.info_panel, bg=C_CARD_BG, padx=14)
        ver_box.pack(side="bottom", fill="x", pady=(0, 10))
        self.lbl_ver = tk.Label(ver_box, text="Ver: 1.0.3.1", font=FONT_SMALL, fg=C_FG_MUTED, bg=C_CARD_BG, anchor="w")
        self.lbl_ver.pack(side="left")

        # 2. Connection Status immediately above version
        status_box = tk.Frame(self.info_panel, bg=C_CARD_BG, padx=14)
        status_box.pack(side="bottom", fill="x", pady=(0, 6))

        status_hdr = tk.Frame(status_box, bg=C_CARD_BG)
        status_hdr.pack(fill="x")

        self.status_canvas = tk.Canvas(status_hdr, width=12, height=12, bg=C_CARD_BG, highlightthickness=0)
        self.status_canvas.pack(side="left", padx=(0, 6))
        self.status_circle = self.status_canvas.create_oval(1, 1, 11, 11, fill=C_DESTRUCTIVE, outline="")

        self.lbl_status = tk.Label(status_hdr, text=t(self.current_lang, "status_probing"), font=FONT_SMALL_BOLD, fg=C_FG_PRIMARY, bg=C_CARD_BG, anchor="w", wraplength=160, justify="left")
        self.lbl_status.pack(side="left", fill="x", expand=True)

        self.lbl_status_node = tk.Label(status_box, text="", font=FONT_SMALL, fg=C_FG_MUTED, bg=C_CARD_BG, anchor="w", wraplength=160, justify="left")
        self.lbl_status_node.pack(fill="x", padx=(18, 0), pady=(1, 0))

        self.btn_fix_udev = tk.Button(
            status_box, text=t(self.current_lang, "btn_fix_udev"), font=FONT_SMALL_BOLD,
            bg=C_ACCENT, fg=C_ACCENT_FG, activebackground=C_ACCENT_HOVER, activeforeground=C_ACCENT_FG,
            bd=0, padx=8, pady=2, relief="flat", command=self._run_udev_fix
        )

        # 3. Divider above connection status
        tk.Frame(self.info_panel, height=1, bg=C_CARD_BORDER).pack(side="bottom", fill="x", padx=14, pady=(0, 8))

        # Header matching Macro List style: Device Connected
        self.lbl_conn = tk.Label(self.info_panel, text=t(self.current_lang, "dev_connected"), font=FONT_TITLE, fg=C_FG_PRIMARY, bg=C_CARD_BG, wraplength=180, justify="center")
        self.lbl_conn.pack(pady=(10, 8), padx=6, fill="x")

        # Device connection info & badge
        dev_box = tk.Frame(self.info_panel, bg=C_CARD_BG, padx=12)
        dev_box.pack(fill="x", pady=(0, 8))

        self.dev_badge = tk.Label(
            dev_box, text=t(self.current_lang, "dev_badge_connected"), font=FONT_BADGE,
            fg=C_SUCCESS, bg=C_VIEW_BG, bd=1, relief="solid", highlightbackground=C_SUCCESS,
            padx=6, pady=5, wraplength=165, justify="center"
        )
        self.dev_badge.pack(fill="x", pady=(0, 6))

        self.lbl_conn_detail = tk.Label(
            dev_box, text="VID: 0x1A2C  PID: 0x7C80\nInterface 1 (HID)",
            font=FONT_SMALL, fg=C_FG_MUTED, bg=C_CARD_BG, justify="center", wraplength=180
        )
        self.lbl_conn_detail.pack()

        # Divider 1
        tk.Frame(self.info_panel, height=1, bg=C_CARD_BORDER).pack(fill="x", padx=14, pady=10)

        # Language selection
        lang_box = tk.Frame(self.info_panel, bg=C_CARD_BG, padx=14)
        lang_box.pack(fill="x", pady=(0, 8))

        self.lbl_lang = tk.Label(lang_box, text=t(self.current_lang, "language"), font=FONT_BOLD, fg=C_FG_PRIMARY, bg=C_CARD_BG, anchor="w")
        self.lbl_lang.pack(anchor="w", pady=(0, 5))

        lang_display_names = list(LANGUAGES.values())
        self.combo_lang = ttk.Combobox(lang_box, values=lang_display_names, state="readonly", width=14)
        current_name = LANGUAGES.get(self.current_lang, "English")
        try:
            self.combo_lang.current(lang_display_names.index(current_name))
        except ValueError:
            self.combo_lang.current(0)
        self.combo_lang.pack(fill="x")
        self.combo_lang.bind("<<ComboboxSelected>>", self._on_language_selected)
        self._style_combobox_popdown(self.combo_lang)

        # Divider 2
        tk.Frame(self.info_panel, height=1, bg=C_CARD_BORDER).pack(fill="x", padx=14, pady=10)

        # Options: Auto Run Checkbox
        opt_box = tk.Frame(self.info_panel, bg=C_CARD_BG, padx=14)
        opt_box.pack(fill="x", pady=(0, 8))

        self.var_autorun = tk.BooleanVar(value=self.config_mgr.settings.auto_run)
        self.cb_autorun = tk.Checkbutton(
            opt_box, text=t(self.current_lang, "auto_run"), variable=self.var_autorun,
            bg=C_CARD_BG, fg=C_FG_BODY, activebackground=C_CARD_BG, activeforeground=C_FG_PRIMARY,
            selectcolor=C_ACCENT, font=FONT_REGULAR, bd=0, highlightthickness=0, relief="flat",
            wraplength=155, justify="left",
            command=self._on_autorun_toggled
        )
        self.cb_autorun.pack(anchor="w")

        # Divider 3
        tk.Frame(self.info_panel, height=1, bg=C_CARD_BORDER).pack(fill="x", padx=14, pady=10)

        # Reset Settings Button
        btn_box = tk.Frame(self.info_panel, bg=C_CARD_BG, padx=14)
        btn_box.pack(fill="x", pady=(4, 8))

        self.btn_restore = tk.Button(
            btn_box, text=t(self.current_lang, "restore_factory"), font=FONT_SMALL_BOLD,
            bg=C_BTN_BG, fg=C_FG_PRIMARY, activebackground=C_BTN_HOVER, activeforeground=C_FG_PRIMARY,
            bd=0, relief="flat", highlightbackground=C_BTN_BORDER, highlightthickness=1,
            padx=4, pady=6, wraplength=140, justify="center",
            command=self._on_factory_reset
        )
        self.btn_restore.pack(fill="x")

        # ---------------------------------------------------------------------
        # 2. RIGHT PANEL: Lighting Modes & Controls (x=280, y=15, w=530, h=494)
        # ---------------------------------------------------------------------
        self.mode_frame = tk.Frame(self, bg=C_CARD_BG, bd=1, relief="solid", highlightbackground=C_CARD_BORDER)
        win_mode = self.canvas.create_window(280, 15, window=self.mode_frame, anchor="nw", width=530, height=494)
        self.light_canvas_items.append(win_mode)

        # Header matching Macro List
        self.lbl_light_modes_title = tk.Label(self.mode_frame, text=t(self.current_lang, "lighting_modes"), font=FONT_TITLE, fg=C_FG_PRIMARY, bg=C_CARD_BG)
        self.lbl_light_modes_title.pack(pady=(10, 8), padx=6, fill="x")

        # Divider under header
        tk.Frame(self.mode_frame, height=1, bg=C_CARD_BORDER).pack(fill="x")

        # Dual-column mode listing without scroll
        grid_frame = tk.Frame(self.mode_frame, bg=C_CARD_BG, padx=18, pady=8)
        grid_frame.pack(fill="both")
        grid_frame.columnconfigure(0, weight=1, uniform="col")
        grid_frame.columnconfigure(1, weight=1, uniform="col")

        self.mode_var = tk.IntVar(value=self.current_mode.id)
        self.mode_radio_buttons = []

        for i, m in enumerate(LIGHT_MODES):
            row = i // 2
            col = i % 2
            rb = tk.Radiobutton(
                grid_frame,
                text=get_mode_name(self.current_lang, m.id, m.name),
                variable=self.mode_var,
                value=m.id,
                command=self._on_mode_selected,
                bg=C_CARD_BG,
                fg=C_FG_BODY,
                activebackground=C_BTN_HOVER,
                activeforeground=C_FG_PRIMARY,
                selectcolor=C_ACCENT,
                font=FONT_REGULAR,
                anchor="w",
                highlightthickness=0,
                bd=0
            )
            rb.grid(row=row, column=col, sticky="w", padx=12, pady=1)
            self.mode_radio_buttons.append((m, rb, None))

        # Horizontal Divider above controls
        tk.Frame(self.mode_frame, height=1, bg=C_CARD_BORDER).pack(fill="x", pady=(6, 8))

        # Controls (Brightness & Speed / Music) - Centered below modes
        self.controls_frame = tk.Frame(self.mode_frame, bg=C_CARD_BG)
        self.controls_frame.pack(fill="both", expand=True)

        # Standard Controls (Brightness & Speed Sliders)
        self.std_controls = tk.Frame(self.controls_frame, bg=C_CARD_BG)
        self.std_controls.pack(fill="both", expand=True, pady=(2, 4))

        sliders_grid = tk.Frame(self.std_controls, bg=C_CARD_BG)
        sliders_grid.pack(anchor="center", pady=(4, 0))

        # Brightness row
        self.lbl_b_title = tk.Label(sliders_grid, text=t(self.current_lang, "light_brightness"), font=FONT_BOLD, fg=C_FG_PRIMARY, bg=C_CARD_BG, anchor="e")
        self.lbl_b_title.grid(row=0, column=0, padx=(0, 14), pady=6, sticky="e")

        self.slider_brightness = ttk.Scale(
            sliders_grid, from_=0, to=4, orient="horizontal", length=280, command=self._on_brightness_change
        )
        self.slider_brightness.grid(row=0, column=1, padx=4, pady=6)
        self.slider_brightness.set(self.brightness)

        # Speed row
        self.lbl_s_title = tk.Label(sliders_grid, text=t(self.current_lang, "light_speed"), font=FONT_BOLD, fg=C_FG_PRIMARY, bg=C_CARD_BG, anchor="e")
        self.lbl_s_title.grid(row=1, column=0, padx=(0, 14), pady=6, sticky="e")

        self.slider_speed = ttk.Scale(
            sliders_grid, from_=0, to=4, orient="horizontal", length=280, command=self._on_speed_change
        )
        self.slider_speed.grid(row=1, column=1, padx=4, pady=6)
        self.slider_speed.set(self.speed)

        # Music Controls
        self.music_controls = tk.Frame(self.controls_frame, bg=C_CARD_BG)

        m_center_box = tk.Frame(self.music_controls, bg=C_CARD_BG)
        m_center_box.pack(anchor="center", pady=2)

        self.lbl_m_title = tk.Label(m_center_box, text=t(self.current_lang, "music_title"), font=FONT_TITLE, fg=C_ACCENT, bg=C_CARD_BG)
        self.lbl_m_title.pack(pady=(0, 6))

        m_grid = tk.Frame(m_center_box, bg=C_CARD_BG)
        m_grid.pack(pady=(0, 8))

        self.lbl_pattern_title = tk.Label(m_grid, text=t(self.current_lang, "music_pattern"), font=FONT_BOLD, fg=C_FG_PRIMARY, bg=C_CARD_BG, anchor="e")
        self.lbl_pattern_title.grid(row=0, column=0, pady=3, padx=(0, 10), sticky="e")

        pattern_names = [t(self.current_lang, "music_mode1"), t(self.current_lang, "music_mode2")]
        self.combo_music_pattern = ttk.Combobox(m_grid, values=pattern_names, state="readonly", width=18)
        self.combo_music_pattern.current(self.music_submode - 1 if self.music_submode in (1, 2) else 0)
        self.combo_music_pattern.grid(row=0, column=1, pady=3, sticky="w")
        self.combo_music_pattern.bind("<<ComboboxSelected>>", self._on_music_submode_change)
        self._style_combobox_popdown(self.combo_music_pattern)

        self.lbl_freq_title = tk.Label(m_grid, text=t(self.current_lang, "music_freq"), font=FONT_BOLD, fg=C_FG_PRIMARY, bg=C_CARD_BG, anchor="e")
        self.lbl_freq_title.grid(row=1, column=0, pady=3, padx=(0, 10), sticky="e")

        freq_options = ["33ms (30Hz)", "66ms (15Hz)", "100ms (10Hz)"]
        self.combo_music_freq = ttk.Combobox(m_grid, values=freq_options, state="readonly", width=18)
        cur_idx = 0 if self.music_delay == 33 else (1 if self.music_delay == 66 else 2)
        self.combo_music_freq.current(cur_idx)
        self.combo_music_freq.grid(row=1, column=1, pady=3, sticky="w")
        self.combo_music_freq.bind("<<ComboboxSelected>>", self._on_music_delay_change)
        self._style_combobox_popdown(self.combo_music_freq)

        row3 = tk.Frame(m_center_box, bg=C_CARD_BG)
        row3.pack()
        self.btn_music_toggle = tk.Button(
            row3, text=t(self.current_lang, "music_start"), font=FONT_BOLD,
            bg=C_ACCENT, fg=C_ACCENT_FG, activebackground=C_ACCENT_HOVER, activeforeground=C_ACCENT_FG,
            bd=0, padx=18, pady=5, relief="flat", command=self._toggle_visualizer
        )
        self.btn_music_toggle.pack(side="left")

        self.lbl_music_status = tk.Label(row3, text=t(self.current_lang, "music_idle"), font=FONT_REGULAR, fg=C_FG_MUTED, bg=C_CARD_BG)
        self.lbl_music_status.pack(side="left", padx=15)

        # Initialize brightness/speed slider states for current mode
        self._update_controls_for_mode(self.current_mode)

    # =========================================================================
    # MACRO VIEW (Matching Screenshot From 2026-10-02 15-10-45.png)
    # =========================================================================
    def _build_macro_view(self):
        # 1. Macro List Panel (Left Column, realigned with right side at y=15)
        panel_x = 56
        panel_y = 15
        panel_w = 210
        panel_h = 494

        self.macro_list_panel = tk.Frame(self, bg=C_CARD_BG, bd=1, relief="solid", highlightbackground=C_CARD_BORDER)
        win_list = self.canvas.create_window(panel_x, panel_y, window=self.macro_list_panel, anchor="nw", width=panel_w, height=panel_h)
        self.macro_canvas_items.append(win_list)

        self.lbl_macro_list_title = tk.Label(self.macro_list_panel, text=t(self.current_lang, "macro_list"), font=FONT_TITLE, fg=C_FG_PRIMARY, bg=C_CARD_BG)
        self.lbl_macro_list_title.pack(pady=(10, 8), padx=6, fill="x")

        list_container = tk.Frame(self.macro_list_panel, bg=C_CARD_BG)
        list_container.pack(fill="both", expand=True, padx=8, pady=(0, 10))

        self.macro_listbox = tk.Listbox(
            list_container,
            bg=C_VIEW_BG,
            fg=C_FG_BODY,
            selectbackground=C_ACCENT,
            selectforeground=C_ACCENT_FG,
            font=FONT_REGULAR,
            bd=0,
            highlightthickness=0,
            activestyle="none",
            exportselection=False
        )
        macro_scrollbar = DarkAutoScrollbar(list_container, command=self.macro_listbox.yview)
        self.macro_listbox.configure(yscrollcommand=macro_scrollbar.set)
        self.macro_listbox.pack(side="left", fill="both", expand=True)
        self.macro_listbox.bind("<<ListboxSelect>>", self._on_macro_selected)

        # 2. Controls & Settings Toolbar ABOVE Macro Record (x=280, y=15, w=530, h=155)
        self.macro_mid_panel = tk.Frame(self, bg=C_CARD_BG, bd=1, relief="solid", highlightbackground=C_CARD_BORDER, padx=16, pady=10)
        win_mid = self.canvas.create_window(280, 15, window=self.macro_mid_panel, anchor="nw", width=530, height=155)
        self.macro_canvas_items.append(win_mid)

        # Action Buttons Grid on left half (3 rows x 2 columns)
        btn_grid = tk.Frame(self.macro_mid_panel, bg=C_CARD_BG)
        btn_grid.pack(side="left", fill="y", padx=(2, 12))

        self.btn_macro_new = tk.Button(btn_grid, text=t(self.current_lang, "btn_new"), font=FONT_SMALL_BOLD, width=10, pady=3, command=self._on_new_macro)
        self.btn_macro_del = tk.Button(btn_grid, text=t(self.current_lang, "btn_delete"), font=FONT_SMALL_BOLD, width=10, pady=3, command=self._on_delete_macro)
        self.btn_macro_copy = tk.Button(btn_grid, text=t(self.current_lang, "btn_copy"), font=FONT_SMALL_BOLD, width=10, pady=3, command=self._on_copy_macro)
        self.btn_macro_rename = tk.Button(btn_grid, text=t(self.current_lang, "btn_rename"), font=FONT_SMALL_BOLD, width=10, pady=3, command=self._on_rename_macro)
        self.btn_macro_import = tk.Button(btn_grid, text=t(self.current_lang, "btn_import"), font=FONT_SMALL_BOLD, width=10, pady=3, command=self._on_import_macro)
        self.btn_macro_export = tk.Button(btn_grid, text=t(self.current_lang, "btn_export"), font=FONT_SMALL_BOLD, width=10, pady=3, command=self._on_export_macro)

        self.btn_macro_new.grid(row=0, column=0, padx=3, pady=3)
        self.btn_macro_del.grid(row=0, column=1, padx=3, pady=3)
        self.btn_macro_copy.grid(row=1, column=0, padx=3, pady=3)
        self.btn_macro_rename.grid(row=1, column=1, padx=3, pady=3)
        self.btn_macro_import.grid(row=2, column=0, padx=3, pady=3)
        self.btn_macro_export.grid(row=2, column=1, padx=3, pady=3)

        # Style initial action buttons
        self._style_action_btn(self.btn_macro_new, enabled=True)
        self._style_action_btn(self.btn_macro_del, enabled=False)
        self._style_action_btn(self.btn_macro_copy, enabled=False)
        self._style_action_btn(self.btn_macro_import, enabled=True)
        self._style_action_btn(self.btn_macro_export, enabled=False)
        self._style_action_btn(self.btn_macro_rename, enabled=False)

        # Vertical Divider between Buttons and Settings
        sep = tk.Frame(self.macro_mid_panel, width=1, bg=C_CARD_BORDER)
        sep.pack(side="left", fill="y", padx=10, pady=2)

        # Settings Section on right half (Repeat Time + Vertically Stacked Delay Modes)
        settings_box = tk.Frame(self.macro_mid_panel, bg=C_CARD_BG)
        settings_box.pack(side="left", fill="both", expand=True, padx=(4, 0))

        # Row 0: Repeat Time
        row_rep = tk.Frame(settings_box, bg=C_CARD_BG)
        row_rep.pack(fill="x", pady=(1, 5))
        self.lbl_rep = tk.Label(row_rep, text=t(self.current_lang, "repeat_time"), font=FONT_BOLD, fg=C_FG_PRIMARY, bg=C_CARD_BG, anchor="w")
        self.lbl_rep.pack(side="left")

        self.entry_repeat = tk.Entry(
            row_rep, width=5, font=FONT_REGULAR,
            bg=C_VIEW_BG, fg=C_FG_PRIMARY, insertbackground=C_FG_PRIMARY,
            justify="center", bd=1, relief="solid",
            highlightbackground=C_CARD_BORDER, highlightcolor=C_ACCENT
        )
        self.entry_repeat.pack(side="left", padx=(6, 6))
        self.entry_repeat.insert(0, "1")
        self.entry_repeat.bind("<KeyRelease>", self._on_repeat_time_change)

        self.lbl_rep_unit = tk.Label(row_rep, text="(1 - 9999)", font=FONT_SMALL, fg=C_FG_MUTED, bg=C_CARD_BG)
        self.lbl_rep_unit.pack(side="left")

        # Row 1: Delay Modes (Label on top, Radio buttons stacked below taking full width)
        delay_box = tk.Frame(settings_box, bg=C_CARD_BG)
        delay_box.pack(fill="x", pady=(2, 0))

        self.lbl_delay_title = tk.Label(
            delay_box, text=t(self.current_lang, "delay_mode"),
            font=FONT_BOLD, fg=C_FG_PRIMARY, bg=C_CARD_BG, anchor="w"
        )
        self.lbl_delay_title.pack(side="top", anchor="w", pady=(2, 3))

        radio_stack = tk.Frame(delay_box, bg=C_CARD_BG)
        radio_stack.pack(side="top", fill="x", expand=True)

        self.delay_mode_var = tk.IntVar(value=DELAY_DEFAULT)

        self.rb_record = tk.Radiobutton(
            radio_stack, text=t(self.current_lang, "delay_record"), variable=self.delay_mode_var,
            value=DELAY_RECORD, command=self._on_delay_mode_changed,
            font=FONT_REGULAR, bg=C_CARD_BG, fg=C_FG_BODY,
            activebackground=C_CARD_BG, activeforeground=C_FG_PRIMARY,
            selectcolor=C_ACCENT, highlightthickness=0, bd=0, anchor="w"
        )
        self.rb_record.pack(anchor="w", pady=1)

        self.rb_nodelay = tk.Radiobutton(
            radio_stack, text=t(self.current_lang, "delay_none"), variable=self.delay_mode_var,
            value=DELAY_NONE, command=self._on_delay_mode_changed,
            font=FONT_REGULAR, bg=C_CARD_BG, fg=C_FG_BODY,
            activebackground=C_CARD_BG, activeforeground=C_FG_PRIMARY,
            selectcolor=C_ACCENT, highlightthickness=0, bd=0, anchor="w"
        )
        self.rb_nodelay.pack(anchor="w", pady=1)

        row_def = tk.Frame(radio_stack, bg=C_CARD_BG)
        row_def.pack(anchor="w", pady=1)

        self.rb_default = tk.Radiobutton(
            row_def, text=t(self.current_lang, "delay_default"), variable=self.delay_mode_var,
            value=DELAY_DEFAULT, command=self._on_delay_mode_changed,
            font=FONT_REGULAR, bg=C_CARD_BG, fg=C_FG_BODY,
            activebackground=C_CARD_BG, activeforeground=C_FG_PRIMARY,
            selectcolor=C_ACCENT, highlightthickness=0, bd=0
        )
        self.rb_default.pack(side="left")

        self.entry_default_delay = tk.Entry(
            row_def, width=4, font=FONT_REGULAR,
            bg=C_VIEW_BG, fg=C_FG_PRIMARY, insertbackground=C_FG_PRIMARY,
            justify="center", bd=1, relief="solid",
            highlightbackground=C_CARD_BORDER, highlightcolor=C_ACCENT
        )
        self.entry_default_delay.pack(side="left", padx=(6, 4))
        self.entry_default_delay.insert(0, "10")
        self.entry_default_delay.bind("<KeyRelease>", self._on_default_delay_change)

        self.lbl_ms = tk.Label(row_def, text=t(self.current_lang, "delay_ms"), font=FONT_BOLD, fg=C_FG_PRIMARY, bg=C_CARD_BG)
        self.lbl_ms.pack(side="left")

        # 3. Macro Record Panel BELOW Settings Toolbar (x=280, y=182, w=530, h=327)
        self.macro_right_panel = tk.Frame(self, bg=C_CARD_BG, bd=1, relief="solid", highlightbackground=C_CARD_BORDER, padx=16, pady=10)
        win_right = self.canvas.create_window(280, 182, window=self.macro_right_panel, anchor="nw", width=530, height=327)
        self.macro_canvas_items.append(win_right)

        # Header Row
        hdr_row = tk.Frame(self.macro_right_panel, bg=C_CARD_BG)
        hdr_row.pack(side="top", fill="x", pady=(2, 6))

        self.lbl_rec_title = tk.Label(hdr_row, text=t(self.current_lang, "macro_record"), font=FONT_TITLE, fg=C_FG_PRIMARY, bg=C_CARD_BG)
        self.lbl_rec_title.pack(side="left")

        btns_sub = tk.Frame(hdr_row, bg=C_CARD_BG)
        btns_sub.pack(side="right")

        self.btn_action_del = tk.Button(
            btns_sub, image=self.img_btn_del, bg=C_CARD_BG, activebackground=C_BTN_HOVER,
            bd=0, highlightthickness=0, relief="flat", command=self._on_action_delete
        )
        self.btn_action_up = tk.Button(
            btns_sub, image=self.img_btn_up, bg=C_CARD_BG, activebackground=C_BTN_HOVER,
            bd=0, highlightthickness=0, relief="flat", command=self._on_action_up
        )
        self.btn_action_down = tk.Button(
            btns_sub, image=self.img_btn_down, bg=C_CARD_BG, activebackground=C_BTN_HOVER,
            bd=0, highlightthickness=0, relief="flat", command=self._on_action_down
        )
        self.btn_action_del.pack(side="left", padx=3)
        self.btn_action_up.pack(side="left", padx=3)
        self.btn_action_down.pack(side="left", padx=3)

        # Bottom Controls Row: Hint on left, Record/Play/Save on right (packed at bottom first)
        bottom_row = tk.Frame(self.macro_right_panel, bg=C_CARD_BG)
        bottom_row.pack(side="bottom", fill="x", pady=(6, 2))

        self.lbl_record_hint = tk.Label(bottom_row, text="", font=FONT_SMALL, fg=C_FG_MUTED, bg=C_CARD_BG, wraplength=210, justify="left")
        self.lbl_record_hint.pack(side="left")

        self.btn_macro_save = tk.Button(
            bottom_row, text=t(self.current_lang, "btn_save"), font=FONT_BOLD,
            bg=C_ACCENT, fg=C_ACCENT_FG, activebackground=C_ACCENT_HOVER, activeforeground=C_ACCENT_FG,
            disabledforeground=C_FG_DISABLED,
            bd=0, relief="flat", padx=20, pady=5, command=self._save_macro
        )
        self.btn_macro_save.pack(side="right", padx=(8, 0))

        self.btn_macro_play = tk.Button(
            bottom_row, text=t(self.current_lang, "btn_play"), font=FONT_BOLD,
            bg=C_BTN_BG, fg=C_FG_PRIMARY, activebackground=C_BTN_HOVER, activeforeground=C_FG_PRIMARY,
            disabledforeground=C_FG_DISABLED,
            highlightbackground=C_BTN_BORDER, highlightcolor=C_ACCENT, highlightthickness=1,
            bd=0, relief="flat", padx=16, pady=5, command=self._play_macro
        )
        self.btn_macro_play.pack(side="right", padx=(8, 0))

        self.btn_macro_record = tk.Button(
            bottom_row, text=t(self.current_lang, "btn_record"), font=FONT_BOLD,
            bg=C_BTN_BG, fg=C_FG_PRIMARY, activebackground=C_BTN_HOVER, activeforeground=C_FG_PRIMARY,
            disabledforeground=C_FG_DISABLED,
            highlightbackground=C_BTN_BORDER, highlightcolor=C_ACCENT, highlightthickness=1,
            bd=0, relief="flat", padx=16, pady=5, command=self._toggle_record
        )
        self.btn_macro_record.pack(side="right")

        # Treeview Table (expands in middle between header and bottom controls)
        tree_container = tk.Frame(self.macro_right_panel, bg=C_CARD_BG)
        tree_container.pack(side="top", fill="both", expand=True, pady=4)

        self.macro_tree = ttk.Treeview(
            tree_container,
            columns=("desc", "action", "delay"),
            show="headings",
            style="Macro.Treeview",
            selectmode="browse",
            height=6
        )
        self.macro_tree.heading("desc", text=t(self.current_lang, "col_desc"))
        self.macro_tree.heading("action", text=t(self.current_lang, "col_action"))
        self.macro_tree.heading("delay", text=t(self.current_lang, "col_delay"))

        self.macro_tree.column("desc", width=255, minwidth=180, anchor="w")
        self.macro_tree.column("action", width=115, minwidth=80, anchor="center")
        self.macro_tree.column("delay", width=110, minwidth=80, anchor="center")

        tree_scroll = DarkAutoScrollbar(tree_container, command=self.macro_tree.yview)
        self.macro_tree.configure(yscrollcommand=tree_scroll.set)
        self.macro_tree.pack(side="left", fill="both", expand=True)

        self.macro_tree.bind("<Double-1>", self._on_tree_double_click)
        self.macro_tree.bind("<Delete>", lambda e: self._on_action_delete())

    def _style_action_btn(self, btn: tk.Button, enabled: bool):
        """Applies Argonaut GNOME theme styling for action buttons."""
        if enabled:
            btn.config(
                state="normal",
                bg=C_BTN_BG,
                fg=C_FG_PRIMARY,
                activebackground=C_BTN_HOVER,
                activeforeground=C_FG_PRIMARY,
                highlightbackground=C_BTN_BORDER,
                highlightcolor=C_ACCENT,
                highlightthickness=1,
                bd=0,
                relief="flat"
            )
        else:
            btn.config(
                state="disabled",
                bg=C_BTN_DISABLED_BG,
                fg=C_FG_DISABLED,
                disabledforeground=C_FG_DISABLED,
                activebackground=C_BTN_DISABLED_BG,
                activeforeground=C_FG_DISABLED,
                highlightbackground=C_CARD_BORDER,
                highlightcolor=C_CARD_BORDER,
                highlightthickness=1,
                bd=0,
                relief="flat"
            )

    # =========================================================================
    # MACRO LOGIC & EVENT HANDLERS
    # =========================================================================
    def _refresh_macro_list(self):
        """Populates the macro listbox from MacroManager."""
        self.macro_listbox.delete(0, "end")
        for m in self.macro_mgr.macros:
            self.macro_listbox.insert("end", f"  {m.name}")

        if self.macro_mgr.macros:
            sel_idx = 0
            if self.current_macro:
                for idx, m in enumerate(self.macro_mgr.macros):
                    if m.id == self.current_macro.id:
                        sel_idx = idx
                        break
            self.macro_listbox.selection_set(sel_idx)
            self._on_macro_selected()
        else:
            self.current_macro = None
            self._style_action_btn(self.btn_macro_del, enabled=False)
            self._style_action_btn(self.btn_macro_copy, enabled=False)
            self._style_action_btn(self.btn_macro_export, enabled=False)
            self._style_action_btn(self.btn_macro_rename, enabled=False)
            self.macro_tree.delete(*self.macro_tree.get_children())

    def _on_macro_selected(self, event=None):
        sel = self.macro_listbox.curselection()
        if not sel:
            self._style_action_btn(self.btn_macro_del, enabled=False)
            self._style_action_btn(self.btn_macro_copy, enabled=False)
            self._style_action_btn(self.btn_macro_export, enabled=False)
            self._style_action_btn(self.btn_macro_rename, enabled=False)
            return
        idx = sel[0]
        if idx >= len(self.macro_mgr.macros):
            return

        self.current_macro = self.macro_mgr.macros[idx]

        # Enable macro-specific buttons
        self._style_action_btn(self.btn_macro_del, enabled=True)
        self._style_action_btn(self.btn_macro_copy, enabled=True)
        self._style_action_btn(self.btn_macro_export, enabled=True)
        self._style_action_btn(self.btn_macro_rename, enabled=True)

        # Load Repeat Time & Delay settings
        self.entry_repeat.delete(0, "end")
        self.entry_repeat.insert(0, str(self.current_macro.repeat_time))

        self.delay_mode_var.set(self.current_macro.delay_type)

        self.entry_default_delay.delete(0, "end")
        self.entry_default_delay.insert(0, str(self.current_macro.default_delay))

        # Populate action table
        self._refresh_macro_table()

    def _refresh_macro_table(self):
        self.macro_tree.delete(*self.macro_tree.get_children())
        if not self.current_macro:
            return
        for action in self.current_macro.actions:
            act_str = t(self.current_lang, "action_down") if action.action == "Down" else (
                t(self.current_lang, "action_up") if action.action == "Up" else action.action
            )
            self.macro_tree.insert("", "end", values=(action.desc, act_str, action.delay_ms))

    def _on_repeat_time_change(self, event=None):
        if not self.current_macro:
            return
        val = self.entry_repeat.get().strip()
        try:
            self.current_macro.repeat_time = max(1, int(val))
        except ValueError:
            pass

    def _on_default_delay_change(self, event=None):
        if not self.current_macro:
            return
        val = self.entry_default_delay.get().strip()
        try:
            d_val = max(0, int(val))
            self.current_macro.default_delay = d_val
            if self.delay_mode_var.get() == DELAY_DEFAULT:
                for a in self.current_macro.actions:
                    a.delay_ms = d_val
                self._refresh_macro_table()
        except ValueError:
            pass

    def _on_delay_mode_changed(self):
        if not self.current_macro:
            return
        mode = self.delay_mode_var.get()
        self.current_macro.delay_type = mode

        if mode == DELAY_NONE and self.current_macro.actions:
            for a in self.current_macro.actions:
                a.delay_ms = 0
            self._refresh_macro_table()
        elif mode == DELAY_DEFAULT and self.current_macro.actions:
            try:
                def_delay = max(0, int(self.entry_default_delay.get().strip()))
            except ValueError:
                def_delay = 10
            for a in self.current_macro.actions:
                a.delay_ms = def_delay
            self._refresh_macro_table()

    def _on_new_macro(self):
        new_m = self.macro_mgr.new_macro()
        self.current_macro = new_m
        self._refresh_macro_list()

    def _on_delete_macro(self):
        if not self.current_macro:
            return
        title = t(self.current_lang, "msg_delete_title")
        msg = t(self.current_lang, "msg_delete_text", name=self.current_macro.name)
        if self._confirm_dialog(title, msg):
            self.macro_mgr.delete_macro(self.current_macro.id)
            self.current_macro = None
            self._refresh_macro_list()

    def _on_copy_macro(self):
        if not self.current_macro:
            return
        copied = self.macro_mgr.copy_macro(self.current_macro.id)
        if copied:
            self.current_macro = copied
            self._refresh_macro_list()

    def _on_rename_macro(self):
        if not self.current_macro:
            return
        title = t(self.current_lang, "msg_rename_title")
        prompt = t(self.current_lang, "msg_rename_prompt")
        new_name = simpledialog.askstring(title, prompt, initialvalue=self.current_macro.name, parent=self)
        if new_name and new_name.strip():
            self.macro_mgr.rename_macro(self.current_macro.id, new_name.strip())
            self._refresh_macro_list()

    def _on_import_macro(self):
        filepath = filedialog.askopenfilename(
            title="Import Macro JSON",
            filetypes=[("JSON Macro Files", "*.json"), ("All Files", "*.*")],
            parent=self
        )
        if filepath:
            try:
                m = self.macro_mgr.import_macro(filepath)
                if m:
                    self.current_macro = m
                    self._refresh_macro_list()
                    messagebox.showinfo("Import", t(self.current_lang, "msg_import_success", name=m.name))
            except Exception as e:
                messagebox.showerror("Import Error", f"Failed to import macro: {e}")

    def _on_export_macro(self):
        if not self.current_macro:
            return
        filepath = filedialog.asksaveasfilename(
            title="Export Macro JSON",
            defaultextension=".json",
            initialfile=f"{self.current_macro.name}.json",
            filetypes=[("JSON Macro Files", "*.json"), ("All Files", "*.*")],
            parent=self
        )
        if filepath:
            try:
                self.macro_mgr.export_macro(self.current_macro.id, filepath)
                messagebox.showinfo("Export", t(self.current_lang, "msg_export_success", path=filepath))
            except Exception as e:
                messagebox.showerror("Export Error", f"Failed to export macro: {e}")

    def _on_action_delete(self):
        if not self.current_macro:
            return
        sel = self.macro_tree.selection()
        if not sel:
            return
        idx = self.macro_tree.index(sel[0])
        if self.macro_mgr.delete_action(self.current_macro.id, idx):
            self._refresh_macro_table()
            children = self.macro_tree.get_children()
            if children:
                new_idx = min(idx, len(children) - 1)
                self.macro_tree.selection_set(children[new_idx])

    def _on_action_up(self):
        if not self.current_macro:
            return
        sel = self.macro_tree.selection()
        if not sel:
            return
        idx = self.macro_tree.index(sel[0])
        if idx > 0:
            if self.macro_mgr.reorder_action(self.current_macro.id, idx, idx - 1):
                self._refresh_macro_table()
                children = self.macro_tree.get_children()
                self.macro_tree.selection_set(children[idx - 1])
                self.macro_tree.see(children[idx - 1])

    def _on_action_down(self):
        if not self.current_macro:
            return
        sel = self.macro_tree.selection()
        if not sel:
            return
        idx = self.macro_tree.index(sel[0])
        if idx < len(self.current_macro.actions) - 1:
            if self.macro_mgr.reorder_action(self.current_macro.id, idx, idx + 1):
                self._refresh_macro_table()
                children = self.macro_tree.get_children()
                self.macro_tree.selection_set(children[idx + 1])
                self.macro_tree.see(children[idx + 1])

    def _on_tree_double_click(self, event):
        if not self.current_macro:
            return
        item = self.macro_tree.identify_row(event.y)
        if not item:
            return
        idx = self.macro_tree.index(item)
        action = self.current_macro.actions[idx]

        title = t(self.current_lang, "msg_edit_delay_title")
        prompt = t(self.current_lang, "msg_edit_delay_prompt", desc=action.desc, action=action.action)
        new_val = simpledialog.askinteger(
            title,
            prompt,
            initialvalue=action.delay_ms,
            minvalue=0,
            maxvalue=60000,
            parent=self
        )
        if new_val is not None:
            action.delay_ms = new_val
            self._refresh_macro_table()
            children = self.macro_tree.get_children()
            if idx < len(children):
                self.macro_tree.selection_set(children[idx])

    def _toggle_record(self):
        if not self.is_recording:
            if not self.current_macro:
                self._on_new_macro()

            self.is_recording = True
            self.record_last_time = None
            self.btn_macro_record.config(text=t(self.current_lang, "btn_stop"), bg=C_DESTRUCTIVE, fg=C_ACCENT_FG, activebackground=C_DESTRUCTIVE_HOVER)
            self.lbl_record_hint.config(text=t(self.current_lang, "hint_recording"), fg=C_DESTRUCTIVE)

            # Bind global keystrokes
            self.bind_all("<KeyPress>", self._on_key_record_press)
            self.bind_all("<KeyRelease>", self._on_key_record_release)
            self.focus_set()
        else:
            self.is_recording = False
            self.btn_macro_record.config(
                text=t(self.current_lang, "btn_record"),
                bg=C_BTN_BG,
                fg=C_FG_PRIMARY,
                activebackground=C_BTN_HOVER,
                activeforeground=C_FG_PRIMARY
            )
            count = len(self.current_macro.actions) if self.current_macro else 0
            self.lbl_record_hint.config(text=t(self.current_lang, "hint_recorded", count=count), fg=C_SUCCESS)

            self.unbind_all("<KeyPress>")
            self.unbind_all("<KeyRelease>")

    def _on_key_record_press(self, event):
        if not self.is_recording or not self.current_macro:
            return

        code, desc = get_key_info(event.keysym, event.keycode)
        now = time.time()
        mode = self.delay_mode_var.get()

        if self.record_last_time is None:
            if mode == DELAY_NONE:
                delay = 0
            elif mode == DELAY_DEFAULT:
                try: delay = max(0, int(self.entry_default_delay.get()))
                except ValueError: delay = 10
            else:
                delay = 10
        else:
            elapsed = int((now - self.record_last_time) * 1000)
            if mode == DELAY_RECORD:
                delay = max(1, elapsed)
            elif mode == DELAY_NONE:
                delay = 0
            else:
                try: delay = max(0, int(self.entry_default_delay.get()))
                except ValueError: delay = 10

        self.record_last_time = now

        action = MacroAction(desc=desc, action="Down", delay_ms=delay, keycode=code)
        self.current_macro.actions.append(action)

        act_str = t(self.current_lang, "action_down")
        item = self.macro_tree.insert("", "end", values=(action.desc, act_str, action.delay_ms))
        self.macro_tree.see(item)
        return "break"

    def _on_key_record_release(self, event):
        if not self.is_recording or not self.current_macro:
            return

        code, desc = get_key_info(event.keysym, event.keycode)
        now = time.time()
        mode = self.delay_mode_var.get()

        elapsed = int((now - self.record_last_time) * 1000) if self.record_last_time else 10
        if mode == DELAY_RECORD:
            delay = max(1, elapsed)
        elif mode == DELAY_NONE:
            delay = 0
        else:
            try: delay = max(0, int(self.entry_default_delay.get()))
            except ValueError: delay = 10

        self.record_last_time = now

        action = MacroAction(desc=desc, action="Up", delay_ms=delay, keycode=code)
        self.current_macro.actions.append(action)

        act_str = t(self.current_lang, "action_up")
        item = self.macro_tree.insert("", "end", values=(action.desc, act_str, action.delay_ms))
        self.macro_tree.see(item)
        return "break"

    def _save_macro(self):
        if not self.current_macro:
            return

        # Commit current settings
        try:
            self.current_macro.repeat_time = max(1, int(self.entry_repeat.get().strip()))
        except ValueError:
            pass

        try:
            self.current_macro.default_delay = max(0, int(self.entry_default_delay.get().strip()))
        except ValueError:
            pass

        self.current_macro.delay_type = self.delay_mode_var.get()

        self.macro_mgr.save()
        status_msg = t(self.current_lang, "macro_saved_status", name=self.current_macro.name)
        self.lbl_record_hint.config(text=status_msg, fg=C_SUCCESS)
        messagebox.showinfo(
            t(self.current_lang, "msg_saved_title"),
            t(self.current_lang, "msg_saved_text", name=self.current_macro.name, count=len(self.current_macro.actions))
        )

    def _play_macro(self):
        if not self.current_macro:
            return
        if not self.current_macro.actions:
            messagebox.showinfo("Empty Macro", t(self.current_lang, "msg_empty_macro", name=self.current_macro.name))
            return

        player = MacroPlayer()
        if not player.is_available():
            messagebox.showwarning(
                t(self.current_lang, "virtual_kb_title"),
                t(self.current_lang, "virtual_kb_unavailable", error=player._init_error)
            )
            return

        self.btn_macro_play.config(
            state="disabled",
            bg=C_BTN_DISABLED_BG,
            fg=C_FG_DISABLED,
            disabledforeground=C_FG_DISABLED
        )
        hint_text = t(self.current_lang, "hint_playing_in", secs=3)
        self.lbl_record_hint.config(text=hint_text, fg=C_WARNING)

        def countdown(secs):
            if secs > 0:
                self.lbl_record_hint.config(text=t(self.current_lang, "hint_playing_in", secs=secs), fg=C_WARNING)
                self.after(1000, lambda: countdown(secs - 1))
            else:
                self.lbl_record_hint.config(text=t(self.current_lang, "hint_playing"), fg=C_SUCCESS)
                import threading
                def worker():
                    ok, msg = player.play(self.current_macro)
                    self.after(0, lambda: self._on_play_done(msg))
                threading.Thread(target=worker, daemon=True).start()

        self.after(1000, lambda: countdown(2))

    def _on_play_done(self, msg):
        self.btn_macro_play.config(
            state="normal",
            bg=C_BTN_BG,
            fg=C_FG_PRIMARY,
            activebackground=C_BTN_HOVER,
            activeforeground=C_FG_PRIMARY
        )
        self.lbl_record_hint.config(text=msg, fg=C_SUCCESS)

    # =========================================================================
    # HELP / USER MANUAL VIEW
    # =========================================================================
    def _build_help_view(self):
        # 1. Left Navigation / Topic Panel (x=56, y=15, w=210, h=494)
        self.help_left_panel = tk.Frame(self, bg=C_CARD_BG, bd=1, relief="solid", highlightbackground=C_CARD_BORDER)
        win_help_left = self.canvas.create_window(56, 15, window=self.help_left_panel, anchor="nw", width=210, height=494)
        self.help_canvas_items.append(win_help_left)

        # Header
        self.lbl_help_title = tk.Label(self.help_left_panel, text=t(self.current_lang, "help_title"), font=FONT_TITLE, fg=C_FG_PRIMARY, bg=C_CARD_BG)
        self.lbl_help_title.pack(pady=(12, 8), padx=6, fill="x")

        # Divider under header
        tk.Frame(self.help_left_panel, height=1, bg=C_CARD_BORDER).pack(fill="x", padx=8, pady=(0, 6))

        # Topic Navigation Container (fills height naturally with comfortable vertical padding)
        self.topic_nav_container = tk.Frame(self.help_left_panel, bg=C_CARD_BG)
        self.topic_nav_container.pack(fill="both", expand=True, padx=4, pady=4)
        self.topic_nav_rows = []

        # 2. Right Content Panel (x=280, y=15, w=530, h=494)
        self.help_right_panel = tk.Frame(self, bg=C_CARD_BG, bd=1, relief="solid", highlightbackground=C_CARD_BORDER)
        win_help_right = self.canvas.create_window(280, 15, window=self.help_right_panel, anchor="nw", width=530, height=494)
        self.help_canvas_items.append(win_help_right)

        # Top Header Row
        hdr_row = tk.Frame(self.help_right_panel, bg=C_CARD_BG, padx=16, pady=10)
        hdr_row.pack(side="top", fill="x")

        self.lbl_help_topic_title = tk.Label(hdr_row, text="", font=FONT_TITLE, fg=C_FG_PRIMARY, bg=C_CARD_BG, anchor="w")
        self.lbl_help_topic_title.pack(side="left")

        self.btn_help_next = tk.Button(
            hdr_row, text=t(self.current_lang, "btn_next"), font=FONT_SMALL_BOLD,
            bg=C_BTN_BG, fg=C_FG_PRIMARY, activebackground=C_BTN_HOVER, activeforeground=C_FG_PRIMARY,
            bd=0, relief="flat", highlightbackground=C_BTN_BORDER, highlightthickness=1, padx=10, pady=3,
            command=self._on_help_next
        )
        self.btn_help_next.pack(side="right")

        self.btn_help_prev = tk.Button(
            hdr_row, text=t(self.current_lang, "btn_prev"), font=FONT_SMALL_BOLD,
            bg=C_BTN_BG, fg=C_FG_PRIMARY, activebackground=C_BTN_HOVER, activeforeground=C_FG_PRIMARY,
            bd=0, relief="flat", highlightbackground=C_BTN_BORDER, highlightthickness=1, padx=10, pady=3,
            command=self._on_help_prev
        )
        self.btn_help_prev.pack(side="right", padx=(0, 6))

        # Divider under header
        tk.Frame(self.help_right_panel, height=1, bg=C_CARD_BORDER).pack(fill="x")

        # Scrollable Text Content Area
        text_container = tk.Frame(self.help_right_panel, bg=C_VIEW_BG)
        text_container.pack(fill="both", expand=True, padx=10, pady=10)

        self.help_text = tk.Text(
            text_container,
            bg=C_VIEW_BG,
            fg=C_FG_BODY,
            font=(FONT_FAMILY, 9),
            wrap="word",
            bd=0,
            padx=14,
            pady=12,
            highlightthickness=0,
            cursor="arrow",
            tabs=(148, "left")
        )
        text_scroll = DarkAutoScrollbar(text_container, command=self.help_text.yview)
        self.help_text.configure(yscrollcommand=text_scroll.set)
        text_scroll.pack(side="right", fill="y")
        self.help_text.pack(side="left", fill="both", expand=True)

        # Style tags for rich typography and precise margin/column alignment
        self.help_text.tag_configure("title", font=(FONT_FAMILY, 11, "bold"), foreground=C_FG_PRIMARY, spacing1=2, spacing3=10, lmargin1=16, lmargin2=16)
        self.help_text.tag_configure("h2", font=(FONT_FAMILY, 10, "bold"), foreground=C_ACCENT, spacing1=14, spacing3=6, lmargin1=16, lmargin2=16)
        self.help_text.tag_configure("body", font=(FONT_FAMILY, 9), foreground=C_FG_BODY, spacing1=3, spacing2=2, spacing3=5, lmargin1=16, lmargin2=16, rmargin=16)
        self.help_text.tag_configure("bold", font=(FONT_FAMILY, 9, "bold"), foreground=C_FG_PRIMARY)
        self.help_text.tag_configure("bullet", font=(FONT_FAMILY, 9), foreground=C_FG_BODY, spacing1=2, spacing2=2, spacing3=4, lmargin1=16, lmargin2=34, rmargin=16)
        self.help_text.tag_configure("bullet_dot", font=(FONT_FAMILY, 9, "bold"), foreground=C_ACCENT)
        self.help_text.tag_configure("key_row", font=(FONT_FAMILY, 9), foreground=C_FG_BODY, spacing1=4, spacing2=2, spacing3=5, lmargin1=16, lmargin2=148, rmargin=16)
        self.help_text.tag_configure("keycap", font=(FONT_FAMILY, 9, "bold"), foreground=C_SUCCESS, background="#1a2133")
        self.help_text.tag_configure("table_row", font=(FONT_FAMILY, 9), foreground=C_FG_BODY, spacing1=3, spacing2=2, spacing3=3, lmargin1=16, lmargin2=148, rmargin=16)
        self.help_text.tag_configure("table_lbl", font=(FONT_FAMILY, 9, "bold"), foreground=C_FG_PRIMARY)
        self.help_text.tag_configure("code", font=("Monospace", 9), foreground="#61afef", background="#161924", spacing1=4, spacing2=2, spacing3=4, lmargin1=24, lmargin2=24, rmargin=24)
        self.help_text.tag_configure("tip", font=(FONT_FAMILY, 9, "italic"), foreground=C_WARNING, background="#222638", spacing1=6, spacing2=2, spacing3=6, lmargin1=20, lmargin2=20, rmargin=20)
        self.help_text.tag_configure("center", justify="center")

        # Initial populate
        self._populate_help_topics()
        self._render_help_topic(0)

    def _populate_help_topics(self):
        for w in self.topic_nav_container.winfo_children():
            w.destroy()
        self.topic_nav_rows = []
        topics = get_topic_titles(self.current_lang)

        for idx, (icon, title) in enumerate(topics):
            row_frame = tk.Frame(self.topic_nav_container, bg=C_CARD_BG, cursor="hand2")
            row_frame.pack(fill="x", padx=4, pady=2)

            bar = tk.Frame(row_frame, width=3, bg=C_CARD_BG)
            bar.pack(side="left", fill="y")

            lbl_ic = tk.Label(row_frame, text=icon, font=(FONT_FAMILY, 10), bg=C_CARD_BG, fg=C_FG_BODY, width=2)
            lbl_ic.pack(side="left", padx=(4, 2), pady=6)

            lbl_tl = tk.Label(row_frame, text=title, font=FONT_SMALL, bg=C_CARD_BG, fg=C_FG_BODY, anchor="w", wraplength=150, justify="left")
            lbl_tl.pack(side="left", fill="x", expand=True, padx=(2, 6), pady=5)

            self.topic_nav_rows.append((row_frame, bar, lbl_ic, lbl_tl))

            def make_click(i):
                return lambda e: self._render_help_topic(i)
            def make_enter(rf, i):
                return lambda e: rf.config(bg="#32364a") if self.current_help_topic != i else None
            def make_leave(rf, i):
                return lambda e: rf.config(bg=C_CARD_BG) if self.current_help_topic != i else None

            for widget in (row_frame, lbl_ic, lbl_tl):
                widget.bind("<Button-1>", make_click(idx))
                widget.bind("<Enter>", make_enter(row_frame, idx))
                widget.bind("<Leave>", make_leave(row_frame, idx))

        self._update_topic_nav_styles()

    def _update_topic_nav_styles(self):
        for i, (row_frame, bar, lbl_ic, lbl_tl) in enumerate(self.topic_nav_rows):
            if i == self.current_help_topic:
                row_frame.config(bg="#1c2842")
                bar.config(bg=C_ACCENT)
                lbl_ic.config(bg="#1c2842", fg="#ffffff")
                lbl_tl.config(bg="#1c2842", fg="#ffffff", font=FONT_SMALL_BOLD)
            else:
                row_frame.config(bg=C_CARD_BG)
                bar.config(bg=C_CARD_BG)
                lbl_ic.config(bg=C_CARD_BG, fg=C_FG_BODY)
                lbl_tl.config(bg=C_CARD_BG, fg=C_FG_BODY, font=FONT_SMALL)

    def _render_help_topic(self, topic_idx: int):
        self.current_help_topic = topic_idx
        self._update_topic_nav_styles()

        topics = get_topic_titles(self.current_lang)
        if 0 <= topic_idx < len(topics):
            icon, title = topics[topic_idx]
            self.lbl_help_topic_title.config(text=f"{icon}  {title}")

        # Enable text editing to insert content
        self.help_text.config(state="normal")
        self.help_text.delete("1.0", "end")

        # In Overview topic, display the keyboard image at the top
        if topic_idx == 0 and hasattr(self, "img_kb_overview") and self.img_kb_overview:
            self.help_text.insert("end", "\n", "center")
            self.help_text.image_create("end", image=self.img_kb_overview)
            self.help_text.insert("end", "\n\n", "center")

        blocks = get_topic_content(topic_idx, self.current_lang)
        for block in blocks:
            btype = block[0]
            if btype == "title":
                pass  # Topic title already cleanly displayed in header bar
            elif btype == "h2":
                self.help_text.insert("end", block[1] + "\n", "h2")
            elif btype == "body":
                self.help_text.insert("end", block[1] + "\n", "body")
            elif btype == "bullet":
                self.help_text.insert("end", "• ", ("bullet", "bullet_dot"))
                self.help_text.insert("end", block[1] + "\n", "bullet")
            elif btype == "key":
                self.help_text.insert("end", f" [ {block[1]} ] ", ("key_row", "keycap"))
                self.help_text.insert("end", f"\t{block[2]}\n", "key_row")
            elif btype == "table_row":
                self.help_text.insert("end", f"{block[1]}\t", ("table_row", "table_lbl"))
                self.help_text.insert("end", f"{block[2]}\n", "table_row")
            elif btype == "code":
                self.help_text.insert("end", f"   {block[1]}\n", "code")
            elif btype == "tip":
                self.help_text.insert("end", f" 💡 Tip: {block[1]}\n", "tip")

        # Disable editing so it is strictly read-only
        self.help_text.config(state="disabled")
        self.help_text.yview_moveto(0.0)

    def _on_help_prev(self):
        if self.current_help_topic > 0:
            self._render_help_topic(self.current_help_topic - 1)

    def _on_help_next(self):
        topics = get_topic_titles(self.current_lang)
        if self.current_help_topic < len(topics) - 1:
            self._render_help_topic(self.current_help_topic + 1)

    def _on_language_selected(self, event=None):
        selected_name = self.combo_lang.get()
        code = LANG_CODE_BY_NAME.get(selected_name, "en")
        if code != self.current_lang:
            self.current_lang = code
            self.config_mgr.settings.language = code
            self.config_mgr.save()
            self._apply_language()

    def _on_autorun_toggled(self):
        val = self.var_autorun.get()
        self.config_mgr.set_autostart(val)

    def _confirm_dialog(self, title: str, message: str) -> bool:
        """Displays a dark-themed confirmation modal dialog with regular, non-bold text."""
        result = [False]
        dlg = tk.Toplevel(self)
        dlg.title(title)
        dlg.configure(bg=C_CARD_BG)
        dlg.resizable(False, False)
        dlg.transient(self)

        content = tk.Frame(dlg, bg=C_CARD_BG, padx=35, pady=25)
        content.pack(fill="both", expand=True)

        lbl = tk.Label(
            content,
            text=message,
            font=FONT_REGULAR,
            fg=C_FG_BODY,
            bg=C_CARD_BG,
            justify="center",
            wraplength=400
        )
        lbl.pack(pady=(0, 22))

        btn_box = tk.Frame(content, bg=C_CARD_BG)
        btn_box.pack()

        def on_yes():
            result[0] = True
            dlg.destroy()

        def on_no():
            result[0] = False
            dlg.destroy()

        btn_yes = tk.Button(
            btn_box,
            text=t(self.current_lang, "btn_yes"),
            font=FONT_REGULAR,
            bg=C_DESTRUCTIVE,
            fg=C_ACCENT_FG,
            activebackground=C_DESTRUCTIVE_HOVER,
            activeforeground=C_ACCENT_FG,
            bd=0,
            relief="flat",
            padx=22,
            pady=6,
            command=on_yes
        )
        btn_yes.pack(side="left", padx=10)

        btn_no = tk.Button(
            btn_box,
            text=t(self.current_lang, "btn_no"),
            font=FONT_REGULAR,
            bg=C_BTN_BG,
            fg=C_FG_PRIMARY,
            activebackground=C_BTN_HOVER,
            activeforeground=C_FG_PRIMARY,
            highlightbackground=C_BTN_BORDER,
            highlightcolor=C_ACCENT,
            highlightthickness=1,
            bd=0,
            relief="flat",
            padx=22,
            pady=6,
            command=on_no
        )
        btn_no.pack(side="left", padx=10)

        # Center over parent window
        dlg.update_idletasks()
        w = dlg.winfo_reqwidth()
        h = dlg.winfo_reqheight()
        px = self.winfo_x() + max(0, (self.winfo_width() - w) // 2)
        py = self.winfo_y() + max(0, (self.winfo_height() - h) // 2)
        dlg.geometry(f"+{px}+{py}")

        dlg.bind("<Escape>", lambda e: on_no())
        btn_no.focus_set()

        dlg.grab_set()
        self.wait_window(dlg)
        return result[0]

    def _on_factory_reset(self):
        title = t(self.current_lang, "msg_factory_reset_title")
        prompt = t(self.current_lang, "msg_factory_reset_confirm")
        if self._confirm_dialog(title, prompt):
            # 1. Reset macros: delete all macros and clear table
            self.macro_mgr.clear_all()
            self.current_macro = None
            self._refresh_macro_list()

            # 2. Reset config defaults (Mode 1 Steady, English, AutoRun off)
            self.config_mgr.restore_factory_defaults()
            self.current_lang = "en"
            self.combo_lang.set(LANGUAGES["en"])
            self.var_autorun.set(False)

            # 3. Reset lighting to Mode 1 (Steady), Brightness 4, Speed 4
            self.mode_var.set(1)
            matched = [m for m in LIGHT_MODES if m.id == 1]
            self.current_mode = matched[0] if matched else LIGHT_MODES[1]
            self.brightness = 4
            self.speed = 4
            self.slider_brightness.set(self.brightness)
            self.slider_speed.set(self.speed)
            self._send_lighting()
            self._update_controls_for_mode(self.current_mode)

            # 4. Refresh language across all UI elements
            self._apply_language()
            messagebox.showinfo(
                t(self.current_lang, "msg_factory_reset_title"),
                t(self.current_lang, "msg_factory_reset_text")
            )

    def _apply_language(self):
        """Dynamically re-translates all UI widget texts in place."""
        # 1. Window
        self.title(t(self.current_lang, "app_title"))

        # 2. Light View
        self.lbl_light_modes_title.config(text=t(self.current_lang, "lighting_modes"))
        for m, rb, _ in self.mode_radio_buttons:
            rb.config(text=get_mode_name(self.current_lang, m.id, m.name))
        if hasattr(self, "lbl_params_title"):
            self.lbl_params_title.config(text=t(self.current_lang, "lighting_modes"))
        self.lbl_b_title.config(text=t(self.current_lang, "light_brightness"))
        self.lbl_s_title.config(text=t(self.current_lang, "light_speed"))
        self.lbl_m_title.config(text=t(self.current_lang, "music_title"))
        self.lbl_pattern_title.config(text=t(self.current_lang, "music_pattern"))
        self.lbl_freq_title.config(text=t(self.current_lang, "music_freq"))

        pattern_names = [t(self.current_lang, "music_mode1"), t(self.current_lang, "music_mode2")]
        self.combo_music_pattern.config(values=pattern_names)
        self.combo_music_pattern.current(self.music_submode - 1 if self.music_submode in (1, 2) else 0)

        if self.is_visualizer_active:
            self.btn_music_toggle.config(text=t(self.current_lang, "music_stop"))
            self.lbl_music_status.config(text=t(self.current_lang, "music_streaming", delay=self.music_delay))
        else:
            self.btn_music_toggle.config(text=t(self.current_lang, "music_start"))
            self.lbl_music_status.config(text=t(self.current_lang, "music_idle"))

        # 3. Macro View
        self.lbl_macro_list_title.config(text=t(self.current_lang, "macro_list"))
        self.btn_macro_new.config(text=t(self.current_lang, "btn_new"))
        self.btn_macro_del.config(text=t(self.current_lang, "btn_delete"))
        self.btn_macro_copy.config(text=t(self.current_lang, "btn_copy"))
        self.btn_macro_import.config(text=t(self.current_lang, "btn_import"))
        self.btn_macro_export.config(text=t(self.current_lang, "btn_export"))
        self.btn_macro_rename.config(text=t(self.current_lang, "btn_rename"))
        self.lbl_rep.config(text=t(self.current_lang, "repeat_time"))
        if hasattr(self, "lbl_delay_title"):
            self.lbl_delay_title.config(text=t(self.current_lang, "delay_mode"))
        self.rb_record.config(text=t(self.current_lang, "delay_record"))
        self.rb_nodelay.config(text=t(self.current_lang, "delay_none"))
        self.rb_default.config(text=t(self.current_lang, "delay_default"))
        self.lbl_ms.config(text=t(self.current_lang, "delay_ms"))
        self.lbl_rec_title.config(text=t(self.current_lang, "macro_record"))

        self.macro_tree.heading("desc", text=t(self.current_lang, "col_desc"))
        self.macro_tree.heading("action", text=t(self.current_lang, "col_action"))
        self.macro_tree.heading("delay", text=t(self.current_lang, "col_delay"))
        self._refresh_macro_table()

        if self.is_recording:
            self.btn_macro_record.config(text=t(self.current_lang, "btn_stop"))
        else:
            self.btn_macro_record.config(text=t(self.current_lang, "btn_record"))
        self.btn_macro_play.config(text=t(self.current_lang, "btn_play"))
        self.btn_macro_save.config(text=t(self.current_lang, "btn_save"))

        # 4. Left Info Panel (Controls and info moved from Home)
        if hasattr(self, "lbl_conn"):
            self.lbl_conn.config(text=t(self.current_lang, "dev_connected"))
        if hasattr(self, "lbl_lang"):
            self.lbl_lang.config(text=t(self.current_lang, "language"))
        if hasattr(self, "cb_autorun"):
            self.cb_autorun.config(text=t(self.current_lang, "auto_run"))
        if hasattr(self, "btn_restore"):
            self.btn_restore.config(text=t(self.current_lang, "restore_factory"))
        if hasattr(self, "lbl_ver"):
            self.lbl_ver.config(text="Ver: 1.0.3.1")

        # 5. Status Bar & Hardware
        self.btn_fix_udev.config(text=t(self.current_lang, "btn_fix_udev"))
        self._update_hardware_ui(self.driver.state)

        # 6. Help View
        if hasattr(self, "lbl_help_title"):
            self.lbl_help_title.config(text=t(self.current_lang, "help_title"))
        if hasattr(self, "btn_help_prev"):
            self.btn_help_prev.config(text=t(self.current_lang, "btn_prev"))
        if hasattr(self, "btn_help_next"):
            self.btn_help_next.config(text=t(self.current_lang, "btn_next"))
        if hasattr(self, "topic_nav_container"):
            self._populate_help_topics()
            self._render_help_topic(self.current_help_topic)

    # =========================================================================
    # HARDWARE POLLER & UI STATE
    # =========================================================================
    def _update_hardware_ui(self, state: str):
        if state == DeviceState.CONNECTED:
            self.status_canvas.itemconfig(self.status_circle, fill=C_SUCCESS)
            self.lbl_status.config(text=t(self.current_lang, "status_ready_title"))
            node = self.driver.device_node or "Interface 1"
            self.lbl_status_node.config(text=node, fg=C_FG_MUTED)
            self.btn_fix_udev.pack_forget()
            if hasattr(self, "lbl_conn") and hasattr(self, "dev_badge"):
                self.lbl_conn.config(text=t(self.current_lang, "dev_connected"), fg=C_FG_PRIMARY)
                self.dev_badge.config(
                    text=t(self.current_lang, "dev_badge_connected"),
                    fg=C_SUCCESS, bg=C_VIEW_BG, highlightbackground=C_SUCCESS
                )
                if hasattr(self, "lbl_conn_detail"):
                    self.lbl_conn_detail.config(text=f"VID: 0x1A2C  PID: 0x7C80\nInterface 1 ({node})", fg=C_FG_MUTED)
        elif state == DeviceState.PERMISSION_DENIED:
            self.status_canvas.itemconfig(self.status_circle, fill=C_WARNING)
            self.lbl_status.config(text=t(self.current_lang, "status_perm_title"))
            node = self.driver.device_node or "HID Device"
            self.lbl_status_node.config(text=node, fg=C_WARNING)
            self.btn_fix_udev.pack(anchor="w", padx=(18, 0), pady=(4, 0))
            if hasattr(self, "lbl_conn") and hasattr(self, "dev_badge"):
                self.lbl_conn.config(text=t(self.current_lang, "dev_perm_required"), fg=C_WARNING)
                self.dev_badge.config(
                    text=t(self.current_lang, "dev_badge_perm"),
                    fg=C_WARNING, bg=C_VIEW_BG, highlightbackground=C_WARNING
                )
                if hasattr(self, "lbl_conn_detail"):
                    self.lbl_conn_detail.config(text=t(self.current_lang, "detail_perm_required"), fg=C_WARNING)
        elif state == DeviceState.CLAIMED_BY_VM:
            self.status_canvas.itemconfig(self.status_circle, fill=C_WARNING)
            self.lbl_status.config(text=t(self.current_lang, "status_vm_title"))
            self.lbl_status_node.config(text=t(self.current_lang, "status_vm_sub"), fg=C_WARNING)
            self.btn_fix_udev.pack_forget()
            if hasattr(self, "lbl_conn") and hasattr(self, "dev_badge"):
                self.lbl_conn.config(text=t(self.current_lang, "dev_captured_vm"), fg=C_WARNING)
                self.dev_badge.config(
                    text=t(self.current_lang, "dev_badge_vm"),
                    fg=C_WARNING, bg=C_VIEW_BG, highlightbackground=C_WARNING
                )
                if hasattr(self, "lbl_conn_detail"):
                    self.lbl_conn_detail.config(text=t(self.current_lang, "detail_claimed_vm"), fg=C_WARNING)
        else:
            self.status_canvas.itemconfig(self.status_circle, fill=C_DESTRUCTIVE)
            self.lbl_status.config(text=t(self.current_lang, "status_not_detected_title"))
            self.lbl_status_node.config(text=t(self.current_lang, "status_connect_usb"), fg=C_FG_DISABLED)
            self.btn_fix_udev.pack_forget()
            if hasattr(self, "lbl_conn") and hasattr(self, "dev_badge"):
                self.lbl_conn.config(text=t(self.current_lang, "dev_disconnected"), fg=C_FG_MUTED)
                self.dev_badge.config(
                    text=t(self.current_lang, "dev_badge_disconnected"),
                    fg=C_FG_MUTED, bg=C_VIEW_BG, highlightbackground=C_CARD_BORDER
                )
                if hasattr(self, "lbl_conn_detail"):
                    self.lbl_conn_detail.config(text=t(self.current_lang, "detail_connect_usb"), fg=C_FG_DISABLED)

    def _poll_hardware(self):
        state, detail = self.driver.probe()
        self._update_hardware_ui(state)

        # Re-check every 2 seconds
        self._poll_job = self.after(2000, self._poll_hardware)

    # Lighting callbacks
    def _on_mode_selected(self):
        mode_id = self.mode_var.get()
        matched = [m for m in LIGHT_MODES if m.id == mode_id]
        if not matched:
            return
        mode = matched[0]
        self.current_mode = mode

        # Persist mode selection
        self.config_mgr.settings.mode_id = mode_id
        self.config_mgr.save()

        self._update_controls_for_mode(mode)
        self._send_lighting()

    def _update_controls_for_mode(self, mode: LightMode):
        if mode.is_music:
            self.std_controls.pack_forget()
            self.music_controls.pack(fill="both", expand=True, pady=(2, 4))
        else:
            if self.is_visualizer_active:
                self._stop_visualizer()
            self.music_controls.pack_forget()
            self.std_controls.pack(fill="both", expand=True, pady=(2, 4))

            if mode.has_brightness:
                self.slider_brightness.state(["!disabled"])
                self.lbl_b_title.config(fg=C_FG_PRIMARY)
            else:
                self.slider_brightness.state(["disabled"])
                self.lbl_b_title.config(fg=C_FG_DISABLED)

            if mode.has_speed:
                self.slider_speed.state(["!disabled"])
                self.lbl_s_title.config(fg=C_FG_PRIMARY)
            else:
                self.slider_speed.state(["disabled"])
                self.lbl_s_title.config(fg=C_FG_DISABLED)

    def _on_brightness_change(self, val):
        self.brightness = int(round(float(val)))
        self.config_mgr.settings.brightness = self.brightness
        self.config_mgr.save()
        self._send_lighting()

    def _on_speed_change(self, val):
        self.speed = int(round(float(val)))
        self.config_mgr.settings.speed = self.speed
        self.config_mgr.save()
        self._send_lighting()

    def _on_music_submode_change(self, event=None):
        selected = self.combo_music_pattern.get()
        if "2" in selected:
            self.music_submode = 2
        else:
            self.music_submode = 1
        self.config_mgr.settings.music_submode = self.music_submode
        self.config_mgr.save()
        if self.is_visualizer_active:
            self.music_engine.set_parameters(submode=self.music_submode)

    def _on_music_delay_change(self, event=None):
        selected = self.combo_music_freq.get()
        if "33ms" in selected:
            self.music_delay = 33
        elif "66ms" in selected:
            self.music_delay = 66
        elif "100ms" in selected:
            self.music_delay = 100
        self.config_mgr.settings.music_delay = self.music_delay
        self.config_mgr.save()
        if self.is_visualizer_active:
            self.music_engine.set_parameters(delay_ms=self.music_delay)

    def _toggle_visualizer(self):
        if self.is_visualizer_active:
            self._stop_visualizer()
        else:
            self._start_visualizer()

    def _start_visualizer(self):
        state, detail = self.driver.probe()
        if state != DeviceState.CONNECTED:
            messagebox.showwarning(
                t(self.current_lang, "device_not_ready"),
                t(self.current_lang, "cannot_stream", detail=detail)
            )
            return

        self.is_visualizer_active = True
        self.btn_music_toggle.config(text=t(self.current_lang, "music_stop"), bg=C_DESTRUCTIVE, activebackground=C_DESTRUCTIVE_HOVER)
        self.lbl_music_status.config(text=t(self.current_lang, "music_streaming", delay=self.music_delay), fg=C_SUCCESS)
        self.music_engine.start(submode=self.music_submode, delay_ms=self.music_delay)

    def _stop_visualizer(self):
        self.is_visualizer_active = False
        self.music_engine.stop()
        self.btn_music_toggle.config(text=t(self.current_lang, "music_start"), bg=C_ACCENT, activebackground=C_ACCENT_HOVER)
        self.lbl_music_status.config(text=t(self.current_lang, "music_idle"), fg=C_FG_MUTED)

    def _send_lighting(self):
        if self.current_mode.is_music:
            return

        state, _ = self.driver.probe()
        if state == DeviceState.CONNECTED:
            self.driver.set_lighting(self.current_mode, brightness=self.brightness, speed=self.speed)

    def _check_startup_permissions(self):
        has_access, reason = check_access()
        if not has_access:
            prompt_password_dialog(self, on_success=self._on_permission_fixed)

    def _on_permission_fixed(self):
        self.driver.probe()
        self._update_hardware_ui(self.driver.state)

    def _run_udev_fix(self):
        prompt_password_dialog(self, on_success=self._on_permission_fixed)

    def destroy(self):
        if hasattr(self, "_poll_job") and self._poll_job:
            try:
                self.after_cancel(self._poll_job)
            except Exception:
                pass
            self._poll_job = None
        if self.is_recording:
            self._toggle_record()
        if self.is_visualizer_active:
            self.music_engine.stop()
        self.driver.close()
        super().destroy()

def main():
    app = FreeWolfK8App()
    app.mainloop()

if __name__ == "__main__":
    main()
