use std::io::Write;
use std::path::Path; 
use std::fs::File;

///TODO Write test cases for this code. 

// a function that will check for the exisiting file, returns Some(File) else None
pub fn check_for_file(file_name: impl Into<String>) -> bool {
    Path::exists(Path::new(&file_name.into()))
}

pub fn open_file(file_name: impl Into<String>) -> Option<File> {
    File::open(&file_name.into()).ok()
}

pub fn create_tree_for_file(new_file_path: impl Into<String>) -> Option<File> {
    let f_path: String = new_file_path.into(); 
    if let Err(e) = std::fs::create_dir_all(Path::new(&f_path).parent().unwrap()) {
        panic!("Failed to create a file {}", e);
    } else {
        File::create(Path::new(&f_path)).ok()
    }
}
