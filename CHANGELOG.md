# Changelog

All notable changes to this project are documented here. The format is based
on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2026-05-13

First public release.

### Added
- Real-time GPU metrics polled every 2s: temperature, core clock, GPU busy %,
  and VRAM used/total.
- Colour-coded temperature zones (green < 80 °C, yellow 80–88 °C, red > 88 °C).
- Rolling 60-second sparkline history for each metric.
- One-click Power Mode switching (HIGH / AUTO) via
  `power_dpm_force_performance_level`.
- One-click Runtime PM switching (ON / AUTO) via the PCI `power/control` node.
- AI Session buttons that toggle HIGH + PM ON together (and restore both to
  AUTO on end).
- Automatic detection of the AMD card (scans `/sys/class/drm/cardN` for
  `vendor == 0x1002`) and derives the PCI address from the device symlink, so
  the app runs on any AMD setup without configuration.
- `pkexec` fallback for sysfs writes: if the direct write is denied, the user
  is transparently prompted via polkit.
- System tray with "Show / Hide" and "Quit" menu items; closing the window
  hides it to the tray instead of quitting.
- Toast notifications for action confirmations and failures.
- Smooth `requestAnimationFrame`-based value tweens and
  `cubic-bezier(0.22, 1, 0.36, 1)` progress-bar transitions.
- Skeleton loader that mirrors the full layout so there is no content shift
  on first data arrival.
- Hover tooltips explaining Power Mode, Runtime PM, and AI Session semantics.
- Charger (AC) status indicator.

### Requirements
- Linux with an AMD GPU (tested on Radeon 880M / gfx1150).
- `webkit2gtk-4.1`.
- `libayatana-appindicator3` (for the system tray).
- `polkit` (recommended, for privileged sysfs writes).

[Unreleased]: https://github.com/Twarga/RocmTop/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/Twarga/RocmTop/releases/tag/v1.0.0
