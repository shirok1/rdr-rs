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

use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use new_string_template::template::Template;
use serde_derive::{Deserialize, Serialize};

impl Executable {
    fn run(&self, values: &HashMap<&str, String>) -> Result<Child, Box<dyn Error>> {
        let render_string = Template::new(&self.arg_template).render(values)?;
        let render_args = render_string.split(' ');
        match &self.exe_type {
            ExecutableType::PythonScript(env) => {
                env.spawn_run(|x| { x.args(render_args) })
            }
            ExecutableType::CustomExecutable => {
                let mut command = Command::new(&self.path);
                command.args(render_args);
                command.stdout(Stdio::piped())
                    .stderr(Stdio::piped());
                command.spawn().map_err(|e| e.into())
            }
            _ => todo!("Not implemented"),
        }
    }

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

mod tokio_cmd {
    use std::collections::HashMap;
    use std::error::Error;
    use std::path::PathBuf;
    use new_string_template::template::Template;
    use tokio::process::{Child, Command};
    use crate::Executable;
    use crate::model::{ExecutableType, PythonEnvironment};

    impl Executable {
        async fn run_async(&self, values: &HashMap<&str, String>) -> Result<Child, Box<dyn Error>> {
            match self.exe_type {
                // ExecutableType::PythonScript(_) => {
                //     Ok(())
                // }
                ExecutableType::CustomExecutable => {
                    let mut command = Command::new(&self.path);

                    let render_string = Template::new(&self.arg_template).render(values)?;
                    let render_args = render_string.split(' ');

                    command.args(render_args);

                    command.spawn().map_err(|e| e.into())
                }
                _ => todo!("Not implemented"),
            }
        }
    }


    impl PythonEnvironment {
        pub async fn run_sth_async(&self, things: impl FnOnce(&mut Command) -> &mut Command) -> Result<String, Box<dyn Error>> {
            use PythonEnvironment::*;
            let output = match self {
                Conda { conda_path, conda_env: env_path } =>
                    things(Command::new(conda_path)
                        .args(["run", "-p", env_path, "python"]))
                        .output().await?.stdout,
                VirtualEnv { venv } =>
                    things(Command::new([venv, "bin", "python3"].iter().collect::<PathBuf>())
                        .env("VIRTUAL_ENV", venv)).output().await?.stdout,
                System =>
                    things(&mut Command::new("python3"))
                        .output().await?.stdout,
            };
            Ok(String::from_utf8(output)?)
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

use std::process::{Child, Command, Stdio};

impl PythonEnvironment {
    pub fn spawn_run(&self, things: impl FnOnce(&mut Command) -> &mut Command) -> Result<Child, Box<dyn Error>> {
        use PythonEnvironment::*;
        let mut cmd = match self {
            Conda { conda_path, conda_env: env_path } => {
                let mut cmd = Command::new(conda_path);
                cmd.args(["run", "-p", env_path, "python"]);
                cmd
            }
            VirtualEnv { venv } => {
                let mut cmd = Command::new([venv, "bin", "python3"].iter().collect::<PathBuf>());
                cmd.env("VIRTUAL_ENV", venv);
                cmd
            }
            System => Command::new("python3"),
        };
        cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped());
        things(&mut cmd);
        cmd.spawn().map_err(|e| e.into())
    }
    pub fn run_and_get_result(&self, things: impl FnOnce(&mut Command) -> &mut Command) -> Result<String, Box<dyn Error>> {
        let child = self.spawn_run(things)?;
        let output = child.wait_with_output()?;
        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?)
        } else { Err(String::from_utf8(output.stderr)?.into()) }
    }
    pub fn get_py_version(&self) -> Result<String, Box<dyn Error>> {
        self.run_and_get_result(|cmd| { cmd.arg("-VV") })
    }
    pub fn get_pip_packages(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let list_str = self.run_and_get_result(|cmd| { cmd.args(["-m", "pip", "list", "--format=freeze"]) })?;
        Ok(list_str.split('\n').map(|s| s.to_string()).collect())
    }
}
