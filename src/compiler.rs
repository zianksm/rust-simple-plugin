use std::{
    error::Error,
    path::Path,
    sync::mpsc::{channel, Receiver, Sender},
};

pub struct Compiler {
    plugin_dir: String,
    inner_recv: Receiver<String>,
    sender_channel: Sender<String>,
}

impl Compiler {
    pub fn new(plugin_dir: &str) -> Self {
        let (inner_sender, watcher_receiver) = channel::<String>();

        Self {
            plugin_dir: plugin_dir.to_string(),
            sender_channel: inner_sender,
            inner_recv: watcher_receiver,
        }
    }

    pub fn compile(&self, file: &str) -> Result<bool, Box<dyn Error>> {
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

    pub fn watcher_receiver(&self) -> &Receiver<String> {
        &self.inner_recv
    }

    pub fn inner_sender(&self) -> Sender<String> {
        self.sender_channel.clone()
    }
}
