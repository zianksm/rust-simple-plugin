use std::{error::Error, path::Path, sync::mpsc::Receiver};

use libloading::{Library, Symbol};

pub struct Loader {
    plugin_dir: String,
    inner_recv: Receiver<String>,
}

impl Loader {
    pub fn new(plugin_dir: String, rx: Receiver<String>) -> Self {
        Self {
            plugin_dir,
            inner_recv: rx,
        }
    }

    fn load(&self, file: &str) -> Result<Library, Box<dyn Error>> {
        let dir = Path::new(&self.plugin_dir);
        let path = dir.join(file);

        unsafe { Ok(libloading::Library::new(path)?) }
    }

    pub fn start(self) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || loop {
            let file = self.inner_recv.recv().unwrap();
            println!("loading: {file}");

            let file = file.replace(".rs", ".dll");

            let lib = self.load(&file).unwrap();

            println!("executing: {file}");
            self.execute(lib);
        })
    }

    fn execute(&self, lib: Library) {
        unsafe {
            let arg = String::from("Alice");
            let _fn: Symbol<fn(arg: String) -> ()> = lib.get(b"test\0").unwrap();
            _fn(arg);

            println!("done\n");
        };
    }
}
