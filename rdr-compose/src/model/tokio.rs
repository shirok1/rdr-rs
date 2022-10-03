use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::process::Stdio;
use new_string_template::template::Template;
use tokio::process::{Child, Command};
use crate::Executable;
use crate::model::{ExecutableType, PythonEnvironment};

impl Executable {
    async fn run_async(&self, values: &HashMap<&str, String>) -> Result<Child, Box<dyn Error>> {
        match self.exe_type {
            // ExecutableType::PythonScript(_) => {
            //             // env.spawn_run(|x| { x.args(render_args) })
            // }
            ExecutableType::CustomExecutable => {
                let mut command = Command::new(&self.path);

                let render_string = Template::new(&self.arg_template).render(values)?;
                let render_args = render_string.split(' ');

                command.args(render_args).stdout(Stdio::piped())
                    .stderr(Stdio::piped()).spawn().map_err(|e| e.into())
            }
            _ => todo!("Not implemented"),
        }
    }
}

impl PythonEnvironment {
    pub async fn spawn_run_async(&self, things: impl FnOnce(&mut Command) -> &mut Command) -> Result<Child, Box<dyn Error>> {
        let mut cmd = self.get_command();
        cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped());
        things(&mut cmd);
        cmd.spawn().map_err(|e| e.into())
    }
    pub fn dry_run(&self, things: impl FnOnce(&mut Command) -> &mut Command) -> String {
        let mut cmd = self.get_command();
        things(&mut cmd);

        format!("{:?}", cmd)
    }

    fn get_command(&self) -> Command {
        use PythonEnvironment::*;
        match self {
            Conda { conda_path, conda_env: env_path } => {
                let mut command = Command::new(conda_path);
                command.args(["run", "-p", env_path, "python"]);
                command
            }
            VirtualEnv { venv } => {
                let mut command = Command::new([venv, "bin", "python3"].iter().collect::<PathBuf>());
                command.env("VIRTUAL_ENV", venv);
                command
            }
            System => Command::new("python3"),
        }
    }

    pub async fn run_and_get_result_async(&self, things: impl FnOnce(&mut Command) -> &mut Command) -> Result<String, Box<dyn Error>> {
        let child = self.spawn_run_async(things).await?;
        let output = child.wait_with_output().await?;
        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?)
        } else { Err(String::from_utf8(output.stderr)?.into()) }
    }

    pub async fn get_py_version(&self) -> Result<String, Box<dyn Error>> {
        self.run_and_get_result_async(|cmd| { cmd.arg("-VV") }).await
    }

    pub async fn get_pip_packages(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let list_str = self.run_and_get_result_async(|cmd| { cmd.args(["-m", "pip", "list", "--format=freeze"]) }).await?;
        Ok(list_str.split('\n').map(|s| s.to_string()).collect())
    }
}
