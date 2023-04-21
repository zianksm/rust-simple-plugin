mod watcher;

use std::path::Path;

use libloading::Symbol;
use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};

fn main() -> Result<()> {
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(|res| {
        match res {
           Ok(event) => println!("event: {:?}", event),
           Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("./src/plugins"), RecursiveMode::Recursive)?;

    loop {

    }

    let compile = std::process::Command::new("rustc")
        .arg("--crate-type")
        .arg("cdylib")
        .arg("./src/plugins/plugin.rs")
        .arg("--out-dir").arg("./src/plugins/")
        .status()
        .unwrap();

    unsafe {
        let lib = libloading::Library::new("./src/plugins/plugin.dll").unwrap();
        let _fn: Symbol<fn() -> ()> = lib.get(b"test\0").unwrap();
        _fn();
    }

    Ok(())
}
