use core::panic;
use std::{
    path::Path,
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
};

use notify::{Event, ReadDirectoryChangesWatcher, RecursiveMode, Watcher};

#[derive(Clone)]
pub struct PluginDirectoryWatcher {
    dir: String,
    inner: Arc<Mutex<Option<ReadDirectoryChangesWatcher>>>,
    compiler_channel: Arc<Mutex<Sender<String>>>,
}

impl PluginDirectoryWatcher {
    pub fn new(dir: String, compiler_send_channel: Sender<String>) -> Self {
        Self {
            dir,
            inner: Arc::new(Mutex::new(None)),
            compiler_channel: Arc::new(Mutex::new(compiler_send_channel)),
        }
    }

    fn handle_event(&self, event: Result<Event, notify::Error>) {
        match event {
            Ok(ev) => self.infer_event(ev),
            Err(err) => panic!("{}", err),
        }
    }

    pub fn start(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let self_clone = self.clone();
        let mut watcher =
            notify::recommended_watcher(move |ev| self_clone.handle_event(ev)).unwrap();

        watcher.watch(Path::new(&self.dir), RecursiveMode::Recursive)?;

        self.inner = Arc::new(Mutex::new(Some(watcher)));

        Ok(true)
    }

    pub fn stop(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let mut watcher = self.inner.lock().unwrap();
        let watcher = watcher.as_mut().unwrap();

        watcher.unwatch(Path::new(&self.dir))?;

        Ok(true)
    }

    fn infer_event(&self, event: Event) {
        match event.kind {
            notify::EventKind::Any => (),
            notify::EventKind::Access(_) => (),
            notify::EventKind::Create(_) => (),
            notify::EventKind::Modify(_) => self.call_compiler(event),
            notify::EventKind::Remove(_) => (),
            notify::EventKind::Other => (),
        }
    }

    fn call_compiler(&self, event: Event) {
        let file = event.paths[0].file_name().unwrap().to_str().unwrap();

        if !file.contains(".rs"){
            return;
        }

        println!("recompiling: {file}");

        let compiler = self.compiler_channel.lock().unwrap();

        compiler.send(file.to_string());
    }
}
