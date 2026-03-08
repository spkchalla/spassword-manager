use crate::core::models::{Credential, Vault};

pub fn create_empty_vault()-> Vault{
    Vault{
        entries: Vec::new(),
    }

}
// using the &mut here to edit the vault
pub fn add_entry(vault: &mut Vault, service: String, username: String, password: String) {
    let credential = Credential{
        service,
        username,
        password,
    }
    vault.entries.push(credential);
}
//using the &Vault to just see it, not tamper or edit it.
pub fn list_entries(vault: &Vault) ->&[Credential]{
    &vault.entries
}
