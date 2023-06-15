use std::{process::Command, collections::HashMap};

use anyhow::{Result, Context};
use mcapi::vanilla::{PistonRuleMatcher, VersionInfo};

use crate::api::{WEAVE, instances::Instance, auth::MCUser, java::JavaInstallation, APP_NAME, APP_VERSION};

pub struct LaunchContext<'a> {
    pub matcher: PistonRuleMatcher,
    pub version: VersionInfo,
    pub instance: &'a Instance,
    pub user: &'a MCUser,
    pub java: &'a JavaInstallation,
}

impl<'a> LaunchContext<'a> {
    pub async fn launch(&self) -> Result<()> {
        let mut map = HashMap::new();

        map.insert("launcher_name".to_owned(), APP_NAME.to_owned());
        map.insert("launcher_version".to_owned(), APP_VERSION.to_owned());

        map.insert("version_name".to_owned(), self.version.id.to_owned());
        map.insert("natives_directory".to_owned(), WEAVE.libraries.path.to_str().unwrap().to_owned());
        map.insert("game_directory".to_owned(), self.instance.path.to_str().unwrap().to_owned());
        map.insert("assets_root".to_owned(), WEAVE.assets.path.to_str().unwrap().to_owned());
        map.insert("assets_index_name".to_owned(), WEAVE.assets.get_index_path(&self.version.id).to_str().unwrap().to_owned());

        let jvm_args = self.matcher.build_args(&self.version.arguments.jvm, &map);
        let game_args = self.matcher.build_args(&self.version.arguments.game, &map);

        let child = Command::new("java")
            .args(jvm_args)
            .arg(&self.version.main_class)
            .args(game_args)
            .spawn().context("Starting java process")?;

        Ok(())
    }
}
