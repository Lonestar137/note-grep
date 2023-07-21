use toml;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct Note {
   filetype: String,
   note_dir: String,
}

#[derive(Debug, Deserialize)]
struct Pager {
   pager: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
   note: Note,
   pager: Pager,
}



    // let file_path = "config.toml";
    // let mut file = std::fs::File::open(file_path).expect("Failed to read file.");
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).expect("Failed to read file.");
    //
    // let running_config: Config = toml::from_str(&contents).unwrap();
    //
    // println!("{:?}", running_config);
