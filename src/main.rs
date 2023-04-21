mod compiler;
mod loader;
mod watcher;

use std::{path::Path, sync::mpsc::channel};

use libloading::Symbol;
use notify::{RecursiveMode, Result, Watcher};

fn main() -> Result<()> {
    let plugin_dir = String::from("./src/plugins");

    let (compiler_tx, compiler_rx) = channel::<String>();
    let (loader_tx, loader_rx) = channel::<String>();

    let mut compiler = compiler::Compiler::new(plugin_dir.clone(), compiler_rx,loader_tx.clone());
    let mut loader  = loader::Loader::new(plugin_dir.clone(),loader_rx);
    let mut watcher = watcher::PluginDirectoryWatcher::new(plugin_dir.clone(), compiler_tx);

    compiler.start();
    loader.start();
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
