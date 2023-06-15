use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MCUser {
    Offline {
        name: String,
        uuid: String,
    },
    MsaAccount {
        
    },
}