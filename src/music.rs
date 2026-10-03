//! Music Mode / Audio Visualizer Engine for FREE WOLF K8

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use crate::driver::FreeWolfK8Driver;
use crate::protocol::LightMode;

pub struct MusicVisualizerEngine {
    running: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl MusicVisualizerEngine {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn start(&mut self, submode: u8, delay_ms: u64) {
        if self.is_running() {
            self.stop();
        }

        let probe = FreeWolfK8Driver::probe();
        let node = match probe.node {
            Some(n) => n,
            None => return,
        };

        // First initialize Music mode (ID 20, wire_id 0x13)
        let music_mode = LightMode {
            id: 20,
            wire_id: 0x13,
            name: "Music",
            attribute: 512,
            description: "Music Mode",
        };
        let _ = FreeWolfK8Driver::set_lighting(&node, &music_mode, 4, 4);

        self.running.store(true, Ordering::SeqCst);
        let running_flag = self.running.clone();

        self.handle = Some(thread::spawn(move || {
            let mut phase = 0.0f32;

            while running_flag.load(Ordering::SeqCst) {
                let t0 = Instant::now();
                phase += 0.15;

                // Synthesize 4 dynamic frequency band energy levels
                let b0 = (128.0 + 120.0 * (phase * 1.3).sin()) as u8;
                let b1 = (128.0 + 120.0 * (phase * 2.1 + 1.0).sin()) as u8;
                let b2 = (128.0 + 120.0 * (phase * 3.4 + 2.0).sin()) as u8;
                let b3 = (128.0 + 120.0 * (phase * 0.9).cos()) as u8;

                let eq_data = [
                    b0.clamp(10, 255),
                    b1.clamp(10, 255),
                    b2.clamp(10, 255),
                    b3.clamp(10, 255),
                ];

                let _ = FreeWolfK8Driver::stream_music(&node, submode, &eq_data);

                let elapsed = t0.elapsed();
                let target = Duration::from_millis(delay_ms.max(10));
                if target > elapsed {
                    thread::sleep(target - elapsed);
                }
            }
        }));
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

impl Drop for MusicVisualizerEngine {
    fn drop(&mut self) {
        self.stop();
    }
}
