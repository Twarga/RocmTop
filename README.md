# RocmTop

Lightweight AMD GPU Monitor for Linux built with Tauri v2 + React + TypeScript.

## Features

- Real-time GPU temperature, clock, VRAM, and load monitoring
- One-click power mode switching (HIGH/AUTO)
- AI Session quick controls
- System tray integration
- <10MB AppImage, <30MB memory usage

## Installation

Download the AppImage from [Releases](https://github.com/Twarga/RocmTop/releases).

```bash
chmod +x RocmTop-x86_64.AppImage
./RocmTop-x86_64.AppImage
```

## Requirements

- Linux with AMD GPU (tested on Radeon 880M)
- WebKitGTK (pre-installed on most distros)

## Build from Source

```bash
git clone https://github.com/Twarga/RocmTop.git
cd RocmTop
npm install
npm run tauri build
```

## License

MIT
