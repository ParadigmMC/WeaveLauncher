use std::{path::PathBuf, fs};

use anyhow::{Result, Context};
use mcapi::vanilla::{VersionInfo, PistonRuleMatcher};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::api::WEAVE;

pub struct VersionManager {
    path: PathBuf,
}

const VERSIONS: &str = "versions";

impl VersionManager {
    pub fn new(path: PathBuf) -> Self {
        VersionManager {
            path,
        }
    }

    pub async fn get_info(&self, version: &str) -> Result<VersionInfo> {
        if self.has_info(version).await {
            Ok(self.read_info(version).await?)
        } else {
            let ver = self.fetch_info(version).await?;
            self.save_info(&ver).await?;
            Ok(ver)
        }
    }
    
    pub async fn has_info(&self, version: &str) -> bool {
        self.info_path(version).exists()
    }

    pub async fn read_info(&self, version: &str) -> Result<VersionInfo> {
        let data = fs::read_to_string(self.info_path(version))?;
        Ok(serde_json::from_str(&data)?)
    }

    pub async fn fetch_info(&self, version: &str) -> Result<VersionInfo> {
        let list = mcapi::vanilla::fetch_version_manifest(&WEAVE.http_client).await?;
        let ver = list.fetch(version, &WEAVE.http_client).await?;
        Ok(ver)
    }

    pub async fn save_info(&self, ver: &VersionInfo) -> Result<()> {
        let str = serde_json::to_string_pretty(ver)?;
        let mut f = File::create(self.info_path(&ver.id)).await?;
        f.write_all(str.as_bytes()).await?;

        Ok(())
    }

    pub fn info_path(&self, version: &str) -> PathBuf {
        self.path.join(VERSIONS).join(version.to_owned() + ".json")
    }

    pub async fn info_file(&self, version: &str) -> Result<File> {
        let path = self.info_path(version);
        fs::create_dir_all(path.parent().unwrap()).context("Creating folder for version info")?;
        Ok(File::create(path).await.context("Creating version info file")?)
    }

    pub async fn download_all(&self, ver: &VersionInfo) -> Result<()> {
        let matcher = PistonRuleMatcher::new()?;

        // TODO: Tasks, reporting, subtasks, logging etc
        WEAVE.clientjars.download_jar(&ver).await?;
        WEAVE.assets.download(&ver.fetch_asset_index(&WEAVE.http_client).await?).await?;
        WEAVE.libraries.download_all(&matcher, &ver).await?;

        Ok(())
    }
}