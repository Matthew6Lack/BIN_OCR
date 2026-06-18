use bincode;
use serde::{Deserialize, Serialize};
use std::f64;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize)]
pub struct Mlp {
    hidden: Layer,
    output: Layer,
    pub learning_rate: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Layer {
    weights: Vec<Vec<f64>>,
    biases: Vec<f64>,
    n_inputs: usize,
    n_neurons: usize,
    is_output: bool,
}

fn relu(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        0.0
    }
}

fn leaky_relu(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        0.01 * x
    }
}
fn leaky_relu_deriv(activated_val: f64) -> f64 {
    if activated_val > 0.0 {
        1.0
    } else {
        0.01
    }
}

fn softmax(logits: &[f64]) -> Vec<f64> {
    let max_val = logits.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let exps: Vec<f64> = logits.iter().map(|&x| (x - max_val).exp()).collect();
    let sum_exps: f64 = exps.iter().sum();
    exps.iter().map(|&x| x / sum_exps).collect()
}

#[inline]
fn normalize_pixel(p: u32) -> f64 {
    1.0 - (p as f64 / 255.0)
}

impl Layer {
    fn new(n_inputs: usize, n_neurons: usize, is_output: bool) -> Self {
        let limit = if is_output {
            (6.0_f64 / (n_inputs + n_neurons) as f64).sqrt()
        } else {
            (2.0_f64 / n_inputs as f64).sqrt()
        };

        let mut weights = vec![vec![0.0_f64; n_neurons]; n_inputs];
        let mut seed: u64 = 0x853c_49e6_748f_ea9b;

        for row in weights.iter_mut() {
            for w in row.iter_mut() {
                seed = seed
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                let norm = (seed >> 11) as f64 / (1u64 << 53) as f64;
                *w = norm * 2.0 * limit - limit;
            }
        }
        Layer {
            weights,
            biases: vec![0.0; n_neurons],
            n_inputs,
            n_neurons,
            is_output,
        }
    }

    fn forward_raw(&self, inputs: &[f64]) -> (Vec<f64>, Vec<f64>) {
        let mut z = self.biases.clone();
        for (i, &x) in inputs.iter().enumerate() {
            for j in 0..self.n_neurons {
                z[j] += x * self.weights[i][j];
            }
        }

        let a = if self.is_output {
            z.clone()
        } else {
            z.iter().map(|&v| relu(v)).collect()
        };

        (a, z)
    }
}

impl Mlp {
    pub fn new(input_size: usize, hidden_size: usize, lr: f64) -> Self {
        Mlp {
            hidden: Layer::new(input_size, hidden_size, false),
            output: Layer::new(hidden_size, 2, true),
            learning_rate: lr,
        }
    }

    fn predict_raw(&self, inputs: &[f64]) -> Vec<f64> {
        let (a_h, _) = self.hidden.forward_raw(inputs);
        let (logits, _) = self.output.forward_raw(&a_h);
        softmax(&logits)
    }

    pub fn predict(&self, pixels: &[u32]) -> char {
        let inputs_vec: Vec<f64> = pixels.iter().map(|&x| normalize_pixel(x)).collect();
        let probs = self.predict_raw(&inputs_vec);
        if probs[1] > probs[0] {
            '1'
        } else {
            '0'
        }
    }

    fn train_one(&mut self, inputs: &[f64], target: f64) {
        let target_idx = target as usize;
        let mut y_hot = vec![0.0, 0.0];
        y_hot[target_idx] = 1.0;

        // Passe avant (forward)
        let (a_h, _) = self.hidden.forward_raw(inputs);
        let (logits, _) = self.output.forward_raw(&a_h);
        let probs = softmax(&logits);

        let mut hidden_morts = 0;
        for &val in &a_h {
            if val == 0.0 {
                hidden_morts += 1;
            }
        }
        /*if hidden_morts == a_h.len() {
            println!("[CRITIQUE] Tous les neurones de la couche cachée sont morts (ReLU = 0) !");
        }*/

        let mut delta_o = vec![0.0; 2];
        for k in 0..2 {
            delta_o[k] = probs[k] - y_hot[k];
        }

        let mut delta_h = vec![0.0_f64; self.hidden.n_neurons];
        for j in 0..self.hidden.n_neurons {
            let mut err = 0.0;
            for k in 0..2 {
                err += delta_o[k] * self.output.weights[j][k];
            }
            delta_h[j] = err * leaky_relu_deriv(a_h[j]);
        }

        let max_delta_h = delta_h.iter().map(|d| d.abs()).fold(0.0, f64::max);
        /*println!(
            "[GRADIENT] delta_o_max = {:.6} | Max |delta_h| = {:.6} | LR appliqué = {}",
            delta_o.iter().map(|d| d.abs()).fold(0.0, f64::max),
            max_delta_h,
            self.learning_rate
        );*/

        for j in 0..self.hidden.n_neurons {
            for k in 0..2 {
                self.output.weights[j][k] -= self.learning_rate * delta_o[k] * a_h[j];
            }
        }
        for k in 0..2 {
            self.output.biases[k] -= self.learning_rate * delta_o[k];
        }

        for i in 0..self.hidden.n_inputs {
            for j in 0..self.hidden.n_neurons {
                self.hidden.weights[i][j] -= self.learning_rate * delta_h[j] * inputs[i];
            }
        }
        for j in 0..self.hidden.n_neurons {
            self.hidden.biases[j] -= self.learning_rate * delta_h[j];
        }
    }
    pub fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(filename)?;

        let encoded: Vec<u8> = bincode::serialize(&self)?;

        file.write_all(&encoded)?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(filename)?;
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer)?;

        let network: Self = bincode::deserialize(&buffer)?;
        Ok(network)
    }
    pub fn fit(&mut self, data: &[(Vec<u32>, f64)], epochs: usize) {
        if let Some((premiers_pixels, premier_target)) = data.first() {
            let inputs_vec: Vec<f64> = premiers_pixels
                .iter()
                .map(|&x| normalize_pixel(x))
                .collect();
            let blancs = inputs_vec.iter().filter(|&&x| x > 0.8).count();
            let noirs = inputs_vec.iter().filter(|&&x| x < 0.2).count();

            println!(
                "[DATA INSPECT] Première image du dataset : {} pixels analysés. Pixels actifs (>0.8) : {} | Pixels de fond (<0.2) : {}. Cible = {}",
                inputs_vec.len(),
                blancs,
                noirs,
                premier_target
            );
        }

        let mut previous_loss = f64::MAX;

        for epoch in 0..epochs {
            let mut total_loss = 0.0_f64;

            for (pixels, target) in data {
                let inputs_vec: Vec<f64> = pixels.iter().map(|&x| normalize_pixel(x)).collect();

                let probs = self.predict_raw(&inputs_vec);
                let target_idx = *target as usize;

                total_loss -= (probs[target_idx] + 1e-15).ln();

                self.train_one(&inputs_vec, *target);
            }

            let avg_loss = total_loss / data.len() as f64;

            if epoch % 10 == 0 || epoch == epochs - 1 {
                println!(
                    "Epoch {:>4} / {} – Cross-Entropy Loss: {:.6}",
                    epoch + 1,
                    epochs,
                    avg_loss
                );
            }

            if avg_loss.is_nan() {
                println!("[ARRÊT CRITIQUE] La Loss est devenue NaN (Explosion des gradients).");
                break;
            }

            if (previous_loss - avg_loss).abs() < 1e-6 {
                println!(
                    "→ Arrêt automatique à l'époque {} : Le modèle a convergé (Loss stable).",
                    epoch + 1
                );
                break;
            }

            previous_loss = avg_loss;
        }
    }
}

pub fn find_good_mlp(
    data_train: &Vec<(Vec<u32>, f64)>,
    data_test: &Vec<(Vec<u32>, f64)>,
) -> Option<Mlp> {
    if data_train.is_empty() || data_test.is_empty() {
        println!("Erreur : Dataset d'entraînement ou de test manquant.");
        return None;
    }

    let input_size = data_train[0].0.len();
    let mut current_epochs: usize = 200;
    let mut current_lr: f64 = 0.5;
    let mut current_hidden: usize = 1;

    loop {
        println!("\n==================================================");
        println!(
            "→ Configuration : Epochs: {}, LR: {}, Hidden Size: {}",
            current_epochs, current_lr, current_hidden
        );

        let mut mlp = Mlp::new(input_size, current_hidden, current_lr);
        mlp.fit(data_train, current_epochs);

        println!("\n--- Phase de Test sur le Jeu de Données Dédié ---");

        let mut erreurs = 0;
        let total_test = data_test.len();

        if total_test == 0 {
            println!("Attention : Aucun lot de test fourni.");
            return None;
        }

        for (pixels, target) in data_test {
            let prediction_char = mlp.predict(pixels);
            let prediction_val = if prediction_char == '1' { 1.0 } else { 0.0 };

            if prediction_val != *target {
                erreurs += 1;
            }
        }

        let taux_erreur = (erreurs as f64 / total_test as f64) * 100.0;
        let taux_reussite = 100.0 - taux_erreur;

        println!("Nombre d'erreurs : {} / {}", erreurs, total_test);
        println!("Taux de Réussite  : {:.2}%", taux_reussite);
        println!("Taux d'Erreur     : {:.2}%", taux_erreur);

        println!("\nLe résultat vous convient-il ?");
        println!(" -> Appuyez sur [Entrée] pour SAUVEGARDER le modèle et quitter.");
        println!(
            " -> Entrez de nouvelles valeurs au format 'EPOCHS LR HIDDEN' (ex: '25000 0.1 16') pour réessayer."
        );

        print!("Votre choix : ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        if trimmed.is_empty() {
            println!("Validation du modèle ! Enregistrement en cours...");
            match mlp.save_to_file("weights.bin") {
                Ok(_) => println!("✓ Modèle sérialisé avec succès dans 'weights.bin' !"),
                Err(e) => eprintln!("✕ Erreur d'écriture : {}", e),
            }
            return Some(mlp);
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() == 3 {
            let parsed_epochs = parts[0].parse::<usize>();
            let parsed_lr = parts[1].parse::<f64>();
            let parsed_hidden = parts[2].parse::<usize>();

            match (parsed_epochs, parsed_lr, parsed_hidden) {
                (Ok(new_epochs), Ok(new_lr), Ok(new_hidden)) => {
                    // SÉCURITÉ : Vérification de la contrainte (entre 4 et 64)
                    if new_hidden >= 4 && new_hidden <= 64 {
                        current_epochs = new_epochs;
                        current_lr = new_lr;
                        current_hidden = new_hidden;
                        println!(
                            "Nouvelles valeurs enregistrées. Relancement de l'entraînement..."
                        );
                    } else {
                        println!(
                            "✕ Erreur : La valeur de hidden_size doit être comprise strictement entre 4 et 64 !"
                        );
                    }
                }
                _ => {
                    println!("Saisie incorrecte. Impossible de lire les nombres.");
                }
            }
        } else {
            println!(
                "Format invalide. Exemple attendu : '30000 0.1 16' (reçu {} arguments)",
                parts.len()
            );
        }
    }
}
