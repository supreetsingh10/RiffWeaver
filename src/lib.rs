pub mod auth;
pub mod token;
pub mod constants;
pub mod user_config;

pub mod utils {
    use std::env::{var, VarError};

    pub fn get_env_var(name: &str) -> Result<String, VarError> {
        var(name)
    }

    pub fn generate_abs_path(dir_name: &str) -> String {
        let mut abs_path= get_env_var("HOME").expect("Failed to get the environment variable"); 
        abs_path.push_str(dir_name);
        println!("{}", abs_path.clone());
        abs_path
    }
}
