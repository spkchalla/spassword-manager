mod cli;
mod commands;
mod core;
mod storage;

use cli::{CliCommand, parse_cli}

fn main() {
    let command = parse_cli();
    match command {
        CliCommand::init =>{
            commands::init::execute();
        }
        CliCommand::add {service, username, password} =>{
            commands::add::execute(service, username, password);
        }
        CliCommand::list =>{
            commands::list::execute();
        }
    }
}
