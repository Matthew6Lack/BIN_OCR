use std::fs;
use std::path::Path;

pub fn find_img_position(i: usize) -> Option<String> {
    let dir_path = "src/img";
    
    if !Path::new(dir_path).is_dir() {
        return None;
    }

    if let Ok(entries) = fs::read_dir(dir_path) {
        let mut valid_entries = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file()); 

        if let Some(entry) = valid_entries.nth(i) {
            // On convertit le Path en String
            return Some(entry.path().to_string_lossy().into_owned());
        }
    }

    None
}