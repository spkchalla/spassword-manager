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
    };
    vault.entries.push(credential);
}
//using the &Vault to just see it, not tamper or edit it.
pub fn list_entries(vault: &Vault) ->&[Credential]{
    &vault.entries
}

pub fn get_entry<'a>(vault: &'a Vault, service: &str) -> Option<&'a Credential> {
    vault.entries.iter().find(|entry| entry.service == service)
}

pub fn delete_entry(vault: &mut Vault, service: &str) -> bool {
    if let Some(pos) = vault.entries.iter().position(|entry| entry.service == service) {
        vault.entries.remove(pos);
        true
    } else {
        false
    }
}
