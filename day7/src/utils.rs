use std::{cell::RefCell, rc::Rc, str::FromStr};

#[derive(Debug)]
pub struct PuzzleFile {
    pub name: String,
    pub size: usize,
}

impl PuzzleFile {
    pub fn new(input_name: &str, input_size: usize) -> Self {
        PuzzleFile {
            name: String::from_str(input_name).expect("Could not assign file name"),
            size: input_size,
        }
    }

    fn get_size(&self) -> usize {
        return self.size;
    }
}

pub struct PuzzleDir {
    pub name: String,
    pub files: Vec<PuzzleFile>,
    pub sub_dirs: Vec<Rc<RefCell<PuzzleDir>>>,
}

impl PuzzleDir {
    fn is_empty(&self) -> bool {
        if self.files.is_empty() & self.sub_dirs.is_empty() {
            return true;
        }
        return false;
    }

    pub fn new(input_name: &str) -> Self {
        PuzzleDir {
            name: String::from_str(input_name).expect("Could not read Dir name"),
            files: Vec::new(),
            sub_dirs: Vec::new(),
        }
    }

    pub fn get_size(&self) -> usize {
        // If empty don't bother computing the size
        if self.is_empty() {
            return 0;
        }

        let mut total_size = 0;

        // Compute size of all child files and dirs

        total_size += self.files.iter().map(|file| file.get_size()).sum::<usize>();
        total_size += self
            .sub_dirs
            .iter()
            .map(|sub_dir| sub_dir.borrow_mut().get_size())
            .sum::<usize>();

        return total_size;
    }
}

const CMD_CD_PREFIX: &str = "$ cd";
const CMD_LS: &str = "$ ls";
const CMD_MOVE_UP: &str = "$ cd ..";
const LS_PREFIX_DIR: &str = "dir ";

#[derive(PartialEq)]
pub enum InputLineType {
    CommandCdIntoParentDir,
    CommandCdIntoDir,
    CommandLsDir,
    LsOutputFile,
    LsOutputDir,
}

pub fn get_line_type(input_line: &String) -> InputLineType {
    // With the exception of the first line, there are only types of cd commands
    // "cd .." or "cd dir_name".
    if input_line.starts_with(CMD_MOVE_UP) {
        return InputLineType::CommandCdIntoParentDir;
    }

    if input_line.starts_with(CMD_CD_PREFIX) {
        return InputLineType::CommandCdIntoDir;
    }

    if input_line.starts_with(CMD_LS) {
        return InputLineType::CommandLsDir;
    }

    if input_line.starts_with(LS_PREFIX_DIR) {
        return InputLineType::LsOutputDir;
    }
    /*
    There is only one other type of line remaining
    Since all other cases have been handled. We can return this safely
    */
    return InputLineType::LsOutputFile;
}
