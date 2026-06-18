use crate::{
    binarisation::*, clear::*, conv_toimg::*, convertpgm::*, cut::*, find_edge::*, make_dataset::*,
    midsized_letter::*, neural_network::*, read_csv::*, resizing::*, solution::*, find_imgposition::*,
};


pub fn run_ocr(ind: usize) -> Option<(i32, i32)> {
    if let Err(e) = clear_digits_folder() {
        eprintln!("Attention, impossible de vider le dossier 'digits' : {:?}", e);
        return None;
    }
    let mut nb_signed = 0;
    let mut nbin = 0;
     
    let path = find_img_position(ind)?;

    let mut dataset_train: Vec<(Vec<u32>, f64)> = vec![];
    if let Some((datatrain, label_training)) = read_data(
        &"src/TRAINING/datatrain_28x28.csv",
        &"src/TRAINING/label_train.csv",
    ) {
        dataset_train = make_advance_dataset(datatrain, label_training);
    }
    if dataset_train.is_empty(){println!("read data don't work");}
    //pour connaitre mon data set et l'utiliser
    /* 
    let mut dataset_test: Vec<(Vec<u32>, f64)> = vec![];
    if let Some((datatest, label_testing)) =
        read_data(&"src/TEST/datatest_28x28.csv", &"src/TEST/label_test.csv")
    {
        dataset_test = make_advance_dataset(datatest, label_testing);
    }

    if !dataset_test.is_empty() {
        println!("\n=== INSPECTION GÉOMÉTRIQUE DU DATASET (CSV) ===");
        
        // On cherche le premier '0' et le premier '1' du CSV pour les afficher
        let mut trouve_zero = false;
        let mut trouve_un = false;

        for (pixels, label) in dataset_test.iter() {
            if *label == 0.0 && !trouve_zero {
                println!("\n--- UN '0' DANS LE CSV (Label: 0) ---");
                afficher_matrice_784(pixels);
                trouve_zero = true;
            }
            if *label == 1.0 && !trouve_un {
                println!("\n--- UN '1' DANS LE CSV (Label: 1) ---");
                afficher_matrice_784(pixels);
                trouve_un = true;
            }
            if trouve_zero && trouve_un { break; }
        }
    }*/


    //Période TRAINING: conclusion epochs = 50; lr = 0.01; hidden = 32; nombres d'erreurs 1/2115; taux de réussite: 99.91%
    /* 
    let epochs: usize = 50;
    let lr: f64 = 0.01;
    let hidden: usize = 1;
    let mut mlp = Mlp::new(784, hidden, lr);
    mlp.fit(&dataset_train, epochs);
    if let Some(my_mlp) = find_good_mlp(&dataset_train, &dataset_test) {
        mlp = my_mlp;
    }*/
    let mlp_intelligent = Mlp::load_from_file("weights.bin")
        .expect("Erreur : Le fichier weights.bin est introuvable !");

    let (pgm_path, w, h) = convert_to_pgm(&path).ok()?;
    let bin = binarize_pgm(&pgm_path, w, h).ok()?;
    if bin.is_empty() {
        println!("Erreur : Le vecteur binaire est vide!");
        return None;
    }

    let mut verif = false;
    for row in bin.iter() {
        if row.iter().any(|&pixel| pixel == 0) {
            verif = true;
            break;
        }
    }
    if !verif {
        println!("Erreur : l'image ne contient pas de nombre binaire!");
        return None;
    }

    let (y, x, ymax, xmax) = search_edges(&bin)?;
    println!(
        "la position du nombre binaire est: x = {}, y = {}, \
         xmax = {}, ymax = {}",
        x, y, xmax, ymax
    );
    let nb_bin = reduce_bin(&bin, x, y, xmax, ymax);

    let size = midsized_digit(&nb_bin)?;
    let digits = find_digit(&nb_bin, size.avg_width, size.avg_height)?;

    let mut new = Vec::new();
    let mut flatten_digit: Vec<Vec<u32>> = Vec::new();

    for d in digits {
        let mut digit = reduce_bin(&nb_bin, d.0, d.1, d.2, d.3);
        digit = resize_and_center(&digit);
        new.push(digit);
    }
    for i in 0..new.len() {
        new[i] = resize_matrix_to_28x28(&new[i], new[i].len());
        flatten_digit.push(new[i].iter().flatten().copied().collect());

        if let Ok(path_digit) = convert_to_img(&new[i]) {
            print!("nouvelle image: {}; ", path_digit);
        } else {
            println!("Erreur: la conversion en image a échoué!");
        }
    }

    if !flatten_digit.is_empty() {
        let input_size = flatten_digit[0].len();
        println!("Taille vecteur aplati : {} pixels", input_size);
        let mut result = String::new();
        for pixels in &flatten_digit {
            let ch: char = mlp_intelligent.predict(pixels);
            result.push(ch);
        }
        nb_signed = bin_todeci(result.clone(), true);
        nbin = bin_todeci(result.clone(), false);
        println!("Chaîne binaire détectée : {}", nbin);
        println!("Résultat final (signé) : {}", nb_signed);
    }

    Some((nb_signed, nbin))
}
pub fn extract_position(ind: usize) -> Option<Vec<(usize, usize, usize, usize)>> {
    let path = find_img_position(ind)?;


    let (pgm_path, w, h) = convert_to_pgm(&path).ok()?;
    let bin = binarize_pgm(&pgm_path, w, h).ok()?;
    if bin.is_empty() {
        println!("Erreur : Le vecteur binaire est vide!");
        return None;
    }

    let mut verif = false;
    for row in bin.iter() {
        if row.iter().any(|&pixel| pixel == 0) {
            verif = true;
            break;
        }
    }
    if !verif {
        println!("Erreur : l'image ne contient pas de nombre binaire!");
        return None;
    }


    let size = midsized_digit(&bin)?;
    
    let digits = find_digit(&bin, size.avg_width, size.avg_height)?;

    Some(digits)
}
















//afin de connaitre mon dataset 
/* 
fn afficher_matrice_784(pixels: &[u32]) {
    for r in 0..28 {
        let mut row_str = String::new();
        for c in 0..28 {
            let idx = r * 28 + c;
            if idx < pixels.len() {
                // Si le pixel est > 0 (ou 128 selon ton encodage), on met un #
                row_str.push_str(if pixels[idx] > 0 { "# " } else { ". " });
            }
        }
        println!("{}", row_str);
    }
}*/