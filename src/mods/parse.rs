use crate::mods::grep::{NoteBlock, NoteFile};

pub trait NoteParser {
    // Called by client to parse note.
    fn parse(&self, note_block: &NoteBlock) -> Vec<NoteFile> {
        self.filter_unit(&note_block)
    }

    // Type specific functions.
    fn divide_and_conquer(&self, note_block: &NoteBlock) -> Vec<NoteFile>;
    fn filter_unit(&self, note_block: &NoteBlock) -> Vec<NoteFile>;
}


#[derive(Debug)]
pub struct SectionParser {
    pub regex: String,
}

impl NoteParser for SectionParser {

    fn divide_and_conquer(&self, note_block: &NoteBlock) -> Vec<NoteFile> {
        let mut filtered_content: Vec<NoteFile> = Vec::new();
        for file in &note_block.content {
            // Split noteblock content into iterable  using delimiter. 
            // This could be improved if only first 3 chars are checked for delimiter 
            let sections = file.content.split(&note_block.delimiter).collect::<Vec<&str>>();

            // for each in sections if regex is inside the section then keep it 
            //let matched_sections: Vec<&&str> = sections.iter().filter(|s| s.contains(&self.regex)).collect();
            sections.iter().for_each(|s| {
                if s.contains(&self.regex) {
                    filtered_content.push(NoteFile { header: format!("{}", &file.header), content: s.to_string() });
                }
            });
        }

        filtered_content
    }

    fn filter_unit(&self, note_block: &NoteBlock) -> Vec<NoteFile> {
        self.divide_and_conquer(&note_block)
    }
}


#[derive(Debug)]
struct FileParser;

// File returns the entire file if match.
impl NoteParser for FileParser {

    fn divide_and_conquer(&self, note_block: &NoteBlock) -> Vec<NoteFile> {

        // TODO
        unimplemented!()
    }

    fn filter_unit(&self, note_block: &NoteBlock) -> Vec<NoteFile> {
        //  section file parser specific code goes here
        unimplemented!()
    }
}


#[derive(Debug)]
struct ContextAwareParser;

// Context aware grabs only the paragraph block that the matching text is in.
// TODO: option to specify depth of context, i.e. how much to get of parent sections.
// Should implement Treesitter to find logical groupings.
impl NoteParser for ContextAwareParser {

    fn divide_and_conquer(&self, note_block: &NoteBlock) -> Vec<NoteFile>{

        // TODO
        unimplemented!()
    }


    fn filter_unit(&self, note_block: &NoteBlock) -> Vec<NoteFile> {
        //  Section ContextAware Parser Specific code goes here
        unimplemented!()
    }
}
