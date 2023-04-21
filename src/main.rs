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
    

    // to keep main thread from exiting
    loop {}

    Ok(())
}
