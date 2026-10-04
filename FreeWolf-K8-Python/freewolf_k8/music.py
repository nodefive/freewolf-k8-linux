"""
Music Mode / Audio Visualizer Engine for FREE WOLF K8
"""
import time
import math
import random
import threading
from typing import Optional, Callable
from .driver import FreeWolfK8Driver

class MusicVisualizerEngine:
    def __init__(self, driver: FreeWolfK8Driver):
        self.driver = driver
        self.running = False
        self.thread: Optional[threading.Thread] = None
        self.submode = 2  # 1 or 2
        self.delay_ms = 66  # 33ms, 66ms, 100ms
        self.on_frame: Optional[Callable[[bytes], None]] = None

    def start(self, submode: int = 2, delay_ms: int = 66):
        """Starts streaming music frames at the specified frequency rate."""
        if self.running:
            self.stop()

        self.submode = submode
        self.delay_ms = delay_ms
        self.running = True

        # Mode wire ID for Music is 0x13; send to initialize visualizer state
        from .protocol import LightMode
        music_mode = LightMode(20, 0x13, "Music", 512, "Music mode")
        self.driver.set_lighting(music_mode, brightness=4, speed=4)

        self.thread = threading.Thread(target=self._worker, daemon=True)
        self.thread.start()

    def set_parameters(self, submode: Optional[int] = None, delay_ms: Optional[int] = None):
        if submode is not None:
            self.submode = submode
        if delay_ms is not None:
            self.delay_ms = delay_ms

    def stop(self):
        """Stops the streaming thread."""
        self.running = False
        if self.thread and self.thread.is_alive():
            self.thread.join(timeout=1.0)
        self.thread = None

    def _worker(self):
        """
        Continuously generates and streams 4-band audio spectrum levels.
        """
        phase = 0.0
        while self.running:
            t0 = time.time()
            phase += 0.15

            # Synthesize 4 dynamic frequency band energy levels (Bass, Low-Mid, Mid-High, High)
            b0 = int(128 + 120 * math.sin(phase * 1.3))
            b1 = int(128 + 120 * math.sin(phase * 2.1 + 1.0))
            b2 = int(128 + 120 * math.sin(phase * 3.4 + 2.0))
            b3 = int(128 + 120 * math.cos(phase * 0.9))

            eq_data = bytes([
                max(10, min(255, b0)),
                max(10, min(255, b1)),
                max(10, min(255, b2)),
                max(10, min(255, b3))
            ])

            self.driver.stream_music_packet(self.submode, eq_data)

            if self.on_frame:
                try:
                    self.on_frame(eq_data)
                except Exception:
                    pass

            # Maintain exact requested delay interval (e.g. 33ms or 66ms)
            elapsed = (time.time() - t0) * 1000.0
            sleep_time = max(0.001, (self.delay_ms - elapsed) / 1000.0)
            time.sleep(sleep_time)
