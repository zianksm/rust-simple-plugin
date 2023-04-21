use core::panic;
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

    fn handle_event(event: Result<Event, notify::Error>) {
        match event {
            Ok(ev) => Self::infer_event(ev),
            Err(err) => panic!("{}", err),
        }
    }

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

    fn infer_event(event: Event) {
        match event.kind {
            notify::EventKind::Any => todo!(),
            notify::EventKind::Access(_) => todo!(),
            notify::EventKind::Create(_) => todo!(),
            notify::EventKind::Modify(_) => Self::call_compiler(event),
            notify::EventKind::Remove(_) => todo!(),
            notify::EventKind::Other => todo!(),
        }
    }

    fn call_compiler(event: Event) {
    //  let path =    event.paths
    }

}
