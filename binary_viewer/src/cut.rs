//créer un tableau réduit afin d'avoir uniquement le nombre binaire ou un des chiffre que l'on souhaite.
pub fn reduce_bin(
    matrix: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    xmax: usize,
    ymax: usize,
) -> Vec<Vec<u32>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }

    let height = matrix.len();
    let width = matrix[0].len();

    if x >= xmax || y >= ymax || xmax > width || ymax > height {
        return matrix.clone();
    }

    let h = ymax - y;
    let w = xmax - x;
    let mut new_matrix: Vec<Vec<u32>> = Vec::with_capacity(h);

    for j in 0..h {
        let mut row: Vec<u32> = Vec::with_capacity(w);
        for i in 0..w {
            row.push(matrix[j + y][i + x]);
        }
        new_matrix.push(row);
    }

    new_matrix
}
