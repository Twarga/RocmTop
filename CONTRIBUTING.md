# Contributing to RocmTop

Thanks for your interest in RocmTop. This is a small, focused hobby project,
and the best contributions are targeted — bug fixes, compatibility
improvements for other AMD GPUs, and small UX polish.

## Reporting bugs

Open an issue with:

- Distro and kernel version (`uname -r`, `cat /etc/os-release | head`).
- GPU model (`lspci -nn | grep -Ei 'vga|3d'`).
- `ls /sys/class/drm/card*/device/vendor` and the value of each `vendor` file.
- RocmTop version (visible in the window footer).
- Exact behaviour you expected vs. what happened, with screenshots if
  relevant.

Please redact any system IDs you consider sensitive.

## Requesting a feature

Open an issue describing the problem first, not the proposed solution. The
scope of this project is intentionally small: a lightweight mini app for
monitoring and quick-switching an AMD GPU's power behaviour on Linux. If a
request expands that scope, it may be politely declined.

## Development setup

```bash
# Install Rust + cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install system libraries (example for Arch / CachyOS)
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator polkit

# Install Node dependencies
npm install

# Run in dev mode with hot reload
npm run tauri dev
```

For a release build:

```bash
npm run tauri build
```

The AppImage is emitted to `src-tauri/target/release/bundle/appimage/`.

## Code style

- **Rust**: `cargo fmt` for formatting, `cargo clippy --all-targets -- -D warnings`
  for lints. Prefer explicit error messages over generic "unknown" values.
- **TypeScript / React**: default `tsc` strictness. Keep components small and
  prop-first (no hidden globals). Follow the existing file layout:
  - `src/components/` — presentational components, one per file.
  - `src/hooks/` — custom hooks, one per file.
  - `src/types/` — shared TS types (mirror Rust structs).
- **Commits**: prefix with `feat:`, `fix:`, `docs:`, `refactor:`, `chore:`,
  or `perf:` followed by a terse imperative subject. Wrap body at 72 chars.
- **No dependencies without a reason**: this project deliberately ships no
  chart library, no animation library, and no CSS framework. Please discuss
  before adding any new runtime dependency.

## Pull requests

- One logical change per PR. Small PRs are reviewed faster.
- `cargo test` and `npm run build` must pass locally.
- Update `CHANGELOG.md` (the `[Unreleased]` section) when the change is
  user-visible.
- If the change touches sysfs paths or detection logic, include the output
  of `ls -la /sys/class/drm/card*/device/` from the affected system.

## Scope reminder

RocmTop does **one** thing: it reads a handful of amdgpu sysfs nodes and lets
the user flip two of them. It is not a replacement for `radeontop`, `nvtop`,
or CoreCtrl, and it is not a benchmarking or overclocking tool. Contributions
that preserve this focus are welcome.
