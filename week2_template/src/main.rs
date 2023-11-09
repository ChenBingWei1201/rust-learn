struct Check {
    name: String,
    description: String,
    done: bool,
}

struct Progress {
    name: String,
    description: String,
    progress: u8,
}

trait Todo {
    fn new(name: String, description: String) -> Self;

    fn to_string(&self, index: usize) -> String;
    fn done(&self) -> bool;

    fn set_status(&mut self);
}

impl Todo for Check {
    fn new(name: String, description: String) -> Self {
        // todo!()
        Check {
            name,
            description,
            done: false
        }
    }

    fn to_string(&self, index: usize) -> String {
        todo!()
    }

    fn done(&self) -> bool {
        todo!()
    }

    fn set_status(&mut self) {
        todo!()
    }
}

impl Todo for Progress {
    fn new(name: String, description: String) -> Self {
        // todo!()
        Progress {
            name,
            description,
            progress: 0
        }
    }

    fn to_string(&self, index: usize) -> String {
        todo!()
    }

    fn done(&self) -> bool {
        todo!()
    }

    fn set_status(&mut self) {
        todo!()
    }
}

fn stdin() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap_or(0);
    input.trim().to_string()
}

fn string_to_u8(input: String) -> Option<u8> {
    match input.parse::<u8>() {
        Ok(progress) => Some(progress),
        Err(_) => None,
    }
}

fn create_todo<T: Todo>(todo_list: &mut Vec<T>) {
    // todo!()
    println!("Name:");
    let name = stdin();
    println!("Description:");
    let description = stdin();
    let new_todo = T::new(name, description);
    todo_list.push(new_todo);
}

fn main() {
    // There's cleaner way to write this code, but not now
    let mut check_list: Vec<Check> = Vec::new();
    let mut progress_list: Vec<Progress> = Vec::new();
    loop {
        println!("What do you want to do?");
        println!("1. Add new todo");
        println!("2. Edit todo");
        println!("3. Show todo list");
        println!("4. Exit");

        // todo!()
        let choice = stdin();
        // choice = stdin();
        match choice.as_str() {
            "1" => {
                println!("What kind of todo you want to add?");
                println!("1. Check");
                println!("2. Progress");
                let choice = stdin();
                match choice.as_str() {
                    "1" => {
                        create_todo(&mut check_list);
                    },
                    "2" => {
                        create_todo(&mut progress_list);
                    },
                    _ => println!("invalid input")
                }
            },
            "2" => {
                println!("Which to do you want to edit? [1 - {}]", progress_list.len())
            },
            "3" => {

            },
            "4" => break,
            _ => println!("invalid input")
        };
        
            
    }
}
