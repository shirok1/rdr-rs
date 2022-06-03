use serde_derive::{Deserialize, Serialize};
use crate::model::Executable;

pub mod model;

#[derive(Serialize, Deserialize, Debug)]
pub struct ComposeFile {
    pub version: String,
    #[serde(default = "conda_path_default")]
    pub conda_path: String,
    #[serde(default)]
    pub links: Vec<(usize, usize)>,
    #[serde(default)]
    pub executables: Vec<Executable>,
}

fn conda_path_default() -> String { "conda".to_string() }