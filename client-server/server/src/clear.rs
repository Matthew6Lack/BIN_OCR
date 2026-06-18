use std::fs;
use std::io;
use std::path::Path;

pub fn clear_digits_folder() -> io::Result<()> {
    let folder_path = Path::new("src/digits");

    if !folder_path.exists() {
        println!("Le dossier {:?} n'existe pas.", folder_path);
        return Ok(());
    }

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            fs::remove_file(&path)?;
            println!("Supprimé : {:?}", path.file_name().unwrap_or_default());
        }
    }

    println!("Nettoyage du dossier 'digits' terminé avec succès !");
    Ok(())
}
