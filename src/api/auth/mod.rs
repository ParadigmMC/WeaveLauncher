use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MCAccount {
    Offline {
        name: String,
        uuid: String,
    },
    MsaAccount {
        
    },
}