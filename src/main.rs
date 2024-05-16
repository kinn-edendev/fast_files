use fast_files::*;

fn main() {
    let mut state = State::new();
    state.build("ff_directories.txt".to_string(), "ff_launchcommands.txt".to_string());

    std::process::Command::new("clear").status().unwrap();
    start_splash();
    state.main_menu();
}
