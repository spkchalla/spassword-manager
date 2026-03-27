use std::fs;
use std::io;

use crate::core::models::Vault;

const VAULT_FILE: &str = "vault.json";

pub fn load_vault() -> io::Result<Vault> {
    let data = fs::read_to_string(VAULT_FILE)?;
    let vault: Vault = serde_json::from_str(&data).expect("Failed to parse vault");

    Ok(vault)
}

pub fn save_vault(vault: &Vault) -> io::Result<()> {
    let data = serde_json::to_string_pretty(vault).expect("Failed to serialize vault");

    fs::write(VAULT_FILE, data)?;

    Ok(())
}
