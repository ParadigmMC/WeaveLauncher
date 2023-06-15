use std::{path::PathBuf, fs};

use anyhow::{Result, Context};
use mcapi::vanilla::{PistonLibrary, PistonRuleMatcher, PistonFile, VersionInfo};
use tokio::fs::File;

use crate::api::WEAVE;

pub struct LibraryManager {
    path: PathBuf,
}

impl LibraryManager {
    pub fn new(path: PathBuf) -> Self {
        LibraryManager {
            path,
        }
    }

    pub async fn download_all(
        &self,
        matcher: &PistonRuleMatcher,
        ver: &VersionInfo
    ) -> Result<()> {
        for lib in ver.libraries.iter() {
            self.download_library(matcher, lib).await?;
        }

        Ok(())
    }

    pub async fn download_library(
        &self,
        matcher: &PistonRuleMatcher,
        lib: &PistonLibrary
    ) -> Result<()> {
        if matcher.should_download_library(lib) {
            let res = lib.download_artifact(&WEAVE.http_client).await?;
            WEAVE.handle_download(res, self.create_file_artifact(lib).await?)
                .await.context("Downloading library")?;

            if let Some(native) = &matcher.get_native_library(lib) {
                let res = native.download(&WEAVE.http_client).await?;
                WEAVE.handle_download(res, self.create_file_native(native).await?)
                    .await.context("Downloading library")?;
            }
        }

        Ok(())
    }

    async fn create_file_artifact(&self, lib: &PistonLibrary) -> Result<File> {
        let path = lib.get_artifact_path();
        fs::create_dir_all(self.path.join(PathBuf::from(&path).parent().unwrap()))
            .context("Creating folder for library")?;
        Ok(File::create(path).await.context("Creating library file")?)
    }
    
    async fn create_file_native(&self, f: &PistonFile) -> Result<File> {
        let path = f.path.as_ref().unwrap();
        fs::create_dir_all(self.path.join(PathBuf::from(path).parent().unwrap()))
            .context("Creating folder for library")?;
        Ok(File::create(path).await.context("Creating library file")?)
    }
}