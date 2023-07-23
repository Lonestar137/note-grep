use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Note {
   pub filetype: String,
   pub note_dir: String,
   pub delimiter: String,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Pager {
   pub pager: String,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Config {
   pub note: Note,
   pub pager: Pager,
}



// Example
    // let file_path = "config.toml";
    // let mut file = std::fs::File::open(file_path).expect("Failed to read file.");
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).expect("Failed to read file.");
    //
    // let running_config: Config = toml::from_str(&contents).unwrap();
    //
    // println!("{:?}", running_config);
