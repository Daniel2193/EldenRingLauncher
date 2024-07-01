# Elden Ring Launcher v3.2

A simple utility to launch Elden Ring with or without anti-cheat, with ModEngine2 and/or EldenModLoader.
## Installation

1. Download the latest [release](https://github.com/Daniel2193/EldenRingLauncher/releases/latest).
2. Navigate to your Elden Ring installation directory (right click the game on Steam > `Manage` > `Browse Local Files`)
   and navigate into the `Game` folder.
3. Rename the existing `start_protected_game.exe` to `start_protected_game_original.exe`.
4. Drag the downloaded `start_protected_game.exe` into your `Elden Ring/Game` folder.
5. (Optional) Copy/Hardlink your `ModEngine2` Folder (the one that contains `modengine2_launcher.exe`) into your Elden Ring directory and if necessary, rename it to `ModEngine2`
6. Start Elden Ring through Steam.

## Building

Download [Rust](https://www.rust-lang.org/tools/install) and run the `cargo build --release` command.
