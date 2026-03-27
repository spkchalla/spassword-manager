use std::env;

pub enum CliCommand {
    Init,
    Add {
        service: String,
        username: String,
        password: String,
    },
    List,
    Get {
        service: String,
    },
    Delete {
        service: String,
    },
}

pub fn parse_cli() -> CliCommand {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No command provided");
    }

    match args[1].as_str() {
        "init" => CliCommand::Init,

        "add" => {
            if args.len() != 5 {
                panic!("Usage: vault add <service> <username> <password>");
            }
            CliCommand::Add {
                service: args[2].clone(),
                username: args[3].clone(),
                password: args[4].clone(),
            }
        }
        "list" => CliCommand::List,

        "get" => {
            if args.len() != 3 {
                panic!("Usage: vault get <service>");
            }
            CliCommand::Get {
                service: args[2].clone(),
            }
        }
        "delete" => {
            if args.len() != 3 {
                panic!("Usage: vault delete <service>");
            }
            CliCommand::Delete {
                service: args[2].clone(),
            }
        }

        _ => panic!("Unknown command"),
    }
}
