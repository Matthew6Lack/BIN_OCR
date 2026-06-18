use image::ImageReader;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Convertit une image (PNG/JPEG/BMP/GIF) en PGM.
/// - `image_path` : l'image source, ex: `numbers/cinq.png`
/// - Écrit le fichier `.pgm` au même endroit, ex: `numbers/cinq.pgm`
/// - Retourne le chemin du fichier `.pgm` généré : `numbers/cinq.pgm`
pub fn convert_to_pgm(image_path: &str) -> Result<(String, u32, u32), String> {
    let input_path = Path::new(image_path);

    let parent = input_path
        .parent()
        .ok_or_else(|| format!("Chemin invalide : {}", image_path))?;

    if !parent.ends_with("images") {
        return Err(format!(
            "Le fichier doit être dans le dossier 'numbers', reçu : {}",
            image_path
        ));
    }

    let extension = input_path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .ok_or_else(|| "Extension manquante ou invalide".to_string())?;

    match extension.as_str() {
        "png" | "jpg" | "jpeg" | "bmp" | "gif" => {}
        other => {
            return Err(format!("Format non supporté : {}", other));
        }
    }

    let img = ImageReader::open(input_path)
        .map_err(|e| format!("Impossible d'ouvrir l'image '{}' : {}", image_path, e))?
        .decode()
        .map_err(|e| format!("Erreur de décodage de l'image '{}' : {}", image_path, e))?
        .into_luma8();

    let (largeur, hauteur) = img.dimensions();

    let stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Nom de fichier invalide")?;

    let output_path = parent.join(format!("{}.pgm", stem));

    let mut f = File::create(&output_path)
        .map_err(|e| format!("Impossible de créer '{}' : {}", output_path.display(), e))?;

    writeln!(f, "P2\n{} {}\n255", largeur, hauteur)
        .map_err(|e| format!("Erreur d'écriture de l'en-tête : {}", e))?;

    for (i, pixel) in img.pixels().enumerate() {
        write!(f, "{} ", pixel[0]).map_err(|e| format!("Erreur d'écriture pixel {} : {}", i, e))?;
        if (i + 1) % largeur as usize == 0 {
            writeln!(f).map_err(|e| format!("Erreur de retour à la ligne : {}", e))?;
        }
    }

    let result_path = output_path
        .to_str()
        .ok_or("Chemin de sortie invalide")?
        .to_string();

    Ok((result_path, largeur, hauteur))
}
