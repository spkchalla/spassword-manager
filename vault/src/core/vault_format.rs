use crate::core::models::{Credential, Vault};

pub fn createEmptyVault () ->Vault{
    Vault{
        entries: Vec::new(),
    }
}

pub fn addEntry (vault: &mut Vault, service: String, username: String, password: String) -> {
    let credential = Credential{
        service,
        username,
        password,
    };
    vault.entries.push(credential);
}

pub fn listEntries(vault: &Vault) -> &[Credential] {
    &vault.entries
}

pub fn findCredential(vault: Vault, service: String) -> Option<&Credential> {
    vault
        .entries
        .iter()
        .find(|entry| entry.service == service)
}
