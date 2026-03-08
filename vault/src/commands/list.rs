use crate::storage::file_io;
use crate::core::models::Credential;


pub fn execute(){
    let vault = file_io::load_vault().expect("Failed to load vault");

    for entry in vault.entries().iter(){
        println!(
            "Service: {}, Username: {}",
            entry.service,
            entry.username
            );
    }
}


