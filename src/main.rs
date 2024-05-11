use fast_files::*;

fn main() {
    let mut state = State::new();

    std::process::Command::new("clear").status().unwrap();
    start_splash();
    state.main_menu();
}
