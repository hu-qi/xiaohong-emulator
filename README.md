# xiaohong-emulator

An open-source virtual development and automated testing platform for Xiaohong AI devices.

> **Status: bootstrap / research prototype.** This project does **not** currently execute WS63 firmware. All simulated device behavior must be explicitly labelled as mock until validated against hardware.

## Mission

Create a developer-friendly virtual Xiaohong device with a 240×240 framebuffer, reproducible button events, serial logs, device-protocol tests, and eventually a research-grade WS63 execution backend.

## Architecture principles

1. Keep **mock**, **hardware bridge**, and **firmware execution** backends distinct.
2. Ship an observable, testable vertical slice before working on WS63 instruction emulation.
3. Never claim timing, peripherals, radio, or firmware compatibility without actual evidence.
4. Work through PRs with CI, hardware observations, and repeatable reproduction steps.
5. Use Rust for the core and a Web frontend for the studio.

## Roadmap

- **M0:** repository bootstrap, Rust workspace, device state machine, tests and CI.
- **M1:** 240×240 framebuffer, key press/release/long-press semantics, interactive Web Studio.
- **M2:** UART/hardware bridge, audio and network mock, real-device trace comparison.
- **M3:** WS63 boot/BSP research and limited RISC-V firmware execution experiment.

## Development

After merging the M0 pull request:\n\n```sh\ncargo fmt --all -- --check\ncargo clippy --workspace --all-targets -- -D warnings\ncargo test --workspace\ncargo run -p xh-cli\n```\n\nSee `docs/ARCHITECTURE.md` and `docs/TODO.md` for technical scope and next steps.

## Related work

- [Espressif esp-emulator](https://github.com/espressif/esp-emulator) (inspiration; its emulator core source is not in the distribution repository).
- [OpenHarmony](https://www.openharmony.cn/)

## License

License selection pending; do not import third-party code without checking its license.
