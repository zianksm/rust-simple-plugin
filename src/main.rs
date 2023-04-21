mod compiler;
mod loader;
mod watcher;

use std::{path::Path, sync::mpsc::channel};

use libloading::Symbol;
use notify::{RecursiveMode, Result, Watcher};

fn main() -> Result<()> {

    let (tx, rx) = channel::<String>();

    let mut watcher = watcher::PluginDirectoryWatcher::new("./src/plugins".to_string(), tx);

    watcher.start();

    loop {}

    // let _compile = std::process::Command::new("rustc")
    //     .arg("--crate-type")
    //     .arg("cdylib")
    //     .arg("./src/plugins/plugin.rs")
    //     .arg("--out-dir").arg("./src/plugins/")
    //     .status()
    //     .unwrap();

    // unsafe {
    //     let lib = libloading::Library::new("./src/plugins/plugin.dll").unwrap();
    //     let _fn: Symbol<fn() -> ()> = lib.get(b"test\0").unwrap();
    //     _fn();
    // }

    Ok(())
}
