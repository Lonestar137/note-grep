mod mods;
use mods::config::Config;
use mods::strategy::FileSystemInterface;

fn get_config(strategy: &dyn FileSystemInterface, path: String) -> Config {
    let contents = strategy.read_config(&path);
    let running_config: Config = toml::from_str(&contents).unwrap();
    running_config
}

fn determine_strategy() -> Box<dyn FileSystemInterface> {
    if cfg!(target_os = "windows") {
        Box::new(mods::strategy::WindowsFileSystemInterface)
    } else {
        Box::new(mods::strategy::UnixFileSystemInterface)
    }
}

fn main() {
    let strategy: Box<dyn FileSystemInterface> = determine_strategy();
    let config = get_config(&*strategy, "./config.toml".to_string());

    println!("{:?}", config);
}
