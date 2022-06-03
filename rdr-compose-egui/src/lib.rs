pub mod python_management {
    use std::error::Error;

    pub fn get_conda_envs(conda_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
        use std::process::Command;
        use serde_derive::Deserialize;

        #[derive(Deserialize)]
        struct CondaEnvListResult {
            envs: Vec<String>,
        }

        let output = Command::new(conda_path).args(["env", "list", "--json"]).output()?;
        let res = serde_json::from_slice::<CondaEnvListResult>(&output.stdout)?;
        Ok(res.envs)
    }
}