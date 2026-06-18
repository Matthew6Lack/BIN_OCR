use image::GenericImageView;
use rfd::FileDialog;
use std::fs;
use std::path::Path;
use crate::binarisation::*;
use image::{GrayImage, Luma};
use image::ImageReader;
use std::fs::File;
use std::io::Write;

pub fn save_matrix_to_images(matrix: &Vec<Vec<u32>>) -> Result<String, String> {
    let images_dir = Path::new("src/images");
    fs::create_dir_all(images_dir)
        .map_err(|e| format!("Impossible de créer src/images : {}", e))?;

    let nb = fs::read_dir(images_dir)
        .map_err(|e| format!("Impossible de lire src/images : {}", e))?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension()
                .and_then(|x| x.to_str())
                .map(|x| x == "png")
                .unwrap_or(false)
        })
        .count();

    if matrix.is_empty() || matrix[0].is_empty() {
        return Err("Matrice vide".to_string());
    }

    let height = matrix.len() as u32;
    let width = matrix[0].len() as u32;

    let mut img = GrayImage::new(width, height);
    for (y, row) in matrix.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let pixel_val: u8 = if val > 0 { 255 } else { 0 };
            img.put_pixel(x as u32, y as u32, Luma([pixel_val]));
        }
    }

    let output_path = images_dir.join(format!("loaded_{}.png", nb));
    img.save(&output_path)
        .map_err(|e| format!("Erreur de sauvegarde PNG : {}", e))?;

    output_path.to_str()
        .ok_or("Chemin invalide".to_string())
        .map(|s| s.to_string())
}

pub fn convert_to_pgm(image_path: &str) -> Result<(String, u32, u32), String> {
    let input_path = Path::new(image_path);

    let parent = input_path
        .parent()
        .ok_or_else(|| format!("Chemin invalide : {}", image_path))?;

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

    println!("[DEBUG CONVERT] Conversion PGM du fichier : '{}'", input_path.display());

    let file = std::fs::File::open(input_path)
        .map_err(|e| format!("Impossible de lire le fichier : {}", e))?;

    let reader = ImageReader::new(std::io::BufReader::new(file))
        .with_guessed_format()
        .map_err(|e| format!("Format non reconnu : {}", e))?;

    let img = reader.decode()
        .map_err(|e| format!("Erreur de décodage : {}", e))?
        .to_luma8();

    let (largeur, hauteur) = img.dimensions();

    let stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Nom de fichier invalide")?;

    let output_path = parent.join(format!("{}.pgm", stem));

    let mut f = File::create(&output_path)
        .map_err(|e| format!("Impossible de créer le PGM : {}", e))?;

    writeln!(f, "P2\n{} {}\n255", largeur, hauteur)
        .map_err(|e| format!("Erreur en-tête PGM : {}", e))?;

    for (i, pixel) in img.pixels().enumerate() {
        write!(f, "{} ", pixel[0]).map_err(|e| format!("Erreur pixel {} : {}", i, e))?;
        if (i + 1) % largeur as usize == 0 {
            writeln!(f).map_err(|e| format!("Erreur retour ligne : {}", e))?;
        }
    }

    let result_path = output_path
        .to_str()
        .ok_or("Chemin de sortie invalide")?
        .to_string();

    Ok((result_path, largeur, hauteur))
}

pub fn load_img() -> Option<(Vec<Vec<u32>>, String, String)> {
    let picked_path = FileDialog::new()
        .add_filter(
            "Images",
            &["png", "jpg", "jpeg", "bmp", "gif", "tiff", "webp"],
        )
        .pick_file()?;

    let image_path_str = picked_path.to_str()?.to_string();

    let (pgm_path, width, height) = match convert_to_pgm(&image_path_str) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("[ERREUR LOAD] Echec convert_to_pgm : {:?}", e);
            return None;
        }
    };

    let matrix = match binarize_pgm(&pgm_path, width, height) {
        Ok(result) => {
            let _ = fs::remove_file(&pgm_path);
            result
        }
        Err(e) => {
            let _ = fs::remove_file(&pgm_path);
            eprintln!("[ERREUR LOAD] Echec binarize_pgm : {:?}", e);
            return None;
        }
    };

    let final_path = match save_matrix_to_images(&matrix) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("[ERREUR LOAD] Echec save_matrix_to_images : {:?}", e);
            return None;
        }
    };

    Some((matrix, image_path_str, final_path))
}