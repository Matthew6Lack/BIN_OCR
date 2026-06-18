use tokio::sync::oneshot;
use std::path::Path;
use std::fs;
use crate::actions_server::*;
use crate::conv_toimg::*;
// Les actions que le client peut envoyer au serveur
pub enum EngineAction {
    Load { 
        matrix: Vec<Vec<u32>>,
        custom_name: String, 
    },
    UpdateViewedImage { 
        index: usize 
    },
    Solve { 
        respond_to: oneshot::Sender<Option<(i32, i32)>> 
    },
    Find { 
        respond_to: oneshot::Sender<Option<Vec<(usize, usize, usize, usize)>>> 
    },
}

pub struct EngineServer {
    receiver: tokio::sync::mpsc::Receiver<EngineAction>,
    current_viewed_index: Option<usize>,
}

impl EngineServer {
    pub fn new(receiver: tokio::sync::mpsc::Receiver<EngineAction>) -> Self {
        Self {
            receiver,
            current_viewed_index: None,
        }
    }

    // Boucle continue du serveur
    pub fn start(mut self) {
        tokio::spawn(async move {
            println!("Serveur Engine actif...");

            while let Some(action) = self.receiver.recv().await {
                match action {
                    EngineAction::Load { matrix, custom_name } => {
                        println!("Action: Load reçue avec le nom final : {}", custom_name);
                        
                        match convert_to_img(&matrix) {
                            Ok(generated_name) => {
                                let source_path = Path::new("src/digits").join(generated_name);
                                
                                let dest_path = Path::new("src/images").join(custom_name);

                                if let Err(e) = fs::rename(&source_path, &dest_path) {
                                    eprintln!(
                                        "Erreur lors du déplacement/renommage de {:?} vers {:?} : {:?}", 
                                        source_path, dest_path, e
                                    );
                                } else {
                                    println!("Image renommée et déplacée avec succès !");
                                }
                            }
                            Err(e) => {
                                eprintln!("Erreur lors de la conversion de l'image : {}", e);
                            }
                        }
                    }

                    EngineAction::UpdateViewedImage { index } => {
                        self.current_viewed_index = Some(index);
                    }

                    EngineAction::Solve { respond_to } => {
                        println!("Action: Solve demandée.");
                        if let Some(index) = self.current_viewed_index {
                            let result = run_ocr(index); 
                            let _ = respond_to.send(result);
                        } else {
                            let _ = respond_to.send(None); // Pas d'image active
                        }
                    }

                    EngineAction::Find { respond_to } => {
                        println!("Action: Find demandée.");
                        if let Some(index) = self.current_viewed_index {
                            // On appelle la fonction de main.rs
                            let result = extract_position(index);
                            let _ = respond_to.send(result); 
                        } else {
                            let _ = respond_to.send(None);
                        }
                    }
                }
            }
        });
    }
}