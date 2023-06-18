use std::{path::PathBuf, fs};

use anyhow::{Result, Context, bail};

mod instance;
pub use instance::*;

pub struct InstanceManager {
    pub path: PathBuf,

    pub instances: Vec<Instance>,
}

impl InstanceManager {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            instances: Vec::new(),
        }
    }

    pub fn create(&self, name: &str) -> Result<Instance> {
        let path = self.path.join(name);

        if path.exists() {
            bail!("Instance with name {name} already exists");
        }

        fs::create_dir_all(path.join(".minecraft"))?;

        let inst = Instance {
            path,
            name: name.to_owned(),
            ..Default::default()
        };

        inst.save()?;

        Ok(inst)
    }

    pub fn reload(&mut self) -> Result<()> {
        self.instances.clear();

        fs::create_dir_all(&self.path).context("Failed to create missing instances dir")?;
    
        for entry_r in self.path.read_dir()? {
            if let Ok(entry) = entry_r {
                if entry.metadata()?.is_dir() {
                    self.instances.push(Instance::from(entry.path())?);
                }
            }
        }
    
        Ok(())
    }
}
