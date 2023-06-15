use std::{path::PathBuf, fs::{File, self}, io::Write};

use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};

use super::WEAVE;

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

impl Instance {
    pub fn save(&self) -> Result<()> {
        let cfg_str = toml::to_string_pretty(&self)?;
        let mut f = File::create(&self.path.join(CONFIG_FILE))?;
        f.write_all(cfg_str.as_bytes())?;

        Ok(())
    }

    pub fn from(path: PathBuf) -> Result<Self> {
        let data = fs::read_to_string(&path)?;
        Ok(Self {
            path,
            ..toml::from_str(&data)?
        })
    }

    pub async fn launch(&self) -> Result<()> {
        
        Ok(())
    }

    pub async fn download_all(&self) -> Result<()> {
        let ver = WEAVE.versions.get_info(&self.mc_version).await?;
        WEAVE.versions.download_all(&ver).await
    }
}

pub fn load_instances(
    path: PathBuf
) -> Result<Vec<Instance>> {
    let mut instances: Vec<Instance> = vec![];

    fs::create_dir_all(&path).context("Failed to create missing instances dir")?;

    for entry_r in path.read_dir()? {
        if let Ok(entry) = entry_r {
            if entry.metadata()?.is_dir() {
                instances.push(Instance::from(entry.path())?);
            }
        }
    }

    Ok(instances)
}