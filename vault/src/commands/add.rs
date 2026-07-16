use crate::core::vault_format;
use crate::storage::file_io;

pub fn execute(service: String, username: String, password: String) {
    let mut vault_file = file_io::load_vault().expect("Failed to load vault");

    vault_format::add_entry(&mut vault_file.vault, service, username, password);
    vault_file.touch();
    file_io::save_vault(&vault_file).expect("Failed to save vault");
    println!("Entry added successfully");
}
