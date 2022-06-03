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

pub fn attribute_id_to_index(id: usize) -> (usize, usize) { (id / 100 - 1, id % 100 / 2) }

impl ComposeFile {
    pub fn attribute_id_to_obj(&self, id: usize) -> (&Executable, &str) {
        let exe = &self.executables[id / 100 - 1];
        if id % 2 == 1 {
            (exe, &exe.inputs[id % 100 / 2])
        } else {
            (exe, &exe.outputs[id % 100 / 2])
        }
    }
}

pub fn index_to_attribute_id(exe_index: usize, port_index: usize) -> usize { (exe_index + 1) * 100 + port_index * 2 }

fn conda_path_default() -> String { "conda".to_string() }