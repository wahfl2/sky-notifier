use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    // pub name: Option<String>,
    pub id: Option<String>,
    pub error_message: Option<String>,
    // pub path: Option<String>,
}