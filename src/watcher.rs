use std::{
    path::Path,
    sync::mpsc::{channel, Sender},
};

use notify::{Event, ReadDirectoryChangesWatcher, RecursiveMode, Watcher};

pub struct PluginDirectoryWatcher {
    dir: String,
    inner: ReadDirectoryChangesWatcher,
    compiler_channel: Sender<String>,
}

impl PluginDirectoryWatcher {
    pub fn new(dir: String, compiler_channel: Sender<String>) -> Self {
        let _self = notify::recommended_watcher(Self::handle_event).unwrap();

        Self {
            dir,
            inner: _self,
            compiler_channel,
        }
    }

    fn handle_event(_event: Result<Event, notify::Error>) {}

    pub fn start(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let mut watcher = self
            .inner
            .watch(Path::new(&self.dir), RecursiveMode::Recursive)?;

        Ok(true)
    }

    pub fn stop(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        self.inner.unwatch(Path::new(&self.dir))?;

        Ok(true)
    }
}
