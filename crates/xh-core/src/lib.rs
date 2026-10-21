//! Hardware-independent, deterministic Xiaohong device-state prototype.
//! No WS63 firmware execution occurs in this crate.

pub const DISPLAY_WIDTH: usize = 240;
pub const DISPLAY_HEIGHT: usize = 240;
pub const FRAME_BYTES_RGB565: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT * 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Minus,
    Menu,
    Plus,
}

impl Key {
    const fn index(self) -> usize {
        match self {
            Self::Minus => 0,
            Self::Menu => 1,
            Self::Plus => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEvent {
    Press { key: Key, at_ms: u64 },
    Release { key: Key, at_ms: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceEvent {
    KeyDown(Key),
    KeyUp { key: Key, held_ms: u64 },
    LongPress { key: Key, held_ms: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceError {
    NonMonotonicTime,
    AlreadyPressed(Key),
    NotPressed(Key),
}

#[derive(Debug, Clone)]
pub struct VirtualDevice {
    pressed_at: [Option<u64>; 3],
    last_time_ms: u64,
    long_press_threshold_ms: u64,
    framebuffer: Vec<u8>,
}

impl Default for VirtualDevice {
    fn default() -> Self {
        Self::new(800)
    }
}

impl VirtualDevice {
    pub fn new(long_press_threshold_ms: u64) -> Self {
        Self {
            pressed_at: [None; 3],
            last_time_ms: 0,
            long_press_threshold_ms,
            framebuffer: vec![0; FRAME_BYTES_RGB565],
        }
    }

    pub fn framebuffer(&self) -> &[u8] {
        &self.framebuffer
    }

    pub fn handle_key(&mut self, event: KeyEvent) -> Result<Vec<DeviceEvent>, DeviceError> {
        let timestamp = match event {
            KeyEvent::Press { at_ms, .. } | KeyEvent::Release { at_ms, .. } => at_ms,
        };
        if timestamp < self.last_time_ms {
            return Err(DeviceError::NonMonotonicTime);
        }
        match event {
            KeyEvent::Press { key, at_ms } => {
                if self.pressed_at[key.index()].is_some() {
                    return Err(DeviceError::AlreadyPressed(key));
                }
                self.pressed_at[key.index()] = Some(at_ms);
                self.last_time_ms = at_ms;
                Ok(vec![DeviceEvent::KeyDown(key)])
            }
            KeyEvent::Release { key, at_ms } => {
                let start = self.pressed_at[key.index()].ok_or(DeviceError::NotPressed(key))?;
                let held_ms = at_ms - start;
                self.pressed_at[key.index()] = None;
                self.last_time_ms = at_ms;
                let mut emitted = vec![DeviceEvent::KeyUp { key, held_ms }];
                if held_ms >= self.long_press_threshold_ms {
                    emitted.push(DeviceEvent::LongPress { key, held_ms });
                }
                Ok(emitted)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn framebuffer_has_expected_dimensions() {
        assert_eq!(VirtualDevice::default().framebuffer().len(), 240 * 240 * 2);
    }

    #[test]
    fn long_press_is_emitted_only_at_threshold() {
        let mut device = VirtualDevice::new(800);
        assert_eq!(
            device.handle_key(KeyEvent::Press { key: Key::Menu, at_ms: 100 }),
            Ok(vec![DeviceEvent::KeyDown(Key::Menu)])
        );
        assert_eq!(
            device.handle_key(KeyEvent::Release { key: Key::Menu, at_ms: 900 }),
            Ok(vec![
                DeviceEvent::KeyUp { key: Key::Menu, held_ms: 800 },
                DeviceEvent::LongPress { key: Key::Menu, held_ms: 800 },
            ])
        );
    }

    #[test]
    fn rejects_duplicate_press_without_corrupting_state() {
        let mut device = VirtualDevice::default();
        device.handle_key(KeyEvent::Press { key: Key::Plus, at_ms: 10 }).unwrap();
        assert_eq!(
            device.handle_key(KeyEvent::Press { key: Key::Plus, at_ms: 20 }),
            Err(DeviceError::AlreadyPressed(Key::Plus))
        );
        assert_eq!(
            device.handle_key(KeyEvent::Release { key: Key::Plus, at_ms: 40 }).unwrap(),
            vec![DeviceEvent::KeyUp { key: Key::Plus, held_ms: 30 }]
        );
    }

    #[test]
    fn rejects_non_monotonic_timestamp() {
        let mut device = VirtualDevice::default();
        device.handle_key(KeyEvent::Press { key: Key::Minus, at_ms: 100 }).unwrap();
        assert_eq!(
            device.handle_key(KeyEvent::Release { key: Key::Minus, at_ms: 99 }),
            Err(DeviceError::NonMonotonicTime)
        );
    }
}
