# Roadmap / TODO

## M0 — Repository foundation
- [x] Initial mission README
- [x] Rust workspace and mock CLI (PR pending)
- [x] Deterministic key state and RGB565 allocation (PR pending)
- [x] CI checks (PR pending)
- [ ] Verify CI green; merge first PR

## M1 — Usable virtual device
- [ ] Web Studio with real 240×240 framebuffer rendering
- [ ] Key press/release/long press/combination and debouncing policy
- [ ] Snapshot testing and event recording/replay
- [ ] Versioned event and rendering protocol

## M2 — Developer tooling
- [ ] UART transport and structured logs
- [ ] Hardware bridge to WS63 board; capture real behavior
- [ ] Audio PCM/WAV injection and capture fixtures
- [ ] Wi-Fi and protocol mock with explicit compatibility boundaries

## M3 — WS63 feasibility gate
- [ ] Document silicon ISA/extensions and firmware image types
- [ ] Determine available ROM, BSP, linker and MMIO evidence
- [ ] Run bare-metal RISC-V test in experimental backend
- [ ] Establish success/failure criteria for attempting LiteOS-M boot
