mod mods;
use mods::config::Config;
use mods::strategy::FileSystemInterface;


// Function that takes the FileSystemStrategy trait as a parameter
fn print_files(strategy: &dyn FileSystemInterface, path: &str) {
    let files = strategy.list_files(path);
    println!("Files: {:?}", files);
}

fn determine_strategy() -> Box<dyn FileSystemInterface> {
    // determine if this is Windows or Unix
    if cfg!(target_os = "windows") {
        Box::new(mods::strategy::WindowsFileSystemInterface)
    } else {
        Box::new(mods::strategy::UnixFileSystemInterface)
    }
}

fn main(){

    let strategy: Box<dyn FileSystemInterface> = determine_strategy();
    print_files(&*strategy, "/tmp/");

    // let config: Config = toml::from_str(r#"
    //    ip = '127.0.0.1'
    //
    //    [keys]
    //    github = 'xxxxxxxxxxxxxxxxx'
    //    travis = 'yyyyyyyyyyyyyyyyy'
    // "#).unwrap();

    // assert_eq!(config.ip, "127.0.0.1");
    // assert_eq!(config.port, None);
    // assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
    // assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");

}
