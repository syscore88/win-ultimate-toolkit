# 🧰 WinUltimate Toolkit

A native Windows desktop application (Rust + [egui](https://github.com/emilk/egui)/`eframe`) that combines a bulk software installer, a system cleanup/debloat suite, and a services & privacy tweaks panel into a single bilingual (Polish/English) GUI tool.

---

## 🚀 Application Features

- **Bilingual UI**: Every label, button, and message is available in Polish and English, switchable live from the top bar.
- **Bulk Software Installer** (~180 popular applications, each with a one-line description in Polish/English): browsers, office suites, dev tools, AI assistants, media/DAW software, backup & partitioning tools, remote-desktop apps, communicators, and more. Programs can be selected individually or all at once, with an option to automatically skip apps that are already installed.
- **Multi-source Installation Fallback Chain**: for each selected program the app tries, in order:
  1. **winget** (by ID, with automatic package-name resolution)
  2. **Scoop**
  3. **Chocolatey**
  4. **Microsoft Store** (for apps with a known Store ID)
  5. **Direct download** of a known installer URL
  6. As a last resort, opens the program's web page in the default browser.
- **First-Run Environment Setup**: on first launch (tracked via a marker file), the app checks for and — via a small WinForms progress dialog — silently installs any missing prerequisites: **winget** (+ Windows App Runtime, UI.Xaml, VCLibs), the **OpenCL/OpenGL Compatibility Pack** (via MS Store), **Scoop**, and **Chocolatey**.
- **System Cleanup / Tools Panel**: one-click launchers and cleanup routines, including standard junk/cache/telemetry/language cleanup, Explorer and Start Menu optimization, classic context menu toggle, SFC/DISM/CHKDSK/TRIM, MSConfig, Resource Monitor, Reliability History, Windows Memory Diagnostic, and switching DNS to Cloudflare.
- **Bloatware Uninstaller**: checkbox list to remove built-in apps and features — general bloatware, Copilot, Calculator, Microsoft Edge, Notepad, Xbox services, Widgets, Phone Link, and Remote Desktop (RDP).
- **Tweaks & Services Panel**: instantly toggle on/off (individually or all at once):
  - System visual effects (performance mode)
  - A large set of "bloat" background services (SysMain, WSearch dependencies, Remote Registry, telemetry-related CDP/advertising settings, etc.)
  - Windows Search indexing, Print Spooler, Windows Image Acquisition, Bluetooth, Wi-Fi auto-config
  - Pausing Windows Update for 180 days
  - Advanced toggles kept separate from "disable all": Hibernation, BitLocker, and UAC
- **System Optimizer**: a deep cleanup routine (browser/app caches, shader caches, crash dumps, jump-list/thumbnail history, empty folders, unused language resource folders, Windows Update cache, Recycle Bin, event logs, and optional removal of legacy bloat folders like Internet Explorer/Windows Media Player), followed by `DISM /StartComponentCleanup`.
- **Extra Utilities**: updater for all installed packages, default browser switcher, quick access to the Apps Folder and Windows "God Mode" folder, an in-app EULA, and a support/donation link.
- **Persistent Settings**: language, "skip installed" preference, and the last-selected programs/tasks are saved to `%LOCALAPPDATA%\WinUltimateToolkit\settings.ini` and restored on next launch.

---

## 🔍 Module Details

### 1. First-Run Setup (`run_first_time_setup`)
Detects missing `winget`, the OpenCL/OpenGL pack, Scoop, and Chocolatey, then runs a background PowerShell routine (with its own small progress window) to install whichever are missing, before the main GUI opens.

### 2. Software Installer (`install_package` + fallback helpers)
For each selected program, tries winget → Scoop → Chocolatey → Microsoft Store → direct download, using ID-resolution helpers (`candidate_package_names`, `known_scoop_id`, `known_choco_id`, `known_msstore_id`, `known_direct_exe_url`) to map the internal winget-style ID to the right identifier for each source. Installation runs on a background thread with live status updates and a cancel flag.

### 3. Cleanup Tools & Uninstaller (`run_cleanup_tasks`)
Runs individual maintenance tasks (SFC, DISM, CHKDSK, TRIM, DNS change, etc.) or bloatware/app-removal tasks selected via checkboxes, each via PowerShell, then restarts Windows Explorer.

### 4. Tweaks & Services (`run_tweak`)
Applies or reverts registry/service changes for visual effects, background services, Windows Search, Print Spooler, WIA, Bluetooth, Wi-Fi, Windows Update pause, Hibernation, BitLocker, and UAC — each independently toggleable, plus "Disable all" / "Enable (Default)" shortcuts that batch the core set.

### 5. System Optimizer (`run_optimizer`)
A more aggressive one-shot cleanup: clears browser and app caches (Chrome, Edge, Brave, Vivaldi, Firefox, Opera/Opera GX), Discord/Telegram/WhatsApp/Twitch caches, GPU shader caches, crash dumps, jump lists, thumbnail cache, empty program-folder cleanup, unused UI-language folders, Windows Update download cache, event logs, Temp folders, and the Recycle Bin, then runs `dism /StartComponentCleanup`.

### 6. Settings Persistence
`save_settings` / `load_settings` read and write a simple `key=value` INI file under `%LOCALAPPDATA%\WinUltimateToolkit\` to remember language, preferences, selected programs, and selected cleanup tasks between runs.

---

## 🚀 How to Build & Run

To compile the project from source, you need a Rust environment (Cargo) installed.

1. Clone this repository
   ```bash
   git clone https://github.com/syscore88/win-ultimate-toolkit.git
   ```
   
2. Enter the downloaded folder
   ```bash
   cd win-ultimate-toolkit
   ```

3. Run the app in development mode
   ```bash
   cargo run
   ```

4. Build an optimized production release (the compiled `.exe` will be found in `/target/release/`)
   ```bash
   cargo build --release
   ```

> 💡 **Tip:** Since the application makes deep modifications to the registry and system services, **always run the compiled `.exe` as Administrator**.

---

### ☕ Support the Project

If you find this tool helpful and it saved you some time, consider buying me a coffee to support further development! 

[![Buy Me A Coffee](https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png)](https://buymeacoffee.com/bartekszczecinski)

<img width="1920" height="1080" alt="Screenshot_win10_2026-08-12_17:04:10" src="https://github.com/user-attachments/assets/382d9dff-04f6-4a17-94af-2ea78e0e482a" />

---

Made with passion in 🦀 **Rust**. If you find this project useful, leave a star! ⭐

## ⚠️ Notes
This tool makes advanced changes to the Windows operating system configuration, including the system registry and system services. The author takes no responsibility for any system damage, data loss, or instability caused by incorrect or unintended use of the application. Before running aggressive cleanup scripts (Tweaks/Debloater), it is strongly recommended to create a System Restore Point.
- Network access is required for package installation, the first-run setup, and any "direct download" fallback installs.
- The uninstaller/tweaks operate on the current Windows installation directly (registry edits, service configuration, app removal) — there is no dry-run mode.
- The application window defaults to 1400×750 (minimum 1200×600) and forces the dark theme with a fixed OpenGL-only rendering backend.
