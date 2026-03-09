use crate::storage::file_io;
use crate::core::vault_format;


pub fn execute(){
    let vault = file_io::load_vault().expect("Failed to load vault");

    for entry in vault_format::list_entries(&vault){
        println!(
            "Service: {}, Username: {}",
            entry.service,
            entry.username
            );
    }
}


