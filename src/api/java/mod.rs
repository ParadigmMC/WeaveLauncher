use std::{path::PathBuf, ffi::OsString};

use anyhow::Result;

pub struct JavaManager {
    pub installations: Vec<JavaInstallation>,
}

impl JavaManager {
    pub fn init() -> Self {
        Self {
            installations: vec![],
        }
    }

    pub async fn search(&mut self) -> Result<()> {
        self.installations.clear();

        self.installations.append(&mut self.find_from_path().await?);

        Ok(())
    }

    pub async fn find_from_path(&self) -> Result<Vec<JavaInstallation>> {
        let mut list: Vec<JavaInstallation> = Vec::new();

        for path in pathsearch::PathSearcher::new(match std::env::consts::OS {
            "windows" => "javaw",
            _ => "java",
        }, std::env::var_os("PATH").as_ref().map(OsString::as_os_str),
            std::env::var_os("PATHEXT").as_ref().map(OsString::as_os_str)) {
            list.push(JavaInstallation::from(&path));
        }

        Ok(list)
    }
}

pub struct JavaInstallation {
    path: PathBuf,
}

impl JavaInstallation {
    pub fn from(path: &PathBuf) -> Self {
        Self {
            path: path.to_owned(),
        }
    }
}