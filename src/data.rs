use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub name: String,
    pub xp: i64,
}

impl Data {
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let bytes = fs::read(path)?;
        let loaded = bincode::deserialize(&bytes)?;

        Ok(loaded)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let bytes = bincode::serialize(self)?;
        fs::write(path, bytes)?;

        Ok(())
    }
}
