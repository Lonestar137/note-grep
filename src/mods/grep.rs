use crate::FileSystemInterface;
use crate::Config;

#[derive(Debug)]
pub struct NoteBlock {
   pub content: String,
}

impl NoteBlock {
   pub fn new(content: String) -> NoteBlock {
      NoteBlock { content }
   }
}


pub struct NoteBlockBuilder<'a> {
   pub config: Config,
   strategy: Box<&'a dyn FileSystemInterface>,
   note_block: NoteBlock,
}

impl<'a> NoteBlockBuilder<'a> {
   pub fn new(config: Config, strategy: Box<&'a dyn FileSystemInterface>, content: String) -> NoteBlockBuilder<'a> {
      NoteBlockBuilder { config, strategy, note_block: NoteBlock::new(content) }
   }

   pub fn fetch_content(mut self) -> NoteBlockBuilder<'a> {
      let note_dir = self.config.note.note_dir.clone();
      let note_files = self.strategy.list_files(&note_dir);
      for file in note_files {
         self.note_block.content.push_str(&self.strategy.read_file(&file));
         //self.note_block.content.push_str("");
      }

       // Return the fetched content
      self
   }

   // Only return blocks that contain a match to the regex.
   // TODO: refactor this to use a filter strategy
   pub fn filter_content(self, regex: &str) -> String {
      self.note_block.content;
      unimplemented!("TODO: filter blocks by regex")
   }

   pub fn build(self) -> NoteBlock {
      self.note_block
   }
}

