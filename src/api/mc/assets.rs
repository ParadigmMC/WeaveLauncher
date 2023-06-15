use std::{path::PathBuf, fs};

use anyhow::{Result, Context};
use mcapi::vanilla::{MCAssetIndex, MCAsset};
use tokio::fs::File;

use crate::api::WEAVE;

pub struct AssetManager {
    path: PathBuf,
}

const INDEXES: &str = "indexes";
const OBJECTS: &str = "objects";
#[allow(dead_code)]
const SKINS: &str = "skins";

impl AssetManager {
    pub fn new(path: PathBuf) -> Self {
        AssetManager {
            path,
        }
    }

    pub fn index_exists(&self, version: &str) -> bool {
        self.path.join(INDEXES).join(version.to_owned() + ".json").exists()
    }

    pub async fn download(&self, index: &MCAssetIndex) -> Result<()> {
        if index.map_to_resources {
            todo!();
            //return Ok(())
        }

        for (filename, asset) in index.objects.iter() {
            let res = asset.download(&WEAVE.http_client).await?;
            WEAVE.handle_download(res, self.to_file(asset).await?)
                .await.context(format!("Downloading asset: {filename}"))?;
        }

        Ok(())
    }

    pub fn to_path(&self, asset: &MCAsset) -> PathBuf {
        self.path.join(OBJECTS).join(asset.get_path())
    }

    pub async fn to_file(&self, asset: &MCAsset) -> Result<File> {
        fs::create_dir_all(self.path.join(&asset.hash[0..2])).context("Creating asset folder")?;
        Ok(File::create(self.to_path(asset)).await.context("Creating asset file")?)
    }
}