use image::GenericImageView;
use rfd::FileDialog;
use std::fs;
use std::path::Path;

pub fn load_img(liste_images: &mut Vec<String>) -> Option<(Vec<Vec<u32>>, String)> {
    // Ouvre l'explorateur de fichiers
    let path = FileDialog::new()
        .add_filter(
            "Images",
            &["png", "jpg", "jpeg", "bmp", "gif", "tiff", "webp"],
        )
        .pick_file()?;

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    let valid_extensions = ["png", "jpg", "jpeg", "bmp", "gif", "tiff", "webp"];
    match ext {
        Some(ref e) if valid_extensions.contains(&e.as_str()) => {}
        _ => return None,
    }

    let name = path.file_name()?.to_str()?.to_string();

    let dest_dir = Path::new("src/img");
    fs::create_dir_all(dest_dir).ok()?;
    let dest_path = dest_dir.join(&name);
    fs::copy(&path, &dest_path).ok()?;

    if let Some(chemin_str) = dest_path.to_str() {
        if !liste_images.contains(&chemin_str.to_string()) {
            liste_images.push(chemin_str.to_string());
        }
    }

    let img = image::open(&dest_path).ok()?;
    let (width, height) = img.dimensions();

    let binary: Vec<Vec<u32>> = (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    let pixel = img.get_pixel(x, y);
                    let [r, g, b, _] = pixel.0;
                    // Luminosité moyenne
                    let luma = (r as u32 + g as u32 + b as u32) / 3;
                    if luma >= 220 {
                        1
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect();

    Some((binary, name))
}
