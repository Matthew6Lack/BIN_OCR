use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// Convertit un tableau de binarisation (`Vec<Vec<u32>>`, 0=noir, 1 ou 255=blanc) en fichier PGM
/// Le nom du fichier est le nombre d'images déjà présentes dans le dossier.
/// Retourne le chemin du fichier créé : `src/digits/nb.pgm`
pub fn convert_to_img(matrix: &Vec<Vec<u32>>) -> Result<String, String> {
    let digits_dir = Path::new("src/digits");

    fs::create_dir_all(digits_dir)
        .map_err(|e| format!("Impossible de créer le dossier 'src/digits' : {}", e))?;

    let nb = fs::read_dir(digits_dir)
        .map_err(|e| format!("Impossible de lire 'src/digits' : {}", e))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e == "pgm")
                .unwrap_or(false)
        })
        .count();

    if matrix.is_empty() || matrix[0].is_empty() {
        return Err("La matrice de binarisation est vide".to_string());
    }

    let hauteur = matrix.len();
    let largeur = matrix[0].len();

    for (i, row) in matrix.iter().enumerate() {
        if row.len() != largeur {
            return Err(format!(
                "Ligne {} : largeur {} incohérente avec la première ligne ({})",
                i,
                row.len(),
                largeur
            ));
        }
        for (j, &val) in row.iter().enumerate() {
            // SÉCURITÉ MODIFIÉE : On accepte maintenant 0, 1 et 255 comme valeurs valides
            if val != 0 && val != 1 && val != 255 {
                return Err(format!(
                    "Valeur invalide ({}) en position [{},{}] — seuls 0, 1 et 255 sont autorisés",
                    val, i, j
                ));
            }
        }
    }

    let output_path = digits_dir.join(format!("{}.pgm", nb));

    let mut f = File::create(&output_path)
        .map_err(|e| format!("Impossible de créer '{}' : {}", output_path.display(), e))?;

    writeln!(f, "P2\n{} {}\n255", largeur, hauteur)
        .map_err(|e| format!("Erreur d'écriture de l'en-tête : {}", e))?;

    for row in matrix {
        let line: Vec<String> = row
            .iter()
            .map(|&pixel| {
                // TRADUCTION MODIFIÉE : Si le pixel vaut 1 ou 255, on écrit "255" (blanc) dans le fichier PGM
                if pixel > 0 {
                    "255".to_string()
                } else {
                    "0".to_string()
                }
            })
            .collect();
        writeln!(f, "{}", line.join(" "))
            .map_err(|e| format!("Erreur d'écriture d'une ligne : {}", e))?;
    }

    let result_path = output_path
        .to_str()
        .ok_or("Chemin de sortie invalide")?
        .to_string();

    Ok(result_path)
}