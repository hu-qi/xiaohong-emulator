# Architecture — M0

## Scope

M0 is a deterministic mock and **not** WS63 hardware emulation.

- `xh-core`: hardware-independent device event semantics and RGB565 framebuffer memory.
- `xh-cli`: smoke demonstration of mock behavior.
- Future `xh-hardware`: physical board bridge, independently tested.
- Future `xh-cpu-riscv` / `xh-soc-ws63`: research-only firmware execution backend.
- Future Web Studio: rendering and input over a stable device protocol.

## Key events

Inputs have monotonic virtual timestamps. Invalid transitions are rejected. A long-press event is emitted on release if its duration meets the configurable threshold. This is **not** yet physical key debouncing, simultaneous-key choreography, or GPIO-level emulation.

## Device correctness

Do not conflate simulated app behavior with actual firmware execution. Every surface must display the current backend, with test evidence for hardware fidelity.

## Immediate next steps

1. Obtain and document WS63 board schematics, boot logs, linker script, and peripheral mapping.
2. Add framebuffer write operations, dirty-rect updates, and snapshot tests.
3. Define versioned device protocol for Web Studio and real-device bridge.
4. Implement key combinations and edge-case tests, informed by measured hardware traces.
