use crate::mods::grep::NoteBlock;

pub trait NoteParser {
    // Called by client to parse note.
    fn parse(&self, note_block: &NoteBlock) -> String {
        self.filter_unit(&note_block)
    }

    // Type specific functions.
    fn divide_and_conquer(&self, note_block: &NoteBlock) -> String;
    fn filter_unit(&self, note_block: &NoteBlock) -> String;
}


#[derive(Debug)]
pub struct SectionParser {
    pub regex: String,
}

impl NoteParser for SectionParser {

    fn divide_and_conquer(&self, note_block: &NoteBlock) -> String {
        // Split noteblock content into iterable  using delimiter.
        let sections: Vec<&str> = note_block.content.split(&note_block.delimiter).collect::<Vec<&str>>();

        // for each in sections if regex is inside the section then keep it 
        let matched_sections: Vec<&&str> = sections.iter().filter(|s| s.contains(&self.regex)).collect();
        let as_string: String = matched_sections.iter().map(|&&s| s).collect::<Vec<&str>>().join("");
        as_string
    }

    fn filter_unit(&self, note_block: &NoteBlock) -> String {
        self.divide_and_conquer(&note_block)
    }
}


#[derive(Debug)]
struct FileParser;

// File returns the entire file if match.
impl NoteParser for FileParser {

    fn divide_and_conquer(&self, note_block: &NoteBlock) -> String{

        // TODO
        unimplemented!()
    }

    fn filter_unit(&self, note_block: &NoteBlock) -> String {
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

    fn divide_and_conquer(&self, note_block: &NoteBlock) -> String{

        // TODO
        unimplemented!()
    }


    fn filter_unit(&self, note_block: &NoteBlock) -> String {
        //  Section ContextAware Parser Specific code goes here
        unimplemented!()
    }
}
