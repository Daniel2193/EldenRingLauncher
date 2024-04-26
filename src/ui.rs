use std::{fs::rename, path::Path};

pub fn draw_ui(ui: &mut imgui::Ui, opened: &mut bool) {
    ui.window(format!("{} v{}", crate::WINDOW_TITLE, crate::VERSION).as_str())
        .size([crate::WINDOW_WIDTH as f32, crate::WINDOW_HEIGHT as f32], imgui::Condition::Once)
        .position([0.0, 0.0], imgui::Condition::Once)
        .movable(false)
        .resizable(false)
        .collapsible(false)
        .opened(opened)
        .build(|| {

            let path_dll = Path::new("./dinput8.dll");
            let path_dll_disabled = Path::new("./dinput8");

            if ui.button_with_size("Start ModEngine2/EldenModLoader", [(crate::WINDOW_WIDTH as f32) - 30.0, ((crate::WINDOW_HEIGHT as f32) / 4.0) - 30.0]) {
                let path = Path::new("./ModEngine2/modengine2_launcher.exe");

                if !path_dll.exists() && !path_dll_disabled.exists() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Warning)
                        .set_title("Warning!")
                        .set_text("dinput8.dll not found")
                        .show_alert();
                    return;
                }

                if path_dll_disabled.exists() {
                    _ = rename(path_dll_disabled, path_dll);
                }

                if !path.exists() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Error)
                        .set_title("Error!")
                        .set_text("Could not find ModEngine2\\modengine2_launcher.exe")
                        .show_alert();

                    return;
                }

                if let Err(e) = std::process::Command::new(path).current_dir("ModEngine2").args(["-t", "er", "-c", ".\\config_eldenring.toml"]).spawn() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Error)
                        .set_title("Error!")
                        .set_text(format!("Could not launch modengine2_launcher.exe\n\nError: {}", e).as_str())
                        .show_alert();
                } else {
                    std::process::exit(0);
                }
            }

            ui.separator();

            if ui.button_with_size("Start Seamless Coop", [(crate::WINDOW_WIDTH as f32) - 30.0, ((crate::WINDOW_HEIGHT as f32) / 4.0) - 30.0]) {
                let path = Path::new("./launch_elden_ring_seamlesscoop.exe");

                if !path.exists() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Error)
                        .set_title("Error!")
                        .set_text("Could not find launch_elden_ring_seamlesscoop.exe\n\nDownload the \"Seamless Coop\" mod from https://www.nexusmods.com/eldenring/mods/510")
                        .show_alert();

                    return;
                }

                if let Err(e) = std::process::Command::new(path).arg("-eac-nop-loaded").spawn() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Error)
                        .set_title("Error!")
                        .set_text(format!("Could not open launch_elden_ring_seamlesscoop.exe\n\nError: {}", e).as_str())
                        .show_alert();
                } else {
                    std::process::exit(0);
                }
            }

            ui.separator();

            if ui.button_with_size("Start Online (Vanilla)", [(crate::WINDOW_WIDTH as f32) - 30.0, ((crate::WINDOW_HEIGHT as f32) / 4.0) - 30.0]) {
                let path = Path::new("./start_protected_game_original.exe");

                if path_dll.exists(){
                    match rename(path_dll, path_dll_disabled){
                        Ok(_) => (),
                        Err(_) => {
                            _ = native_dialog::MessageDialog::new()
                                .set_type(native_dialog::MessageType::Error)
                                .set_title("Error!")
                                .set_text("Could not rename dinput8.dll to dinput8\n\nRefusing to launch the game")
                                .show_alert();
                            return;
                        }
                    }
                }

                if !path.exists() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Error)
                        .set_title("Error!")
                        .set_text("Could not find start_protected_game_original.exe\n\nDid you follow the installation instructions correctly?")
                        .show_alert();

                    return;
                }

                if let Err(e) = std::process::Command::new(path).spawn() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Error)
                        .set_title("Error!")
                        .set_text(format!("Could not open start_protected_game.exe\n\nError: {}", e).as_str())
                        .show_alert();
                } else {
                    std::process::exit(0);
                }
            }

            ui.separator();

            if ui.button_with_size("Start Offline (Vanilla)", [(crate::WINDOW_WIDTH as f32) - 30.0, ((crate::WINDOW_HEIGHT as f32) / 4.0) - 30.0]) {
                let path = Path::new("./eldenring.exe");

                if path_dll.exists(){
                    _ = rename(path_dll, path_dll_disabled);
                }

                if !path.exists() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Error)
                        .set_title("Error!")
                        .set_text("Could not find eldenring.exe")
                        .show_alert();

                    return;
                }

                if let Err(e) = std::process::Command::new(path).arg("-eac-nop-loaded").spawn() {
                    _ = native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Error)
                        .set_title("Error!")
                        .set_text(format!("Could not open eldenring.exe\n\nError: {}", e).as_str())
                        .show_alert();
                } else {
                    std::process::exit(0);
                }
            }
        });
}
