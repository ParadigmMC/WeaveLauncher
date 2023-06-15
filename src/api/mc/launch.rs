use anyhow::Result;

use crate::api::WEAVE;

pub async fn launch_demo() -> Result<()> {
    let ver = WEAVE.versions.get_info("1.20.1").await?;
    WEAVE.versions.download_all(&ver).await?;

    let file = WEAVE.clientjars.to_path("1.20.1");

    

    Ok(())
}