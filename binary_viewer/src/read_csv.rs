use std::fs::File;

pub fn read_data(path_digits: &str, path_labels: &str) -> Option<(Vec<Vec<u32>>, Vec<u32>)> {
    match process_files(path_digits, path_labels) {
        Ok(data) => Some(data),
        Err(e) => {
            eprintln!("Erreur lors de la lecture des données : {}", e);
            None
        }
    }
}

fn process_files(
    path_digits: &str,
    path_labels: &str,
) -> Result<(Vec<Vec<u32>>, Vec<u32>), Box<dyn std::error::Error>> {
    let mut digits: Vec<Vec<u32>> = Vec::new();
    let mut labels: Vec<u32> = Vec::new();

    // 1. LECTURE DES PIXELS
    let file_digits = File::open(path_digits)?;
    let mut rdr_digits = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_reader(file_digits);

    for result in rdr_digits.records() {
        let record = result?;

        let row: Vec<u32> = record
            .iter()
            .map(|s| s.trim().parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .map_err(|e| format!("Erreur de parsing pixel dans {}: {}", path_digits, e))?;

        digits.push(row);
    }

    // 2. LECTURE DES LABELS
    let file_labels = File::open(path_labels)?;
    let mut rdr_labels = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file_labels);

    for result in rdr_labels.records() {
        let record = result?;
        if let Some(first_col) = record.get(0) {
            let label: u32 = first_col
                .trim()
                .parse::<u32>()
                .map_err(|e| format!("Erreur de parsing label dans {}: {}", path_labels, e))?;
            labels.push(label);
        }
    }

    Ok((digits, labels))
}
