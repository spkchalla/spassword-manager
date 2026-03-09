use crate::core::vault_format;
use crate::storage::file_io;

pub fn execute(service: String, username: String, password: String) {
    let mut vault = file_io::load_vault().expect("Failed to load vault");

    vault_format::add_entry(&mut vault, service, username, password);
    file_io::save_vault(&vault).expect("Failed to save vault");
    println!("Entry added successfully");
}
