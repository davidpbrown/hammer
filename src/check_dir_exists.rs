//! Create directory and any parents
use std::fs;

pub fn check_dir_exists(dir: &str) -> bool {
	match fs::create_dir_all(format!("{}", &dir)) {
		Ok(_) => true,
		Err(err) => { println!("Error creating directory: {}\n{}", dir, err); false}
	}
}

