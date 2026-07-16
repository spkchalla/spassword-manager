use std::time::{PSystemTime, UNIX_EPOCH}
use serde::{Deserialize, Serialize};

use crate::core::models::Vault;

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultMetadata{
    pub version: u32,
    pub created_at: u64,
    pub modified_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultFile{
    pub metadata: VaultMetadata,
    pub vault: Vault,
}

impl VaultFile{
    pub fn new(vault: Vault)-> Self{
        let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

        Self {
            metadata: VaultMetadata{
                version: 1,
                created_at: now,
                modified_at: now,
            },
            vault,
        }
    }

    pub fn touch(&mut self) {
        self.metadata.modified_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        as_secs();
    }
}