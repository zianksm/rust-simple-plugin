use std::{
    error::Error,
    os::windows::thread,
    path::Path,
    sync::mpsc::{channel, Receiver, Sender},
};

pub struct Compiler {
    plugin_dir: String,
    inner_recv: Receiver<String>,
    loader_send_channel: Sender<String>,
}

impl Compiler {
    pub fn new(
        plugin_dir: String,
        rx: Receiver<String>,
        loader_send_channel: Sender<String>,
    ) -> Self {

        Self {
            plugin_dir,
            inner_recv: rx,
            loader_send_channel,
        }
    }

    fn compile(&self, file: &str) -> Result<bool, Box<dyn Error>> {
        let dir = Path::new(&self.plugin_dir);

        let file = dir.join(file);
        let file = file.to_str().unwrap();

        let dir = dir.to_str().unwrap();

        let _compile = std::process::Command::new("rustc")
            .arg("--crate-type")
            .arg("cdylib")
            .arg(file)
            .arg("--out-dir")
            .arg(dir)
            .status()?;

        Ok(true)
    }

    pub fn start(self) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || loop {
            let file = self.inner_recv.recv().unwrap();
            self.compile(&file);
            self.loader_send_channel.send(file.clone());
        })
    }

}
