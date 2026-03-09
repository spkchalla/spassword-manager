use crate::core::vault_format;
use crate:: storage::file_io;

pub fn execute(){
    let vault = vault_format::create_empty_vault();
    file_io::save_vault(&vault).expect("Failed to initialize a vault");
    println!("Vault initialized successfully");
}
