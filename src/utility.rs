use std::{path::Path, env}; 
use std::fs::File;
use rand::{distributions::Alphanumeric, Rng};

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

pub fn generate_random_string(length: u64) -> Option<String> {
    Some(rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect())
}

pub fn get_env_var(var_name: impl Into<String>) -> String {
    match env::var(var_name.into()) {
        Ok(v) => v.to_owned(),
        Err(_) => {
            panic!("Failed to get the environment variable"); 
        }
    }
}


