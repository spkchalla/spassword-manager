use crate::core::vault_format;
use crate::storage::file_io;

pub fn execute(service: String) {
    let vault = file_io::load_vault().expect("Failed to load vault");

    if let Some(entry) = vault_format::get_entry(&vault, &service) {
        println!("Service: {}", entry.service);
        println!("Username: {}", entry.username);
        println!("Password: {}", entry.password);
    } else {
        eprintln!("Error: No entry found for service '{}'", service);
    }
}
