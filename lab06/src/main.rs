use rusqlite::{Connection, params};

trait CommandTemplate {
    fn get_name(&self) -> String;
    fn exec(&mut self, arguments: &str);
}
struct Ping;
impl CommandTemplate for Ping {
    fn get_name(&self) -> String {
        "ping".to_string()
    }
    fn exec(&mut self, _arguments: &str) {
        println!("Pong!");
    }
}
struct Count;
impl CommandTemplate for Count {
    fn get_name(&self) -> String {
        "count".to_string()
    }
    fn exec(&mut self, arguments: &str) {
        let mut nr = 0;
        for args in arguments.split(' ') {
            if !args.is_empty() {
                nr += 1;
            }
        }
        println!("{}", nr);
    }
}
struct Times {
    count: i32,
}
impl CommandTemplate for Times {
    fn get_name(&self) -> String {
        "times".to_string()
    }
    fn exec(&mut self, _arguments: &str) {
        self.count += 1;
        println!("{}", self.count)
    }
}
struct New;
impl CommandTemplate for New {
    fn get_name(&self) -> String {
        "new".to_string()
    }
    fn exec(&mut self, _arguments: &str) {
        println!("New!");
    }
}
struct Bk;
impl CommandTemplate for Bk {
    fn get_name(&self) -> String {
        "bk".to_string()
    }
    fn exec(&mut self, arguments: &str) {
        let mut args = arguments.split(' ').into_iter();
        let arg1 = args.next().unwrap_or("");
        if arg1 == "add" {
            let name = args.next().unwrap_or("");
            let url = args.next().unwrap_or("");
            //TODO
            println!("{name} {url}");
        }
        if arg1 == "search" {
            let name = args.next().unwrap_or("");
            //TODO
            println!("{name}");
        }
    }
}

struct Terminal {
    registered_commands: Vec<Box<dyn CommandTemplate>>,
    run_list: String,
}
impl Terminal {
    fn new() -> Terminal {
        Terminal {
            registered_commands: (Vec::new()),
            run_list: std::fs::read_to_string("input.txt").unwrap_or("".to_string()),
        }
    }
    fn register(&mut self, command: Box<dyn CommandTemplate>) {
        self.registered_commands.push(command);
    }
    fn run(&mut self) {
        for line in self.run_list.lines() {
            let line = line.trim();
            let detected_command = &line[0..line.find(' ').unwrap_or(line.len())];
            let args = &line[line.find(' ').unwrap_or(line.len())..line.len()].trim_start();

            if detected_command.is_empty() {
                println!("NO COMMAND DETECTED");
            }

            if detected_command == "stop" {
                return;
            }

            for command in &mut self.registered_commands {
                if command.get_name() == detected_command {
                    command.exec(args);
                }
            }
        }
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(Ping {}));
    terminal.register(Box::new(Count {}));
    terminal.register(Box::new(Times { count: 0 }));
    terminal.register(Box::new(New {}));
    terminal.register(Box::new(Bk {}));

    terminal.run();
}
