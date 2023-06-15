use std::{path::PathBuf, fs::{File, self}, io::Write};

use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Instance {
    #[serde(skip)]
    pub folder_name: String,
    //pub group: String,

    pub name: String,
}

impl Instance {
    pub fn save(&self) -> Result<()> {
        let cfg_str = toml::to_string_pretty(&self)?;
        let mut f = File::create(&self.folder_name)?;
        f.write_all(cfg_str.as_bytes())?;

        Ok(())
    }

    pub fn from(path: PathBuf) -> Result<Self> {
        let data = fs::read_to_string(path)?;
        Ok(toml::from_str(&data)?)
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