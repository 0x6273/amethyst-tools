use std::{
    fs::{create_dir, remove_dir_all},
    path::Path,
    process::Command,
};

use crate::{
    error::{ErrorKind, Result, ResultExt},
    templates,
};

/// Options for the New subcommand. If `version` is None, then it uses
/// the latest version available
#[derive(Clone, Debug)]
pub struct New {
    pub project_name: String,
    pub version: Option<String>,
    pub no_defaults: bool,
}

impl New {
    /// Creates the project template.
    ///
    /// # Errors
    ///
    /// Fails if the project directory already exists, or when failing to create the directory.
    pub fn execute(&self) -> Result<()> {
        self.execute_inner()
            .chain_err(|| ErrorKind::New(self.project_name.clone()))
    }

    fn execute_inner(&self) -> Result<()> {
        let path = Path::new(&self.project_name);
        if path.exists() {
            bail!("project directory {:?} already exists", path);
        }
        create_dir(path).chain_err(|| "could not create project folder")?;

        let mut params = templates::Parameters::new();
        params.insert(
            "project_name".into(),
            templates::Value::scalar(self.project_name.to_owned()),
        );

        if let Err(err) = templates::deploy("main", &self.version, self.no_defaults, path, &params)
        {
            remove_dir_all(path).chain_err(|| "could not clean up project folder")?;
            Err(err)
        } else {
            Command::new("git")
                .arg("init")
                .current_dir(path)
                .spawn()?
                .try_wait()?;
            Ok(())
        }
    }
}

impl Default for New {
    #[must_use]
    fn default() -> Self {
        Self {
            project_name: "game".to_owned(),
            version: None,
            no_defaults: false,
        }
    }
}
