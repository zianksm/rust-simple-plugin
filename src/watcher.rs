

use notify::{Event, ReadDirectoryChangesWatcher};

pub struct Watcher {
    dir: String,
    _self: ReadDirectoryChangesWatcher,
}

impl Watcher {
    pub fn new(dir: String) -> Self {
        let _self = notify::recommended_watcher(Self::handle_event).unwrap();

        Self { dir, _self }
    }

    fn handle_event(_event: Result<Event, notify::Error>) {}

    pub fn start(&mut self) {}

    pub fn stop(&mut self) {}
}
