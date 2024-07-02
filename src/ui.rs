use std::{
    ffi::CString,
    fs::rename,
    path::{Path, MAIN_SEPARATOR},
    process::Command,
    thread,
};
use winapi::um::winuser::FindWindowA;

use crate::{
    BASEGAME_EXE, DLL_DISABLED, DLL_ENABLED, EAC_EXE, MODENGINE2_DIR, MODENGINE2_EXE, SEAMLESS_EXE,
    SEAMLESS_URL,
};

pub fn draw_ui(ui: &mut imgui::Ui, opened: &mut bool) {
    ui.window(format!("{} v{}", crate::WINDOW_TITLE, crate::VERSION).as_str())
        .size([crate::WINDOW_WIDTH as f32, crate::WINDOW_HEIGHT as f32], imgui::Condition::Once)
        .position([0.0, 0.0], imgui::Condition::Once)
        .movable(false)
        .resizable(false)
        .collapsible(false)
        .opened(opened)
        .build(|| {
            let path_dll = Path::new(DLL_ENABLED);
            let path_dll_disabled = Path::new(DLL_DISABLED);
            if ui.button_with_size("Start ModEngine2/EldenModLoader", [(crate::WINDOW_WIDTH as f32) - 30.0, ((crate::WINDOW_HEIGHT as f32) / 4.0) - 30.0]) {
                let path_me2l = Path::new(MODENGINE2_DIR).join(MODENGINE2_EXE);
                if !path_dll.exists() && !path_dll_disabled.exists() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Warning)
                        .set_title("Warning!")
                        .set_text(format!("{} not found", DLL_ENABLED).as_str())
                        .show_alert();
                    return;
                }
                if path_dll_disabled.exists() {
                    _ = rename(path_dll_disabled, path_dll);
                }
                if !path_me2l.exists() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Error)
                        .set_title("Error!")
                        .set_text(format!("Could not find {}{}{}", MODENGINE2_DIR, MAIN_SEPARATOR, MODENGINE2_EXE).as_str())
                        .show_alert();
                    return;
                }
                if let Err(e) = Command::new(path_me2l).current_dir(MODENGINE2_DIR).args(["-p", format!("..{}{}", MAIN_SEPARATOR, BASEGAME_EXE).as_str(),"-t", "er", "-c", ".\\config_eldenring.toml"]).spawn() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Error)
                        .set_title("Error!")
                        .set_text(format!("Could not launch {}\n\nError: {}", MODENGINE2_EXE, e).as_str())
                        .show_alert();
                } else {
                    for _ in 0..10 {
                        if check_if_game_launched() {
                            break;
                        }
                        thread::sleep(std::time::Duration::from_secs(1));
                    }
                    if path_dll.exists() {
                        _ = rename(path_dll, path_dll_disabled);
                    }
                    std::process::exit(0);
                }
            }

            ui.separator();

            if ui.button_with_size("Start Seamless Coop", [(crate::WINDOW_WIDTH as f32) - 30.0, ((crate::WINDOW_HEIGHT as f32) / 4.0) - 30.0]) {
                let path_coop = Path::new(SEAMLESS_EXE);
                if path_dll.exists(){
                    _ = rename(path_dll, path_dll_disabled);
                }
                if !path_coop.exists() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Error)
                        .set_title("Error!")
                        .set_text(format!("Could not find {}\n\nYou can download the \"Seamless Coop\" mod from {}", SEAMLESS_EXE, SEAMLESS_URL).as_str())
                        .show_alert();
                    return;
                }
                if let Err(e) = std::process::Command::new(path_coop).current_dir(".").spawn() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Error)
                        .set_title("Error!")
                        .set_text(format!("Could not open {}\n\nError: {}", SEAMLESS_EXE, e).as_str())
                        .show_alert();
                } else {
                    std::process::exit(0);
                }
            }

            ui.separator();

            if ui.button_with_size("Start Online (Vanilla)", [(crate::WINDOW_WIDTH as f32) - 30.0, ((crate::WINDOW_HEIGHT as f32) / 4.0) - 30.0]) {
                let path_eac = Path::new(EAC_EXE);
                if path_dll.exists(){
                    match rename(path_dll, path_dll_disabled){
                        Ok(_) => (),
                        Err(_) => {
                            _ = native_dialog::MessageDialog::new()
                                .set_type(native_dialog::MessageType::Error)
                                .set_title("Error!")
                                .set_text(format!("Could not rename {} to {}\n\nRefusing to launch the game", DLL_ENABLED, DLL_DISABLED).as_str())
                                .show_alert();
                            return;
                        }
                    }
                }
                if !path_eac.exists() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Error)
                        .set_title("Error!")
                        .set_text(format!("Could not find {}\n\nDid you follow the install instructions correctly?", EAC_EXE).as_str())
                        .show_alert();
                    return;
                }
                if let Err(e) = std::process::Command::new(path_eac).spawn() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Error)
                        .set_title("Error!")
                        .set_text(format!("Could not open {}\n\nError: {}", EAC_EXE, e).as_str())
                        .show_alert();
                } else {
                    std::process::exit(0);
                }
            }

            ui.separator();

            if ui.button_with_size("Start Offline (Vanilla)", [(crate::WINDOW_WIDTH as f32) - 30.0, ((crate::WINDOW_HEIGHT as f32) / 4.0) - 30.0]) {
                let path_game = Path::new(BASEGAME_EXE);
                if path_dll.exists(){
                    _ = rename(path_dll, path_dll_disabled);
                }
                if !path_game.exists() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Error)
                        .set_title("Error!")
                        .set_text(format!("Could not find {}", BASEGAME_EXE).as_str())
                        .show_alert();
                    return;
                }
                if let Err(e) = std::process::Command::new(path_game).arg("-eac-nop-loaded").spawn() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Error)
                        .set_title("Error!")
                        .set_text(format!("Could not open {}\n\nError: {}", BASEGAME_EXE, e).as_str())
                        .show_alert();
                } else {
                    std::process::exit(0);
                }
            }
        });
}

fn check_if_game_launched() -> bool {
    let file = CString::new(BASEGAME_EXE).expect("CString::new failed");
    let window_handle = unsafe { FindWindowA(std::ptr::null_mut(), file.as_ptr()) };
    window_handle != std::ptr::null_mut()
}
