pub const WINDOW_TITLE: &str = "EldenRingLauncher";
pub const WINDOW_WIDTH: u32 = 300;
pub const WINDOW_HEIGHT: u32 = 400;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RUDA_BOLD: &[u8; 50472] = include_bytes!("../assets/Ruda-Bold.ttf");
pub const BASEGAME_EXE: &str = "eldenring.exe";
pub const EAC_EXE: &str = "start_protected_game_original.exe";
pub const SEAMLESS_EXE: &str = "ersc_launcher.exe";
pub const SEAMLESS_URL: &str = "https://github.com/LukeYui/EldenRingSeamlessCoopRelease/releases";
pub const MODENGINE2_DIR: &str = "ModEngine2";
pub const MODENGINE2_EXE: &str = "modengine2_launcher.exe";
pub const DLL_ENABLED: &str = "dinput8.dll";
pub const DLL_DISABLED: &str = "dinput8";
