use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Executable {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(default)]
    pub outputs: Vec<String>,
    #[serde(default)]
    pub arg_template: String,
    #[serde(default)]
    pub env_template: HashMap<String, String>,
    #[serde(default = "ExecutableType::default")]
    pub exe_type: ExecutableType,
}

impl Executable {
    pub fn default() -> Self {
        Self {
            name: "未命名".to_string(),
            path: "./untitled".to_string(),
            arg_template: "".to_string(),
            env_template: HashMap::new(),
            exe_type: ExecutableType::CustomExecutable,
            inputs: vec![],
            outputs: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum ExecutableType {
    CustomExecutable,
    PythonScript(PythonEnvironment),
}

impl ExecutableType {
    pub fn default() -> Self {
        ExecutableType::CustomExecutable
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "python_env_type")]
// #[serde(untagged)]
pub enum PythonEnvironment {
    Conda { conda_path: String, conda_env: String },
    VirtualEnv { venv: String },
    System,
}

mod tokio;
