use std::{path::PathBuf, fs};

use anyhow::{Result, Context};
use mcapi::vanilla::VersionInfo;
use tokio::fs::File;

use crate::api::WEAVE;

pub struct ClientJarManager {
    path: PathBuf,
}

impl ClientJarManager {
    pub fn new(path: PathBuf) -> Self {
        ClientJarManager {
            path,
        }
    }

    pub fn jar_exists(&self, version: &str) -> bool {
        self.to_path(version).exists()
    }

    pub async fn download_jar_from_version(&self, version: &str) -> Result<()> {
        let list = mcapi::vanilla::fetch_version_manifest(&WEAVE.http_client).await?;

        let ver = list.fetch(version, &WEAVE.http_client).await?;

        self.download_jar(&ver).await
    }

    pub async fn download_jar(&self, ver: &VersionInfo) -> Result<()> {
        let res = ver.downloads.client.download(&WEAVE.http_client).await?;
        WEAVE.handle_download(res, self.to_file(&ver.id).await?).await?;
        Ok(())
    }

    /// Returns the path of the client jar for version
    pub fn to_path(&self, version: &str) -> PathBuf {
        self.path.join("client-".to_owned() + version + ".jar")
    }

    pub async fn to_file(&self, version: &str) -> Result<File> {
        let path = self.to_path(version);
        fs::create_dir_all(path.parent().unwrap()).context("Creating folder for client jar")?;
        Ok(File::create(path).await.context("Creating client jar file")?)
    }
}