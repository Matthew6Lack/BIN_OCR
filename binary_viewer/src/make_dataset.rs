pub fn make_advance_dataset(digits: Vec<Vec<u32>>, labels: Vec<u32>) -> Vec<(Vec<u32>, f64)> {
    assert_eq!(
        digits.len(),
        labels.len(),
        "Les vecteurs digits et labels doivent avoir la même taille !"
    );

    digits
        .into_iter()
        .zip(labels.into_iter())
        .map(|(digit, label)| (digit, label as f64))
        .collect() // On rassemble le tout dans un nouveau Vec
}
