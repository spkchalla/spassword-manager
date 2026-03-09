mod cli;
mod commands;
mod core;
mod storage;

use cli::{CliCommand, parse_cli};

fn main() {
    let command = parse_cli();
    match command {
        CliCommand::Init =>{
            commands::init::execute();
        }
        CliCommand::Add {service, username, password} =>{
            commands::add::execute(service, username, password);
        }
        CliCommand::List =>{
            commands::list::execute();
        }
    }
}
