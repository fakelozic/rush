use std::{
    collections::HashMap,
    env::{self, VarError},
    fs,
    io::{self, Write},
    os::unix::fs::PermissionsExt,
    path::Path,
    process::Command,
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
                    println!("{}", use_quotes(args));
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
                if args.is_empty() {
                    println!("Usage: type <args>");
                } else {
                    match cmd_fn.get(args) {
                        Some(k) => println!("{}", k.description),
                        None => print_path(&use_quotes(args)),
                    }
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

        let (command, args) = trimmed_input.split_once(" ").unwrap_or((trimmed_input, ""));

        let should_continue = exec_command(&cmd_fn, command, args);

        if !should_continue {
            return Ok(());
        }
    }
}

fn exec_command(cmd_fn: &HashMap<&str, CmdFn>, cmd: &str, args: &str) -> bool {
    match cmd_fn.get(cmd) {
        Some(v) => (v.function)(cmd_fn, args),
        None => {
            let (path, err) = check_path(cmd);
            match err {
                Some(e) => println!("Error accessing path: {}", e),
                None => match path {
                    Some(_) => {
                        let parsed_args = use_quotes(args);
                        let arg_vec: Vec<&str> = parsed_args.split_whitespace().collect();
                        let mut child = Command::new(cmd)
                            .args(arg_vec)
                            .spawn()
                            .expect("Failed to execute command");

                        child.wait().expect("Failed to wait on child");
                    }
                    None => println!("{}: not found", cmd),
                },
            };
            true
        }
    }
}

fn check_path(cmd: &str) -> (Option<String>, Option<VarError>) {
    let path = env::var("PATH");
    match path {
        Ok(val) => {
            for dir in val.split(":") {
                let path_buf = Path::new(dir).join(cmd);
                if let Ok(metadata) = fs::metadata(&path_buf) {
                    let permissions = metadata.permissions();
                    let is_executable = permissions.mode() & 0b01001001 != 0;
                    if metadata.is_file() && is_executable {
                        return (Some(path_buf.display().to_string()), None);
                    }
                }
            }
            (None, None)
        }
        Err(err) => (None, Some(err)),
    }
}

fn print_path(cmd: &str) {
    let (path, err) = check_path(cmd);
    match err {
        Some(e) => println!("Error accessing path: {}", e),
        None => match path {
            Some(p) => println!("{} is {}", cmd, p),
            None => println!("{}: not found", cmd),
        },
    }
}

fn use_quotes(args: &str) -> String {
    args.trim()
        .split('\'')
        .filter(|x| !x.is_empty())
        .map(|x| if x.trim().is_empty() { " " } else { x })
        .collect::<String>()
}

#[test]
fn use_quotes_test() {
    assert_eq!(use_quotes("'hello world'"), String::from("hello world"));
    assert_eq!(
        use_quotes("    'hello world'    "),
        String::from("hello world")
    );
    assert_eq!(use_quotes("'hello''world'"), String::from("helloworld"));
    assert_eq!(use_quotes("'hello' 'world'"), String::from("hello world"));
    assert_eq!(use_quotes("'hello'  'world'"), String::from("hello world"));
    assert_eq!(use_quotes("hello''world"), String::from("helloworld"));
}
