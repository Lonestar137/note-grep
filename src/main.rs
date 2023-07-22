mod mods;
use mods::grep::NoteBlockBuilder;
use mods::config::Config;
use mods::strategy::FileSystemInterface;

use std::io::Write;
use std::process::{Stdio, Command};

// TODO: create config file automatically.
// TODO: use a state pattern to manage the config files location and state, pass to strategy as
// paremeter.

// If a custom config path is defined it will override the default config.
fn get_config(strategy: &dyn FileSystemInterface, path: String) -> Config {
    let contents = if path.is_empty() {
        println!("Using default config");
        strategy.read_default_config()
    } else {
        println!("Using custom config");
        strategy.read_config(&path)
    };

    let config: Config = toml::from_str(&contents).unwrap();
    config 
}

fn determine_strategy() -> Box<dyn FileSystemInterface> {
    if cfg!(target_os = "windows") {
        Box::new(mods::strategy::WindowsFileSystemInterface)
    } else {
        Box::new(mods::strategy::UnixFileSystemInterface)
    }
}

fn main() {
    //let config_file_dir = "./config.toml"; // TODO: let this be changed by command line argument
    let config_file_dir = "";
    let strategy: Box<dyn FileSystemInterface> = determine_strategy();
    let config: Config = get_config(&*strategy, config_file_dir.to_string());

    let pager: String = config.pager.pager.clone();
    let note_block = NoteBlockBuilder::new(config, Box::new(&*strategy), "hello".to_string())
        .fetch_content()
        .build();

    let mut less_cmd = Command::new(pager);
    less_cmd.stdin(Stdio::piped());
    let mut child = less_cmd.spawn().expect("Failed to spawn pager command");
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(note_block.content.as_bytes()).expect("Failed to write to stdin.");
    }

    let status = child.wait().expect("Failed to wait for pager process.");
    if !status.success() {
        eprintln!("'less' command returned an error: {:?}", status);
    }
    //println!("{:?}", note_block);
}
