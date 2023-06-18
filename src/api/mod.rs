
use std::{path::PathBuf, sync::Arc};

use anyhow::{Result, Context};
use futures::StreamExt;
use tokio::{fs::File, io::BufWriter, sync::RwLock};
use lazy_static::lazy_static;

use self::{instances::InstanceManager, mc::{assets::AssetManager, libraries::LibraryManager, versions::VersionManager, clientjar::ClientJarManager}};
use self::java::JavaManager;

pub mod auth;
pub mod instances;
//pub mod tasks;
mod mc;
pub use mc::launch;
mod java;

lazy_static! {
    pub static ref WEAVE: Arc<RwLock<WeaveAPI>> = {
        Arc::new(RwLock::new(WeaveAPI::init().expect("Couldn't init launcher")))
    };
}

pub const APP_NAME: &str = "WeaveLauncher";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_USER_AGENT: &str = concat!(
    "ParadigmMC/WeaveLauncher/",
    env!("CARGO_PKG_VERSION"),
    " - ",
    env!("CARGO_PKG_REPOSITORY"),
);

pub struct WeaveAPI {
    pub folder_root: PathBuf,
    pub http_client: reqwest::Client,

    pub instances: InstanceManager,

    pub versions: VersionManager,
    pub clientjars: ClientJarManager,
    pub libraries: LibraryManager,
    pub assets: AssetManager,

    pub java: JavaManager,
}

impl WeaveAPI {
    pub fn init() -> Result<Self> {
        let folder_root = dirs::config_dir().expect("Couldn't get config dir").join(APP_NAME);

        let http_client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(WeaveAPI {
            http_client,
            instances: InstanceManager::new(folder_root.join("instances")),
            assets: AssetManager::new(folder_root.join("mc").join("assets")),
            libraries: LibraryManager::new(folder_root.join("mc").join("libraries")),
            versions: VersionManager::new(folder_root.join("mc").join("versions")),
            clientjars: ClientJarManager::new(folder_root.join("mc").join("jars")),
            java: JavaManager::init(),
            folder_root,
        })
    }

    pub async fn handle_download(&self, response: reqwest::Response, file: File) -> Result<()> {

        let mut writer = BufWriter::new(file);
        #[allow(unused)]
        let mut bytes_downloaded = 0;

        let mut stream = response.bytes_stream();
        while let Some(item) = stream.next().await {
            let item = item?;
            tokio::io::copy(&mut item.as_ref(), &mut writer).await?;

            bytes_downloaded += item.len();
            //progress_bar.set_position(bytes_downloaded as u64);
        }
        Ok(())
    }
}
