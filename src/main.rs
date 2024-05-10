use fast_files::{State, start_splash};

fn main() {
    let mut state = State::new();

    start_splash();
    state.main_menu();
}
