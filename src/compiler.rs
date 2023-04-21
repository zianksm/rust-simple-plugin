use std::{error::Error, path::Path};

pub struct Compiler {
    plugin_dir: String,
}

impl Compiler {
    pub fn new(plugin_dir: &str) -> Self {
        Self {
            plugin_dir: plugin_dir.to_string(),
        }
    }

    pub fn compile(&self, file: &str) -> Result<bool, Box<dyn Error>> {
        let dir = Path::new(&self.plugin_dir);

        let file = dir.join(file);
        let file = file.to_str().unwrap();

        let dir = dir.to_str().unwrap();

        let compile = std::process::Command::new("rustc")
            .arg("--crate-type")
            .arg("cdylib")
            .arg(file)
            .arg("--out-dir")
            .arg(dir)
            .status()?;

        Ok(true)
    }
}
