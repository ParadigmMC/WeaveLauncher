use std::{path::PathBuf, fs::{File, self}, io::Write};

use anyhow::Result;
use serde::{Serialize, Deserialize};

use crate::api::WEAVE;

pub const CONFIG_FILE: &str = "instance.json";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Instance {
    #[serde(skip)]
    pub path: PathBuf,

    #[serde(skip)]
    pub is_active: bool,

    pub name: String,
    pub mc_version: String,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            is_active: false,
            mc_version: "".to_string(),
            name: "".to_string(),
        }
    }
}

impl Instance {
    pub fn save(&self) -> Result<()> {
        let cfg_str = toml::to_string_pretty(&self)?;
        let mut f = File::create(&self.path.join(CONFIG_FILE))?;
        f.write_all(cfg_str.as_bytes())?;

        Ok(())
    }

    pub fn from(path: PathBuf) -> Result<Self> {
        let data = fs::read_to_string(&path.join(CONFIG_FILE))?;
        Ok(Self {
            path,
            ..toml::from_str(&data)?
        })
    }

    pub async fn launch(&self) -> Result<()> {
        
        Ok(())
    }

    pub async fn download_all(&self) -> Result<()> {
        let weave = WEAVE.read().await;
        let ver = weave.versions.get_info(&self.mc_version).await?;
        weave.versions.download_all(&ver).await
    }
}