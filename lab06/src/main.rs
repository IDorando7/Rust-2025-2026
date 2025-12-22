use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

trait Command {
    fn get_name(&self) -> &'static str;
    fn exec(&mut self, args: &[&str]);
}

struct PingCommand;
impl Command for PingCommand {
    fn get_name(&self) -> &'static str {
        "ping"
    }
    fn exec(&mut self, _args: &[&str]) {
        println!("pong!");
    }
}

struct CountCommand;
impl Command for CountCommand {
    fn get_name(&self) -> &'static str {
        "count"
    }
    fn exec(&mut self, args: &[&str]) {
        println!("counted {} args", args.len());
    }
}

struct TimesCommand {
    count: u64,
}
impl Command for TimesCommand {
    fn get_name(&self) -> &'static str {
        "times"
    }
    fn exec(&mut self, _args: &[&str]) {
        self.count += 1;
        println!("{}", self.count);
    }
}

struct EchoCommand;
impl Command for EchoCommand {
    fn get_name(&self) -> &'static str {
        "echo"
    }
    fn exec(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!();
        } else {
            println!("{}", args.join(" "));
        }
    }
}

struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    fn register(&mut self, cmd: Box<dyn Command>) {
        self.commands.push(cmd);
    }

    fn run(&mut self, file_path: &str) {
        let file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("error: could not open '{}': {}", file_path, e);
                return;
            }
        };

        let reader = BufReader::new(file);
        for (lineno, line_res) in reader.lines().enumerate() {
            let line = match line_res {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("error: failed to read line {}: {}", lineno + 1, e);
                    continue;
                }
            };

            let mut parts = line.split_whitespace();
            let cmd_raw = match parts.next() {
                Some(name) => name,
                None => continue,
            };

            if cmd_raw.eq("stop") {
                return;
            }
            if cmd_raw.eq_ignore_ascii_case("stop") && cmd_raw != "stop" {
                eprintln!("unknown command '{}'. Did you mean 'stop'?", cmd_raw);
                continue;
            }

            let args_vec: Vec<&str> = parts.collect();

            let mut found = false;
            for cmd in self.commands.iter_mut() {
                if cmd.get_name() == cmd_raw {
                    cmd.exec(&args_vec);
                    found = true;
                    break;
                }
            }

            if !found {
                let mut suggestion: Option<&'static str> = None;
                for cmd in self.commands.iter() {
                    if cmd.get_name().eq_ignore_ascii_case(cmd_raw) {
                        suggestion = Some(cmd.get_name());
                        break;
                    }
                }

                if let Some(correct) = suggestion {
                    eprintln!("unknown command '{}'. Did you mean '{}'?", cmd_raw, correct);
                } else {
                    eprintln!("unknown command '{}'", cmd_raw);
                }
            }
        }
    }
}

fn main() {
    let file_path = match env::args().nth(1) {
        Some(s) => s,
        None => String::from("commands.txt"),
    };

    let mut terminal = Terminal::new();
            terminal.register(Box::new(PingCommand));
    terminal.register(Box::new(CountCommand));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(EchoCommand));

    terminal.run(&file_path);
}
