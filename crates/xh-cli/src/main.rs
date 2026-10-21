use xh_core::{Key, KeyEvent, VirtualDevice, DISPLAY_HEIGHT, DISPLAY_WIDTH};

fn main() {
    let mut device = VirtualDevice::default();
    println!(
        "xiaohong-emulator prototype: {}x{} RGB565 framebuffer (mock mode)",
        DISPLAY_WIDTH, DISPLAY_HEIGHT
    );
    println!(
        "press: {:?}",
        device.handle_key(KeyEvent::Press { key: Key::Menu, at_ms: 100 })
    );
    println!(
        "release: {:?}",
        device.handle_key(KeyEvent::Release { key: Key::Menu, at_ms: 1000 })
    );
}
