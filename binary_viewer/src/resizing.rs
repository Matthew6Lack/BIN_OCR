// Pour l'OCR le résau de neuronnes s'applique à des matrice carré dont la largeur et la hauteur sont la même
// Cette fonction permet de redimensionner le tableau de binarisation pour le transformer en matrice carré
// De plus cette fonction permet au chiffre de rester positionner au centre
/*
pub fn resize_and_center(matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let old_height = matrix.len();
    if old_height == 0 {
        return Vec::new();
    }
    let old_width = matrix[0].len();
    if old_width == 0 {
        return Vec::new();
    }

    let new_dim = old_height.max(old_width);

    let pad_top = (new_dim - old_height) / 2;
    let pad_left = (new_dim - old_width) / 2;

    let mut new_matrix = vec![vec![0; new_dim]; new_dim];

    for r in 0..old_height {
        for c in 0..old_width {
            new_matrix[r + pad_top][c + pad_left] = matrix[r][c];
        }
    }

    new_matrix
}

pub fn resize_matrix_to_28x28(matrix: &Vec<Vec<u32>>, current_size: usize) -> Vec<Vec<u32>> {
    if current_size == 28 {
        return matrix.clone();
    }

    let mut resized = vec![vec![0; 28]; 28];
    let scale = current_size as f64 / 28.0;

    for y_28 in 0..28 {
        for x_28 in 0..28 {
            let src_x = (x_28 as f64 * scale).round() as usize;
            let src_y = (y_28 as f64 * scale).round() as usize;

            let safe_x = src_x.min(current_size - 1);
            let safe_y = src_y.min(current_size - 1);
            resized[y_28][x_28] = matrix[safe_y][safe_x];
        }
    }

    resized
}*/
pub fn resize_and_center(matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let old_height = matrix.len();
    if old_height == 0 {
        return Vec::new();
    }
    let old_width = matrix[0].len();
    if old_width == 0 {
        return Vec::new();
    }

    let mut min_r = old_height;
    let mut max_r = 0;
    let mut min_c = old_width;
    let mut max_c = 0;
    let mut found = false;

    for r in 0..old_height {
        for c in 0..old_width {
            // On détecte le chiffre qu'il soit écrit avec des 1 ou des 255
            if matrix[r][c] > 0 {
                if r < min_r {
                    min_r = r;
                }
                if r > max_r {
                    max_r = r;
                }
                if c < min_c {
                    min_c = c;
                }
                if c > max_c {
                    max_c = c;
                }
                found = true;
            }
        }
    }

    // Sécurité : Si l'image est vide, on renvoie une matrice noire standard
    if !found {
        return vec![vec![0u32; 28]; 28];
    }

    let digit_height = max_r - min_r + 1;
    let digit_width = max_c - min_c + 1;
    let digit_max_dim = digit_height.max(digit_width);

    // On impose un padding fixe de 4 pixels autour du chiffre
    let padding = 4;
    let new_dim = digit_max_dim + padding * 2;

    let mut new_matrix = vec![vec![0u32; new_dim]; new_dim];

    // Calcul du centrage strict
    let pad_top = padding + (digit_max_dim - digit_height) / 2;
    let pad_left = padding + (digit_max_dim - digit_width) / 2;

    for r in 0..digit_height {
        for c in 0..digit_width {
            // Sécurité anti-débordement
            if (r + pad_top) < new_dim && (c + pad_left) < new_dim {
                // On force la valeur à 255 pour que l'étape suivante fonctionne à coup sûr
                if matrix[min_r + r][min_c + c] > 0 {
                    new_matrix[r + pad_top][c + pad_left] = 255;
                }
            }
        }
    }

    new_matrix
}

pub fn resize_matrix_to_28x28(matrix: &Vec<Vec<u32>>, current_size: usize) -> Vec<Vec<u32>> {
    if current_size == 28 {
        return matrix.clone();
    }

    let mut resized = vec![vec![0u32; 28]; 28];
    let scale = current_size as f64 / 28.0;

    for y_28 in 0..28 {
        for x_28 in 0..28 {
            let src_x = x_28 as f64 * scale;
            let src_y = y_28 as f64 * scale;

            let x0 = (src_x.floor() as usize).min(current_size - 1);
            let y0 = (src_y.floor() as usize).min(current_size - 1);
            let x1 = (x0 + 1).min(current_size - 1);
            let y1 = (y0 + 1).min(current_size - 1);

            let fx = src_x - src_x.floor();
            let fy = src_y - src_y.floor();

            let v00 = matrix[y0][x0] as f64;
            let v10 = matrix[y0][x1] as f64;
            let v01 = matrix[y1][x0] as f64;
            let v11 = matrix[y1][x1] as f64;

            let value = v00 * (1.0 - fx) * (1.0 - fy)
                + v10 * fx * (1.0 - fy)
                + v01 * (1.0 - fx) * fy
                + v11 * fx * fy;

            // Puisque la fonction précédente force à 255, ce seuil à 120.0
            // préservera parfaitement les contours sans faire disparaître le chiffre.
            resized[y_28][x_28] = if value >= 120.0 { 255 } else { 0 };
        }
    }

    resized
}
