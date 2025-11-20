use std::process::Command;

pub fn clean_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/c").arg("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}
