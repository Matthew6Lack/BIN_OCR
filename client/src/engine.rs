/*use crate::actions_client::{ClientMsg, ServerMsg};



pub struct ClientEngine {
    // Si vous utilisez le tableau statique BINARY_ITEMS généré automatiquement,
    // le vecteur d'images sert principalement à suivre la taille ou l'état local.
    pub images: Vec<String>,

    pub current_index: usize,

    pub last_ocr_result: Option<String>,
}

impl ClientEngine {
    pub fn new() -> Self {
        Self {
            images: vec![],
            current_index: 0,
            last_ocr_result: None,
        }
    }

    /// Action de chargement raccordée avec le serveur
    /// Appelle `load_img` pour obtenir la matrice et le nom custom.
    pub fn action_load<F>(&mut self, load_img: F) -> Option<(ClientMsg, Vec<ToUI>)>
    where
        F: FnOnce() -> Option<(Vec<Vec<u32>>, String)>,
    {
        // On appelle la fonction passée en paramètre pour récupérer la matrice et le nom
        if let Some((matrix, custom_name)) = load_img() {

            // On peut simuler l'ajout d'un chemin ou identifiant dans notre liste locale
            let mock_path = format!("src/img/{}", custom_name);
            self.images.push(mock_path);
            self.current_index = self.images.len() - 1;

            // On prépare le message attendu par la variante `EngineAction::Load` du serveur
            let msg = ClientMsg::Load {
                matrix,
                custom_name,
            };

            // On demande un rafraîchissement global de la page pour voir la nouvelle image
            let ui = vec![ToUI::RefreshUI];

            Some((msg, ui))
        } else {
            None
        }
    }

    pub fn action_find(&self) -> Option<ClientMsg> {
        if self.images.is_empty() {
            return None;
        }
        Some(ClientMsg::Find)
    }

    pub fn action_solve(&self) -> Option<ClientMsg> {
        if self.images.is_empty() {
            return None;
        }
        // Raccordé avec le serveur (UpdateViewedImage + Solve)
        Some(ClientMsg::Solve)
    }

    pub fn action_save(&self) -> Vec<ToUI> {
        match &self.last_ocr_result {
            Some(text) => vec![ToUI::SaveFileDialog(text.clone())],
            None => vec![ToUI::Error("Aucun résultat OCR à sauvegarder.".into())],
        }
    }

    pub fn previous_image(&mut self) -> (ClientMsg, Vec<ToUI>) {
        if self.images.is_empty() {
            // Valeurs par défaut si vide
            return (ClientMsg::UpdateViewedImage { index: 0 }, vec![]);
        }
        if self.current_index == 0 {
            self.current_index = self.images.len() - 1;
        } else {
            self.current_index -= 1;
        }

        // On informe le serveur du changement d'index de l'image vue
        let msg = ClientMsg::UpdateViewedImage { index: self.current_index };
        let ui = vec![ToUI::ShowImage {
            index: self.current_index,
            path: self.images[self.current_index].clone(),
        }];
        (msg, ui)
    }

    pub fn next_image(&mut self) -> (ClientMsg, Vec<ToUI>) {
        if self.images.is_empty() {
            return (ClientMsg::UpdateViewedImage { index: 0 }, vec![]);
        }
        self.current_index = (string_index + 1) % self.images.len();

        // On informe le serveur du changement d'index de l'image vue
        let msg = ClientMsg::UpdateViewedImage { index: self.current_index };
        let ui = vec![ToUI::ShowImage {
            index: self.current_index,
            path: self.images[self.current_index].clone(),
        }];
        (msg, ui)
    }

    /// Gestion des réponses asynchrones du serveur
    pub fn handle_server(&mut self, msg: ServerMsg) -> Vec<ToUI> {
        match msg {
            ServerMsg::ImageLoaded { new_index } => {
                self.current_index = new_index;
                // On rafraîchit la page/UI pour voir la nouvelle image dans la liste
                vec![ToUI::RefreshUI]
            }

            // Quand la réponse arrive pour le Find (reçu du oneshot serveur)
            ServerMsg::Positions(boxes) => {
                vec![ToUI::ShowBoxes(boxes)]
            }

            // Quand la réponse arrive pour le Solve (reçu du oneshot serveur)
            ServerMsg::OcrResult(text) => {
                self.last_ocr_result = Some(text.clone());
                vec![ToUI::ShowOcrResult(text)]
            }

            ServerMsg::Error(e) => {
                vec![ToUI::Error(e)]
            }
        }
    }
}*/
