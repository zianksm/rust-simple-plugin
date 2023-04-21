use std::{error::Error, path::Path};

use libloading::{Library, Symbol};

pub struct Loader {
    plugin_dir: String,
}

impl Loader {
    pub fn new(plugin_dir: String) -> Self {
        Self { plugin_dir }
    }

    pub fn load(&self, file: &str) -> Result<Library, Box<dyn Error>> {
        let dir = Path::new(&self.plugin_dir);
        let path = dir.join(file);
        
        unsafe { Ok(libloading::Library::new(path)?) }
    }
}
