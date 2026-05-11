# RocmTop — Task Breakdown

**Project:** RocmTop — Lightweight AMD GPU Monitor for Linux  
**Tech Stack:** Tauri v2 + React + TypeScript + Rust  
**Target:** Radeon 880M (gfx1150) on CachyOS  
**Output:** AppImage (~8MB), GitHub Pages landing page

---

## Phase 1: Project Initialization

### TA-1: Create GitHub Repository
- Initialize git repo in `/home/twarga/Projects/AMD_Monitor`
- Create `README.md` with project description
- Commit and push to `https://github.com/Twarga/RocmTop.git`
- Set up `.gitignore` for Tauri + Node + Rust

### TA-2: Initialize Tauri v2 Project
- Run `npm create tauri-app@latest` with React + TypeScript template
- Project name: `rocm-top`
- Configure for Linux target only
- Set up basic project structure

### TA-3: Configure Rust for Minimal Binary
- Update `Cargo.toml` with release profile optimizations:
  - `lto = true`
  - `codegen-units = 1`
  - `panic = "abort"`
  - `strip = true`
- Add dependencies: `serde`, `serde_json`
- Configure `tauri.conf.json` with app identifier and window settings

### TA-4: Set Up TypeScript Types
- Create `src/types/gpu.ts` with interfaces:
  - `GpuStats` — all metrics in one object
  - `PowerMode` — "high" | "auto"
  - `RuntimePmMode` — "on" | "auto"
- Export types for use across components

---

## Phase 2: Rust Backend — Sysfs Reading

### TA-5: Implement Temperature Reader
- Create `src-tauri/src/commands.rs`
- Function: `get_temperature() -> Result<u32, String>`
- Glob `/sys/class/hwmon/hwmon*/temp1_input`
- Parse value, divide by 1000 for °C
- Fallback: return 0 if no sensor found
- Add unit test with mock sysfs path

### TA-6: Implement GPU Clock Reader
- Function: `get_gpu_clock() -> Result<u32, String>`
- Read `/sys/class/drm/card1/device/pp_dpm_sclk`
- Parse line containing `*` marker
- Extract MHz value
- Return 0 if unreadable

### TA-7: Implement GPU Busy Percent Reader
- Function: `get_gpu_busy() -> Result<u32, String>`
- Read `/sys/class/drm/card1/device/gpu_busy_percent`
- Parse as integer
- Return 0 if unreadable

### TA-8: Implement VRAM Readers
- Function: `get_vram_used() -> Result<u64, String>`
- Read `/sys/class/drm/card1/device/mem_info_vram_used`
- Return bytes (convert to MB in frontend)
- Function: `get_vram_total() -> Result<u64, String>`
- Read `/sys/class/drm/card1/device/mem_info_vram_total`
- Return bytes

### TA-9: Implement Power Mode Reader
- Function: `get_power_mode() -> Result<String, String>`
- Read `/sys/class/drm/card1/device/power_dpm_force_performance_level`
- Return "high", "auto", or "manual"
- Return "unknown" if unreadable

### TA-10: Implement Charger Status Reader
- Function: `get_charger_status() -> Result<bool, String>`
- Read `/sys/class/power_supply/AC/online`
- Fallback: `/sys/class/power_supply/ACAD/online`
- Return true if plugged, false if battery
- Return false if unreadable

### TA-11: Implement Runtime PM Reader
- Function: `get_runtime_pm() -> Result<String, String>`
- Read `/sys/bus/pci/devices/0000:63:00.0/power/control`
- Return "on" or "auto"
- Return "unknown" if unreadable

### TA-12: Create Aggregated Stats Command
- Function: `get_all_stats() -> Result<GpuStats, String>`
- Call all individual readers
- Return single `GpuStats` struct
- Handle partial failures gracefully (N/A for failed fields)

---

## Phase 3: Rust Backend — Sysfs Writing

### TA-13: Implement Power Mode Writer
- Function: `set_power_mode(mode: String) -> Result<(), String>`
- Write to `/sys/class/drm/card1/device/power_dpm_force_performance_level`
- Validate input: "high" or "auto" only
- Return error message if write fails
- Log write operation for debugging

### TA-14: Implement Runtime PM Writer
- Function: `set_runtime_pm(mode: String) -> Result<(), String>`
- Write to `/sys/bus/pci/devices/0000:63:00.0/power/control`
- Validate input: "on" or "auto" only
- Return error message if write fails

### TA-15: Implement AI Session Commands
- Function: `start_ai_session() -> Result<(), String>`
  - Call `set_power_mode("high")`
  - Call `set_runtime_pm("on")`
  - Return error if either fails
- Function: `end_ai_session() -> Result<(), String>`
  - Call `set_power_mode("auto")`
  - Call `set_runtime_pm("auto")`
  - Return error if either fails

### TA-16: Register Commands in Tauri
- Import all commands in `src-tauri/src/main.rs`
- Register with `tauri::Builder::invoke_handler()`
- Export commands module in `lib.rs`
- Verify commands are callable from frontend

---

## Phase 4: React Frontend — Core UI

### TA-17: Create Base CSS Styles
- Create `src/styles.css`
- Dark theme color palette
- CSS variables for colors
- Base layout: 420×560px window
- Grid system for metric cards
- Button styles for toggles

### TA-18: Create Metric Card Component
- Create `src/components/MetricCard.tsx`
- Props: `title`, `value`, `unit`, `percentage`, `status`
- Display large value with unit
- Progress bar showing percentage
- Status label below value
- Reusable for all 4 metrics

### TA-19: Create Toggle Button Component
- Create `src/components/ToggleButton.tsx`
- Props: `label`, `options`, `activeOption`, `onChange`
- Render 2-3 toggle buttons in a row
- Highlight active option
- Call `onChange` on click

### TA-20: Create Status Row Component
- Create `src/components/StatusRow.tsx`
- Props: `label`, `value`, `icon`
- Display label and value in a row
- Optional icon for charger status
- Use for charger and last updated

### TA-21: Build Main App Layout
- Update `src/App.tsx` with full layout
- 2×2 grid of MetricCards (temp, clock, load, VRAM)
- Status section with ToggleButtons
- AI Session buttons
- Footer with last updated timestamp
- Wire up to `useGpuStats` hook

---

## Phase 5: React Frontend — Data Layer

### TA-22: Create Polling Hook
- Create `src/hooks/useGpuStats.ts`
- Poll every 2 seconds using `setInterval`
- Call `invoke('get_all_stats')` via Tauri
- Return `GpuStats | null`
- Handle errors gracefully
- Cleanup interval on unmount

### TA-23: Implement Temperature Color Coding
- Add CSS classes: `.temp-normal`, `.temp-warm`, `.temp-hot`
- Logic in `App.tsx`:
  - `< 80°C` → green
  - `80-88°C` → yellow
  - `> 88°C` → red
- Apply class to temperature MetricCard

### TA-24: Wire Up Power Mode Toggle
- Add state for power mode in `App.tsx`
- Call `invoke('set_power_mode')` on toggle click
- Update local state after successful write
- Show error toast if write fails

### TA-25: Wire Up Runtime PM Toggle
- Add state for runtime PM in `App.tsx`
- Call `invoke('set_runtime_pm')` on toggle click
- Update local state after successful write
- Show error toast if write fails

### TA-26: Wire Up AI Session Buttons
- "Start AI Session" button:
  - Call `invoke('start_ai_session')`
  - Update power mode and runtime PM state
- "End AI Session" button:
  - Call `invoke('end_ai_session')`
  - Update power mode and runtime PM state
- Show success/error feedback

### TA-27: Add Last Updated Timestamp
- Track last poll time in `useGpuStats`
- Display "Last updated: Xs ago"
- Update every second via separate interval
- Show "Updating..." during poll

---

## Phase 6: System Tray Integration

### TA-28: Configure System Tray in Tauri
- Update `tauri.conf.json` with tray config
- Add tray icon (SVG or PNG)
- Set tray tooltip to show current temperature
- Configure show/hide on tray click

### TA-29: Implement Tray Icon Color Changes
- Create function to update tray icon color
- Change based on temperature thresholds
- Green (<80°C), Yellow (80-88°C), Red (>88°C)
- Update icon on every poll

### TA-30: Implement Minimize to Tray
- Override window close behavior
- Minimize to tray instead of quitting
- Add "Quit" option in tray menu
- Remember window position

---

## Phase 7: Testing & Quality Assurance

### TA-31: Write Rust Unit Tests
- Test each sysfs reader with mock paths
- Test error handling for unreadable files
- Test validation in writer functions
- Run `cargo test` and ensure all pass

### TA-32: Write Frontend Component Tests
- Test MetricCard renders correctly
- Test ToggleButton state changes
- Test useGpuStats hook with mock data
- Run tests with `npm test`

### TA-33: Manual Integration Testing
- Verify all metrics display correctly
- Test power mode toggle writes to sysfs
- Test runtime PM toggle writes to sysfs
- Test AI session buttons change both settings
- Verify no crashes on file errors
- Test tray functionality
- Test window minimize/show

### TA-34: Performance Profiling
- Measure memory usage with `htop`
- Verify <30MB memory footprint
- Measure startup time with `time ./rocm-top`
- Verify <1s startup
- Measure CPU usage while polling
- Verify <1% CPU idle, <0.5% polling

---

## Phase 8: Build & Package

### TA-35: Build Development Binary
- Run `npm run tauri dev`
- Verify dev build works
- Test hot reload functionality
- Check console for errors

### TA-36: Build Release Binary
- Run `npm run tauri build`
- Output: `src-tauri/target/release/rocm-top`
- Verify binary size <5MB
- Test release binary on clean system

### TA-37: Create AppImage Package
- Configure AppImage build in `tauri.conf.json`
- Run `npm run tauri build -- --target appimage`
- Output: `RocmTop-x86_64.AppImage`
- Verify AppImage size <10MB
- Test AppImage on CachyOS

### TA-38: Create .desktop File
- Create `RocmTop.desktop` entry
- Include in AppImage resources
- Set icon and categories
- Test app appears in system menu

---

## Phase 9: Documentation

### TA-39: Update README.md
- Project description and features
- Screenshots of the app
- Installation instructions
- Usage guide
- Build from source instructions
- License and credits

### TA-40: Create CHANGELOG.md
- Initial release notes
- List all features
- Known limitations
- Future roadmap

### TA-41: Create CONTRIBUTING.md
- How to report bugs
- How to submit PRs
- Code style guidelines
- Development setup

---

## Phase 10: GitHub Pages Landing Page

### TA-42: Create Landing Page HTML
- Create `docs/index.html`
- Project name and tagline
- Feature list with icons
- Screenshots section
- Download link for AppImage
- GitHub repo link
- Responsive design

### TA-43: Create Landing Page CSS
- Create `docs/styles.css`
- Dark theme matching app
- Modern, clean design
- Mobile responsive
- Smooth animations

### TA-44: Add Screenshots to Landing Page
- Capture screenshots of app running
- Optimize images for web
- Add to `docs/screenshots/`
- Display in gallery format

### TA-45: Configure GitHub Pages
- Enable GitHub Pages in repo settings
- Set source to `docs/` folder
- Verify page loads at `https://twarga.github.io/RocmTop/`
- Add custom domain (optional)

---

## Phase 11: Release to GitHub

### TA-46: Create Release Tag
- Tag version as `v1.0.0`
- Create annotated tag with release notes
- Push tag to GitHub

### TA-47: Create GitHub Release
- Go to GitHub Releases
- Create new release from tag
- Upload AppImage binary
- Include release notes from CHANGELOG.md
- Mark as latest release

### TA-48: Update Landing Page with Download Link
- Add direct download link to AppImage
- Link to GitHub Releases page
- Add version number display

### TA-49: Final Verification
- Test download link works
- Verify landing page loads
- Check all documentation is accurate
- Ensure AppImage runs on clean system

---

## Task Summary

| Phase | Tasks | Description |
|-------|-------|-------------|
| 1 | TA-1 to TA-4 | Project initialization |
| 2 | TA-5 to TA-12 | Rust sysfs reading |
| 3 | TA-13 to TA-16 | Rust sysfs writing |
| 4 | TA-17 to TA-21 | React UI components |
| 5 | TA-22 to TA-27 | React data layer |
| 6 | TA-28 to TA-30 | System tray |
| 7 | TA-31 to TA-34 | Testing |
| 8 | TA-35 to TA-38 | Build & package |
| 9 | TA-39 to TA-41 | Documentation |
| 10 | TA-42 to TA-45 | GitHub Pages |
| 11 | TA-46 to TA-49 | Release |

**Total: 49 tasks**
