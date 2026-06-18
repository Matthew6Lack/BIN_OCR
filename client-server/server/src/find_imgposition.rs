use std::fs;
use std::path::Path;

pub fn find_img_position(i: usize) -> Option<String> {
    let dir_path = "src/images";
    
    if !Path::new(dir_path).is_dir() {
        return None;
    }

    if let Ok(entries) = fs::read_dir(dir_path) {
        let mut paths: Vec<_> = entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_file())
            .collect();
        paths.sort();
        if let Some(path) = paths.get(i) {
            return Some(path.to_string_lossy().into_owned());
        }
    }

    None
}