use std::io::{self, Write};

pub struct State {
    menu: String,
    commands: Vec<String>,
    comment: String,
}

pub fn start_splash() {
    println!("Fast Files - File directory storage UI for fast access and sorting\n");
}

fn poll_commands(state: &mut State) -> () { // poll_command() should return type fn()
    // initialize text input variables
    let mut text_entry = String::new();
    let mut commands: Vec<String> = Vec::new();

    // preparing IO
    print!("> ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut text_entry)
        .expect("Failed to read line");   

    // tokenizing commands
    for word in text_entry.split_whitespace() {
        commands.push(String::from(word));
    }        

    // making sure first command is only a single character, else try polling again
    if commands[0].len() != 1 {
        println!("Invalid command at index 0, character length should be 1");
        poll_commands(state);
    }

    // creating list of valid arguments depending on the menu state
    let valid_args = &state.commands;

    // command list: level1 checks if the command is available in the menu state, level2 executes the command.
    let _command_level1 = match &commands[0] {
        x if valid_args.contains(x) => {
            let _command_level2 = match commands[0].as_str() {
                "l" => return state.directories(),
                "o" => return state.directories(), // change functionality when implimented
                "n" => return state.directories(), // change functionality when implimented
                "r" => return state.main_menu(),
                "d" => return state.directories(), // change functionality when implimented
                "R" => return state.directories(), // change functionality when impliemnted
                "q" => std::process::exit(0),
                &_ => poll_commands(state),
            };
        },
        &_ => poll_commands(state), 
    };
    // ToDo: Check to see if second command which should be a directory exists in the directory storage file.

}
impl State {
    
    // On program startup, create a new state which dictates what menu and options are displayed
    pub fn new() -> State {
        State {
            menu: String::new(),
            commands: Vec::new(),
            comment: String::new(),
        }
    }

    // Update state with corresponding options: main_menu(), directories().
    // called from the aforementioned functions.
    fn update(&mut self, status: &str) {
        self.menu = String::from(status);
        self.update_commands()
    }

    // Update command list display following change to State's 'menu' field.
    // called from update()
    fn update_commands(&mut self) {
        match self.menu.as_str() {
            "MainMenu" => {
                self.comment = String::from("[l] - List saved directories\n[o] - Open file\n[n] - New Directory\n[R] - Refresh saved directories\n[d] - Default opening process\n[q] - Quit\n\n");
                self.commands = Vec::from(["l".to_string(), "o".to_string(), "n".to_string(), "R".to_string(), "d".to_string(), "q".to_string()]);
            },
            "Directories" => {
                self.comment = String::from("[#] - Open file number\n[s] - Change sort (current: last modified)\n[d] - Delete directory\n[r] - Return to main menu\n[q] - Quit\n\n");
                self.commands = vec!["s".to_string(), "d".to_string(), "r".to_string(), "q".to_string()];
            },
            &_ => ()
        };
    }

    // returns list of available commands following a call from State change functions
    fn print_commands(&self) -> &String {
        &self.comment
    }

    // State change functions
    //
    // Changes State to main menu
    pub fn main_menu(&mut self) {
        self.update("MainMenu");
        std::process::Command::new("clear").status().unwrap();
        print!("{}", self.print_commands());
        poll_commands(self);
    }

    // Changes State to directories
    pub fn directories(&mut self) {
        self.update("Directories");
        std::process::Command::new("clear").status().unwrap();
        print!("{}", self.print_commands());
        poll_commands(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_state() {
        let mut state = State::new();
        assert_eq!(state.menu, "", "testing menu State creation");
    }

    #[test]
    fn generate_commands() {
        let mut state = State::new();
        assert_eq!(state.commands, "", "testing commands State creation");
    }

    #[test]
    fn change_main_menu() {
        let mut state = State::new();
        state.directories();
        state.main_menu();
        assert_eq!(state.menu, "MainMenu", "testing if fn main_menu() acts correctly");
    }

    #[test]
    fn change_directories() {
        let mut state = State::new();
        state.main_menu();
        state.directories();
        assert_eq!(state.menu, "Directories", "testing if fn directories() acts correctly");
    }

    #[test]
    fn display_correct_commands() {
        let mut state = State::new();
        state.main_menu();
        assert_eq!(state.commands,
                   "[l] - List saved directories\n[o] - Open file\n[n] - New Directory\n[r] - Refresh saved directories\n[d] - Default opening process\n[q] - Quit\n",
                   "testing if commands are selected correctly")
    }
}

