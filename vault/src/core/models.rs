use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]


pub struct Credential {
    pub service: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct Vault{
    pub entries: Vec<Credential>,
}
