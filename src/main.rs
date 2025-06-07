use std::{io::{self, Write}};

enum Command {
    Add(String),
    Remove(usize),
    List,
    Quit,
}

struct GlobalStore {
    list: Vec<String>,
}

impl GlobalStore {
    fn new() -> Self {
        Self {list: Vec::new()}
    }

    fn update(&mut self, cmd: Command) {
        match cmd {
            Command::Add(x) => {
                self.list.push(x);
                println!("Item added");
            },
            Command::List => self.display_list(),
            Command::Remove(x) => {
                if self.list.len() <= x {
                    eprintln!("index out of range!");
                }
                else {
                    self.list.remove(x);
                    println!("Item removed");
                }
            },
            Command::Quit => {}
        }
    }
    fn display_list(&self) {
        if self.list.is_empty() {
            println!("No item in the list");
        }
        else {
            for (i, item) in self.list.iter().enumerate() {
                println!("{}: {}", i, item);
            }
        }
    }
}

fn main() {
    let mut global_store = GlobalStore::new();
    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                
                match process_input(&input) {
                    Ok(Command::Quit) => break,
                    Ok(cmd) => global_store.update(cmd),
                    Err(err) => {
                        eprintln!("Error: {}", err);
                    }
                }
                println!();
            },
            Err(err) => eprintln!("Error! Unable to read input: {}", err),
        }
    }
}

fn process_input(input: &str) -> Result<Command, String> {
    let (cmd, arg) = input.trim().split_once(' ').unwrap_or((input.trim(), ""));
    let arg = arg.trim();

    match cmd.to_ascii_lowercase().as_str() {
        "quit" => Ok(Command::Quit),

        "list" => Ok(Command::List),

        "add" => {
            if arg.is_empty() {
                Err(format!("invalid argument: empty item found\n add <item to add>"))
            }
            else {
                Ok(Command::Add(arg.to_string()))
            }
        }

        "remove" => {
            arg.parse().map(Command::Remove).map_err(|e| format!("invalid argument: {}\n> remove <index>", e))
        }

        _ => {
            Err(format!("Unknown command: {}", cmd))
        }
    }
}

// fn handle_command(cmd: Command) {
//     match cmd {
//         Command::Add(x) => {
//             println!("Item added: \"{}\"", x);
//         }
//         Command::List => {
//             println!("All items: ");
//         }
//         Command::Remove(x) => {
//             println!("{}th item removed", x)
//         }
//         _ => {}
//     }
// }
