use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;


pub trait FileSystemInterface {
   fn read_file(&self, path: &str) -> String;
   fn read_config(&self, file_path: &str) -> String;
   fn read_default_config(&self) -> String;
   fn list_files(&self, path: &str, file_ext: &str) -> Vec<String>;
}


pub struct UnixFileSystemInterface;

impl FileSystemInterface for UnixFileSystemInterface {
   fn read_file(&self, file_path: &str) -> String {
      // Implement the logic to read a file in Linux
      let mut file = File::open(file_path).expect("Failed to read file.");
      let mut contents = String::new();
      file.read_to_string(&mut contents).expect("Failed to read file.");
      contents
   }

   fn read_default_config(&self) -> String {
      let home = env::var("HOME").ok();
      let config_path = PathBuf::from(home.unwrap()).join(".config/note-grep/config.toml");

      let mut file = File::open(config_path).expect("Failed to read file.");
      let mut contents = String::new();
      file.read_to_string(&mut contents).expect("Failed to read file.");
      contents
   }

   fn read_config(&self, file_path: &str) -> String {
      let config_file = self.read_file(file_path);
      config_file
   }

   fn list_files(&self, path: &str, file_ext: &str) -> Vec<String> {
      // Implement the logic to list files for Unix
      let files = std::fs::read_dir(path).unwrap();
      let mut vec = Vec::new();
      for file in files {
         let path_buf: &PathBuf = &file.as_ref().unwrap().path();
         if  &path_buf.extension().unwrap().to_str().unwrap().to_string() == &file_ext {
            vec.push(path_buf.to_str().unwrap().to_string());
         }
      }
      vec
   }
}


pub struct WindowsFileSystemInterface;

impl FileSystemInterface for WindowsFileSystemInterface {
   fn read_file(&self, file_path: &str) -> String {
      // Implement the logic to list files for Windows
      let mut file = File::open(file_path).expect("Failed to read file.");
      let mut contents = String::new();
      file.read_to_string(&mut contents).expect("Failed to read file.");
      contents
   }

   fn read_default_config(&self) -> String {
      let home = env::var("APPDATA").ok();
      let config_path = PathBuf::from(home.unwrap()).join("\\note-grep\\config.toml");

      let mut file = File::open(config_path).expect("Failed to read file.");
      let mut contents = String::new();
      file.read_to_string(&mut contents).expect("Failed to read file.");
      contents
   }

   fn read_config(&self, file_path: &str) -> String {
      let config_file = self.read_file(file_path);
      config_file
   }

   fn list_files(&self, path: &str, file_ext: &str) -> Vec<String> {
      // Implement the logic to list files for Windows
      let files = std::fs::read_dir(path).unwrap();
      let mut vec = Vec::new();
      for file in files {
         let path_buf: &PathBuf = &file.as_ref().unwrap().path();
         if  &path_buf.extension().unwrap().to_str().unwrap().to_string() == &file_ext {
            vec.push(path_buf.to_str().unwrap().to_string());
         }
      }
      vec
   }
}



pub struct SystemInterfaceBuilder {
   strategy: Box<dyn FileSystemInterface>,
}

impl SystemInterfaceBuilder {
   pub fn new() -> SystemInterfaceBuilder {
      let strategy: Box<dyn FileSystemInterface> = if cfg!(target_os = "windows") {
         Box::new(WindowsFileSystemInterface)
      } else {
         Box::new(UnixFileSystemInterface)
      };

      SystemInterfaceBuilder { strategy }
   }

   pub fn build(self) -> Box<dyn FileSystemInterface> {
      self.strategy
   }
}
