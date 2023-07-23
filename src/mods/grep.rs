use crate::FileSystemInterface;
use crate::mods::parse::{SectionParser, NoteParser};
use crate::Config;

#[derive(Debug)]
pub struct NoteFile {
   pub header: String,
   pub content: String
}

#[derive(Debug)]
pub struct NoteBlock {
   pub content: Vec<NoteFile>,
   pub delimiter: String, // default = ---
}

impl NoteBlock {
   pub fn new(content: Vec<NoteFile>, delimiter: String) -> NoteBlock {
      NoteBlock { content, delimiter }
   }

   pub fn update_content(&mut self, content: Vec<NoteFile>) {
      self.content = content
   }
}


pub struct NoteBlockBuilder<'a> {
   pub config: Config,
   strategy: Box<&'a dyn FileSystemInterface>,
   note_block: NoteBlock,
}

impl<'a> NoteBlockBuilder<'a> {
   pub fn new(config: Config, strategy: Box<&'a dyn FileSystemInterface>, content: Vec<NoteFile>) -> NoteBlockBuilder<'a> {
      let delimiter = config.note.delimiter.clone();
      NoteBlockBuilder { config, strategy, note_block: NoteBlock::new(content, delimiter ) }
   }

   pub fn fetch_content(mut self) -> NoteBlockBuilder<'a> {
      let note_dir = &self.config.note.note_dir;
      let note_filetype = &self.config.note.filetype;
      let note_files = self.strategy.list_files(&note_dir, &note_filetype);

      for file in note_files {
         let file_header: String = format!("# File: `[{}]`\n", &file);
         let block = self.strategy.read_file(&file);
         let note_file = NoteFile { header: file_header, content: block.to_string() };
         self.note_block.content.push(note_file);
      }

      self
   }

   // Only return blocks that contain a match to the regex.
   // TODO: refactor this to use a filter strategy
   pub fn filter_content(mut self, regex: &str) -> NoteBlockBuilder<'a> {
      let parser: Box<dyn NoteParser> = Box::new(SectionParser{regex: regex.to_string()});
      let filtered_content: Vec<NoteFile> = parser.parse(&self.note_block);
      self.note_block.update_content(filtered_content);
      self
   }

   pub fn build(self) -> NoteBlock {
      self.note_block
   }
}

