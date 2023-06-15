
use std::path::PathBuf;

use anyhow::{Result, Context};
use futures::StreamExt;
use tokio::{fs::File, io::BufWriter};
use lazy_static::lazy_static;

use self::{instances::{Instance, load_instances}, mc::{assets::AssetManager, libraries::LibraryManager, versions::VersionManager, clientjar::ClientJarManager}};
use self::java::JavaManager;

pub mod auth;
pub mod instances;
mod mc;
mod java;

lazy_static! {
    pub static ref WEAVE: WeaveAPI = WeaveAPI::init().expect("Couldn't init launcher");
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
    pub instances: Vec<Instance>,
    pub http_client: reqwest::Client,
    pub assets: AssetManager,
    pub libraries: LibraryManager,
    pub versions: VersionManager,
    pub clientjars: ClientJarManager,
    pub java: JavaManager,
}

impl WeaveAPI {
    pub fn init() -> Result<Self> {
        let folder_root = dirs::config_dir().expect("Couldn't get config dir").join(APP_NAME);

        let instances = load_instances(folder_root.join("instances")).context("Loading instances")?;

        let http_client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(WeaveAPI {
            http_client,
            instances,
            assets: AssetManager::new(folder_root.join("assets")),
            libraries: LibraryManager::new(folder_root.join("libraries")),
            versions: VersionManager::new(folder_root.join("versions")),
            clientjars: ClientJarManager::new(folder_root.join("jars")),
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

trait TaskReporter {
    fn begin(&self, name: &str);
    fn progress(&self, perc: u8, all: u8);
    fn end(&self);
}
