use std::process::Child;
use std::{process::Command, collections::HashMap};

use anyhow::{Result, Context};
use mcapi::vanilla::{PistonRuleMatcher, VersionInfo};

use crate::api::{WEAVE, instances::Instance, auth::MCUser, java::JavaInstallation, APP_NAME, APP_VERSION};
use crate::util::PathBufExt;

pub struct LaunchContext<'a> {
    pub matcher: PistonRuleMatcher,
    pub version: VersionInfo,
    pub instance: &'a Instance,
    pub user: &'a MCUser,
    pub java: &'a JavaInstallation,
}

impl<'a> LaunchContext<'a> {
    pub async fn launch(&self) -> Result<Child> {
        let weave = WEAVE.read().await;

        let mut map = HashMap::new();

        map.insert("launcher_name".to_owned(), APP_NAME.to_owned());
        map.insert("launcher_version".to_owned(), APP_VERSION.to_owned());

        map.insert("version_name".to_owned(), self.version.id.to_owned());
        map.insert("natives_directory".to_owned(), weave.libraries.path.unwrap_to_string());
        map.insert("game_directory".to_owned(), self.instance.path.join(".minecraft").unwrap_to_string());
        map.insert("assets_root".to_owned(), weave.assets.path.unwrap_to_string());
        map.insert("assets_index_name".to_owned(), weave.assets.get_index_path(&self.version.id).unwrap_to_string());

        let jvm_args = self.matcher.build_args(&self.version.arguments.jvm, &map);
        let game_args = self.matcher.build_args(&self.version.arguments.game, &map);

        let child = Command::new("java")
            .args(jvm_args)
            .arg(&self.version.main_class)
            .args(game_args)
            .spawn().context("Starting java process")?;

        Ok(child)
    }
}
