use crate::core::vault_format;
use crate::storage::file_io;

pub fn execute(service: String) {
    let mut vault_file = file_io::load_vault().expect("Failed to load vault");

    if vault_format::delete_entry(&mut vault_file.vault, &service) {
        vault_file.touch();
        file_io::save_vault(&vault_file).expect("Failed to save vault");
        println!("Entry for service '{}' deleted successfully", service);
    } else {
        eprintln!("Error: No entry found for service '{}'", service);
    }
}
