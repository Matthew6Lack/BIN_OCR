use std::fs::File;
use std::io::{BufRead, BufReader, Read};

/// Binarizes a PGM image
/// If a pixel is darker than light beige claire, it becomes 0 (black), else 1 (white).

pub fn binarize_pgm(path: &str, width: u32, height: u32) -> Result<Vec<Vec<u32>>, String> {
    let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let mut header_lines_skipped = 0;

    while header_lines_skipped < 3 {
        line.clear();
        reader
            .read_line(&mut line)
            .map_err(|_| "Failed to read header")?;

        if !line.trim().starts_with('#') && !line.trim().is_empty() {
            header_lines_skipped += 1;
        }
    }

    let mut rest_of_file = String::new();
    reader
        .read_to_string(&mut rest_of_file)
        .map_err(|e| format!("Failed to read pixels as text: {}", e))?;

    let mut pixels: Vec<u32> = Vec::new();
    for token in rest_of_file.split_whitespace() {
        if let Ok(value) = token.parse::<u32>() {
            pixels.push(value);
        }
    }

    if pixels.len() < (width * height) as usize {
        return Err(format!(
            "Pas assez de pixels. Attendu : {}, Trouvé : {}",
            width * height,
            pixels.len()
        ));
    }

    let mut matrix = Vec::with_capacity(height as usize);

    for y in 0..height as usize {
        let mut row = Vec::with_capacity(width as usize);
        for x in 0..width as usize {
            let index = y * (width as usize) + x;
            let pixel_value = pixels[index];

            let binary_pixel = if pixel_value < 220 { 1 } else { 0 };
            row.push(binary_pixel);
        }
        matrix.push(row);
    }

    Ok(matrix)
}
