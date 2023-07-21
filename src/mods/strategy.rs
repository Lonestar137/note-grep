use std::fs::File;
use std::io::Read;


pub trait FileSystemInterface {
   fn read_file(&self, path: &str) -> String;
   fn list_files(&self, path: &str) -> Vec<String>;
}


pub struct UnixFileSystemInterface;

impl FileSystemInterface for UnixFileSystemInterface {
   fn read_file(&self, file_path: &str) -> String {
      // Implement the logic to list files for Windows
      let mut file = File::open(file_path).expect("Failed to read file.");
      let mut contents = String::new();
      file.read_to_string(&mut contents).expect("Failed to read file.");
      contents
   }

   fn list_files(&self, path: &str) -> Vec<String> {
      // Implement the logic to list files for Unix
      let files = std::fs::read_dir(path).unwrap();
      let mut vec = Vec::new();
      for file in files {
         let file = file.unwrap();
         vec.push(file.path().to_str().unwrap().to_string());
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

   fn list_files(&self, path: &str) -> Vec<String> {
      // Implement the logic to list files for Unix
      let parsed_path = path.replace("\\", "/");
      let files = std::fs::read_dir(parsed_path).unwrap();
      let mut vec = Vec::new();
      for file in files {
         let file = file.unwrap();
         vec.push(file.path().to_str().unwrap().to_string());
      }
      vec
   }
}

