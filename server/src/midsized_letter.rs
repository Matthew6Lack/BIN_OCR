#[derive(Debug, Clone, Copy)]
pub struct ComponentSize {
    pub avg_width: usize,
    pub avg_height: usize,
}

#[derive(Debug, Clone, Copy)]
struct BoundingBox {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

/// Fonction outil pour inverser une image (le noir devient blanc, le blanc devient noir)
fn invert_binary_image(matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(|&pixel| if pixel == 0 { 1 } else { 0 }) // Transforme le noir en 1 et le reste en 0
                .collect()
        })
        .collect()
}

/// Réalise la méthode flood fill
fn flood_fill_bounding_box(
    pixels: &mut Vec<Vec<u32>>,
    start_x: usize,
    start_y: usize,
) -> Option<BoundingBox> {
    let height = pixels.len();
    if height == 0 {
        return None;
    }
    let width = pixels[0].len();

    if pixels[start_y][start_x] == 0 {
        return None;
    }

    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push((start_x, start_y));

    let mut bbox = BoundingBox {
        min_x: start_x,
        max_x: start_x,
        min_y: start_y,
        max_y: start_y,
    };

    while let Some((cx, cy)) = stack.pop() {
        if pixels[cy][cx] == 0 {
            continue;
        }

        pixels[cy][cx] = 0;

        if cx < bbox.min_x {
            bbox.min_x = cx;
        }
        if cx > bbox.max_x {
            bbox.max_x = cx;
        }
        if cy < bbox.min_y {
            bbox.min_y = cy;
        }
        if cy > bbox.max_y {
            bbox.max_y = cy;
        }

        if cx + 1 < width && pixels[cy][cx + 1] != 0 {
            stack.push((cx + 1, cy));
        }
        if cx > 0 && pixels[cy][cx - 1] != 0 {
            stack.push((cx - 1, cy));
        }
        if cy + 1 < height && pixels[cy + 1][cx] != 0 {
            stack.push((cx, cy + 1));
        }
        if cy > 0 && pixels[cy - 1][cx] != 0 {
            stack.push((cx, cy - 1));
        }
    }

    Some(bbox)
}

/// Retourn un struct ComponentSize contenant à la fois la hauteur moyenne et la largeur moyenne du chiffre
pub fn midsized_digit(binary_image: &Vec<Vec<u32>>) -> Option<ComponentSize> {
    if binary_image.is_empty() || binary_image[0].is_empty() {
        return None;
    }

    let mut pixels = if binary_image[0][0] != 0 {
        invert_binary_image(binary_image)
    } else {
        binary_image.clone()
    };

    let height = pixels.len();
    let width = pixels[0].len();

    let mut count: u64 = 0;
    let mut total_width: u64 = 0;
    let mut total_height: u64 = 0;

    for y in 0..height {
        for x in 0..width {
            if pixels[y][x] != 0 {
                if let Some(bbox) = flood_fill_bounding_box(&mut pixels, x, y) {
                    let component_width = (bbox.max_x - bbox.min_x + 1) as u64;
                    let component_height = (bbox.max_y - bbox.min_y + 1) as u64;
                    total_width += component_width;
                    total_height += component_height;
                    count += 1;
                }
            }
        }
    }

    if count > 0 {
        Some(ComponentSize {
            avg_width: (total_width / count) as usize,
            avg_height: (total_height / count) as usize,
        })
    } else {
        None
    }
}

/// localise le chiffre et retourne ces cordonné dans un Vec<(usize,usize,usize,usize)>
pub fn find_digit(
    binary_image: &Vec<Vec<u32>>,
    avg_width: usize,
    avg_height: usize,
) -> Option<Vec<(usize, usize, usize, usize)>> {
    if binary_image.is_empty() || binary_image[0].is_empty() {
        return None;
    }

    let mut pixels = if binary_image[0][0] != 0 {
        invert_binary_image(binary_image)
    } else {
        binary_image.clone()
    };

    let height = pixels.len();
    let width = pixels[0].len();

    let min_width = (avg_width as f64 * 0.25).ceil() as usize;
    let min_height = (avg_height as f64 * 0.65).ceil() as usize;

    let avg_area = (avg_width * avg_height) as f64;

    let mut digits: Vec<(usize, usize, usize, usize)> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if pixels[y][x] != 0 {
                if let Some(bbox) = flood_fill_bounding_box(&mut pixels, x, y) {
                    let component_width = bbox.max_x - bbox.min_x + 1;
                    let component_height = bbox.max_y - bbox.min_y + 1;
                    let component_area = (component_width * component_height) as f64;

                    let dimensions_ok =
                        component_width >= min_width && component_height >= min_height;

                    let area_ok = component_area >= (avg_area * 0.20);
                    let ratio = component_height as f64 / component_width as f64;
                    let shape_ok = ratio > 0.2 && ratio < 5.0;

                    if dimensions_ok && area_ok && shape_ok {
                        digits.push((bbox.min_x, bbox.min_y, bbox.max_x, bbox.max_y));
                    }
                }
            }
        }
    }

    if digits.is_empty() {
        None
    } else {
        digits.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        Some(digits)
    }
}
