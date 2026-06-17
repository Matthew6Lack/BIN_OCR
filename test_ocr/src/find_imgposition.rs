use std::fs;
use std::path::Path;

pub fn find_img_position(i: usize) -> Option<String> {
    let dir_path = "src/img";
    
    if !Path::new(dir_path).is_dir() {
        return None;
    }

    if let Ok(entries) = fs::read_dir(dir_path) {
        // 1. On extrait et filtre tous les chemins de fichiers valides
        let mut paths: Vec<String> = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| e.path().to_string_lossy().into_owned())
            .collect();

        // 2. CORRECTION CRUCIALE : On trie la liste par ordre alphabétique
        paths.sort();

        // 3. On vérifie si l'index demandé existe dans notre liste triée
        if i < paths.len() {
            return Some(paths[i].clone());
        }
    }

    None
}