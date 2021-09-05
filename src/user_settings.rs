use std::fs;
use ammonia::clean_text;

pub fn get_filescontainer() -> String {
//! Get the user setting of FilesContainer where that exists
	let filename = format!("./❰Downloads❱/userinput_filescontainer");
	match std::path::Path::new(&filename).exists() {
    	true => { clean_text(&String::from_utf8_lossy(&fs::read(&filename).unwrap())) },
		false => "".to_string(),
	}
}

