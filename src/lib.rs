use std::io::{self, Write, BufRead};
use std::collections::HashMap;
use regex::Regex;
use std::{thread, time};
use std::fs::{OpenOptions, File};
use std::path::Path;

pub struct State {
    menu: String,
    commands: Vec<String>,
    comment: String,
    directories: HashMap<String, String>, // saved in order of (filepath, endpoint)
    launch_command: HashMap<String, String>, // saved in order of (extension, command)
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
                "o" => return state.open_directory(),
                "n" => return state.new_directory(),
                "r" => return state.main_menu(),
                "d" => return state.set_default_opening_process(),
                "R" => return state.refresh_list(),
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
            directories: HashMap::new(),
            launch_command: HashMap::new(),
        }
    }

    // On a blank state, populate directories and launch_command hashmaps from saved file
    pub fn build(&mut self, directories: String, launchcommands: String) {
        // populate from directories file into state.directories
        if let Ok(lines) = read_lines(directories) {
            for line in lines.flatten() {
                let mut kv_pair: Vec<String> = Vec::new();
                for keyvalue in line.split_whitespace() {
                    kv_pair.push(keyvalue.to_string());
                }
                self.directories.insert(String::from(&kv_pair[0]), String::from(&kv_pair[1]));
            }
        }
        // populate from launchcommands file into state.launch_command
        if let Ok(lines) = read_lines(launchcommands) {
            for line in lines.flatten() {
                let mut kv_pair: Vec<String> = Vec::new();
                for keyvalue in line.split_whitespace() {
                    kv_pair.push(keyvalue.to_string());
                }
                self.launch_command.insert(String::from(&kv_pair[0]), String::from(&kv_pair[1]));
            }
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
                self.comment = String::from("Select Action\n[l] - List saved directories\n[o] - Open file\n[n] - New Directory\n[R] - Refresh saved directories\n[d] - Default opening process\n[q] - Quit\n\n");
                self.commands = Vec::from(["l".to_string(), "o".to_string(), "n".to_string(), "R".to_string(), "d".to_string(), "q".to_string()]);
            },
            "Directories" => {
                self.comment = String::from("Select Action\n[#] - Open file number\n[s] - Change sort (current: last modified)\n[d] - Delete directory\n[r] - Return to main menu\n[q] - Quit\n\n");
                self.commands = vec!["s".to_string(), "d".to_string(), "r".to_string(), "q".to_string()];
            },
            &_ => ()
        };
    }

    // returns list of available commands following a call from State change functions
    fn print_commands(&self) -> &String {
        &self.comment
    }

    // adds a new directory and endpoint to the hashmap of status.directories
    fn new_directory(&mut self) {
        // initialize text input variables
        let mut text_entry = String::new();
        let mut commands: Vec<String> = Vec::new();

        // preparing IO
        print!("\nPlease enter a new filepath...\n> ");
        io::stdout().flush().expect("Failed to flush");
        io::stdin()
            .read_line(&mut text_entry)
            .expect("Failed to read line");

        // tokenizing commands
        for word in text_entry.split_whitespace() {
            commands.push(String::from(word));
        }

        // declaring regex for use in the for loop, documentation said its 'expensive' to declare
        // so its only done once.       
        let re_endpoint = Regex::new(r"(?<endpoint>[[:word:]]+\.[[:word:]]+)").unwrap();
        let re_ext = Regex::new(r"(?<ext>\.[[:word:]]+)").unwrap();
        
        // this loop captures the endpoint and extension for every inputted directory, then adds
        // them to their respective hashmaps in state.directories and state.launch_command
        for directory in commands.iter() {

            // endpoints are captured and taken out of their Option() state.
            let cap = re_endpoint.captures(directory).and_then(|cap| {
                cap.name("endpoint").map(|endpoint| endpoint.as_str())
            });
            let endpoint = match cap {
                Some(endp) => endp,
                None => "",
            };

            // extensions are captured and taken out of their Option() state
            let cap = re_ext.captures(directory).and_then(|cap| {
                cap.name("ext").map(|ext| ext.as_str())
            });
            let extension = match cap {
                Some(ex) => ex,
                None => "",
            };
            
            // directory:endpoint key-value pairs are inserted to the hashmap
            self.directories.insert(String::from(directory), String::from(endpoint));
            println!("\n{} added and saved to directories...", endpoint);

            // creating vector list of known (registered) extensions for checking later
            let mut registered_extensions: Vec<&str> = Vec::new();

            for (reg_ext, _process) in &self.launch_command {
                registered_extensions.push(reg_ext);
            }

            // if the extension is not registered, register with a command and add to hashmap
            if !registered_extensions.contains(&extension) {
                println!("New filetype discovered, what process would you like to open '{}' files with?", extension);

                // initialize text input variables
                let mut launch_command = String::new();

                // preparing IO
                print!("> ");
                io::stdout().flush().expect("Failed to flush");
                io::stdin()
                    .read_line(&mut launch_command)
                    .expect("Failed to read line");

                let launch_command = launch_command.trim_end();

                println!("\n'{}' file will now be opened with '{}' by default, use 'd' from the main menu to change.", extension, launch_command);
                self.launch_command.insert(String::from(extension), String::from(launch_command));

            }

        }

        // write changes to file "ff_directories.txt" which allows for cross-instance usage
        let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open("ff_directories.txt")
                .expect("Unable to open file");
        
        for (directory, endpoint) in &self.directories { // issue: currently rewrites entire storage when saving as a redundancy against overwrites
            let data = format!("{directory} {endpoint}\n");
            file.write(data.as_bytes()).expect("Unable to write to directories");
        }

        // write changes to file "ff_launchcommands.txt" which allows for cross-instance usage
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("ff_launchcommands.txt")
            .expect("Unable to open file");

        for (extension, commands) in &self.launch_command { // same issue
            let data = format!("{extension} {commands}\n");
            file.write(data.as_bytes()).expect("Unable to write to launchcommands");
        }

        // after waiting, return to main menu
        thread::sleep(time::Duration::from_secs(2));
        std::process::Command::new("clear").status().unwrap();
        print!("{}", self.print_commands());
        poll_commands(self);
    }

    fn open_directory(&mut self) {}

    fn set_default_opening_process(&mut self) {
        // reuse code from new_directory, swap out directories for launch_command
    }

    fn refresh_list(&mut self) {
        // maybe call build() and then reuse code from new_directory()
    }


    // State change functions
    //
    // Changes State to main menu
    pub fn main_menu(&mut self) {
        self.update("MainMenu");
        std::process::Command::new("clear").status().unwrap();
        start_splash();
        print!("{}", self.print_commands());
        poll_commands(self);
    }

    // Changes State to directories
    pub fn directories(&mut self) {
        self.update("Directories");
        std::process::Command::new("clear").status().unwrap();

        // prints directories from state.directories hashmap
        println!("Listing Saved Directories...");

        let mut count: i32 = 1;
        for (_filepath, endpoint) in &self.directories {
            println!("{}. {}", count, endpoint);
            count += 1;
        }

        print!("\n{}", self.print_commands());
        poll_commands(self);
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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

