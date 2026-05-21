use std::{
    collections::HashMap,
    io::{self, Write},
};
struct CmdFn {
    function: fn(map: &HashMap<&str, CmdFn>, args: &str) -> bool,
    description: String,
}

fn main() -> io::Result<()> {
    let mut cmd_fn = HashMap::new();

    cmd_fn.insert(
        "exit",
        CmdFn {
            function: |_, _| -> bool { false },
            description: String::from("exit is a shell builtin"),
        },
    );
    cmd_fn.insert(
        "echo",
        CmdFn {
            function: |_, args| -> bool {
                if args.is_empty() {
                    println!("Usage: echo <args>");
                } else {
                    println!("{}", args);
                }
                true
            },
            description: String::from("echo is a shell builtin"),
        },
    );
    cmd_fn.insert(
        "type",
        CmdFn {
            function: |cmd_fn, args| -> bool {
                match cmd_fn.get(args) {
                    Some(k) => println!("{}", k.description),
                    None => println!("invalid_command: not found"),
                }
                true
            },
            description: String::from("type is a shell builtin"),
        },
    );

    loop {
        print!(">> ");
        io::stdout().flush()?;
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("unable to read user input");

        let trimmed_input = input.trim();

        if trimmed_input.is_empty() {
            continue;
        }

        let (command, args) = trimmed_input
            .split_once(" ")
            .unwrap_or_else(|| (trimmed_input, ""));

        let should_continue = exec_command(&cmd_fn, command, args);

        if !should_continue {
            return Ok(());
        }
    }
}

fn exec_command(cmd_fn: &HashMap<&str, CmdFn>, command: &str, args: &str) -> bool {
    match cmd_fn.get(command) {
        Some(v) => (v.function)(cmd_fn, args),
        None => {
            println!("Command not found {}", command);
            true
        }
    }
}
