use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, image, row, stack, text, text_input, Space},
    Border, Color, Element, Font, Length, Pixels, Task, Theme,
};
mod load_image;
use iced::widget::canvas;
use load_image::load_img;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

const BINARY_ITEMS: &[&str] = &[
    "src/img/BINimg1.png",
    "src/img/BINimg2.png",
    "src/img/BINimg3.png",
];

#[derive(Debug, Clone)]
enum WorkerResponse {
    FindDone(Vec<(usize, usize, usize, usize)>),
    LoadDone(Vec<Vec<u32>>, String), 
    SolveDone(i32, i32),
    Error(String),
}

#[derive(Debug)]
enum WorkerRequest {
    Find {
        // 2. On lance le thread du serveur (worker_loop)
        std::thread::spawn(move || worker_loop(req_rx, resp_tx));
 
        // 3. On initialise TOUS les champs de la structure
        image_path: String,
    },
    Load {
        image_path: String,
        liste_images: Vec<String>,
    },
    Solve {
        image_path: String,
    },
}

#[derive(Debug, Clone)]
enum Message {
    Previous,
    Next,
    Find,
    Load,
    Save,
    Solve,
    WorkerReply(WorkerResponse),
    TextInputChanged(String),
}
#[derive(Debug)]
struct BinaryViewer {
    current_index: usize,
    liste_images: Vec<String>,
    bounding_boxes: Vec<(usize, usize, usize, usize)>,

    loaded_image_path: Option<String>,

    solve_result: Option<(i32, i32)>,

    status_text: String,

    user_text: String,

    worker_tx: Sender<WorkerRequest>,

    worker_rx: Arc<Mutex<Receiver<WorkerResponse>>>,
}
impl Default for BinaryViewer {
    fn default() -> Self {
        // 1. On crée les canaux pour communiquer avec le Worker
        let (req_tx, req_rx) = mpsc::channel::<WorkerRequest>();
        let (resp_tx, resp_rx) = mpsc::channel::<WorkerResponse>();
 
        // 2. On lance le thread du serveur (worker_loop)
        std::thread::spawn(move || worker_loop(req_rx, resp_tx));
 
        // 3. On initialise TOUS les champs de la structure
        Self {
            current_index: 0,
            liste_images: BINARY_ITEMS.iter().map(|s| s.to_string()).collect(),
            bounding_boxes: Vec::new(),
            loaded_image_path: None,
            solve_result: None,
            status_text: String::new(),
            user_text: String::new(),
            worker_tx: req_tx,
            worker_rx: Arc::new(Mutex::new(resp_rx)),
        }
    }
}
impl BinaryViewer {
    fn new() -> Self {
        let (req_tx, req_rx) = mpsc::channel::<WorkerRequest>();
        let (resp_tx, resp_rx) = mpsc::channel::<WorkerResponse>();

        // Lance le worker dans un thread dédié
        std::thread::spawn(move || worker_loop(req_rx, resp_tx));

        Self {
            current_index: 0,
            liste_images: BINARY_ITEMS.iter().map(|s| s.to_string()).collect(),
            bounding_boxes: Vec::new(),
            loaded_image_path: None,
            solve_result: None,
            status_text: String::new(),
            user_text: String::new(),
            worker_tx: req_tx,
            worker_rx: Arc::new(Mutex::new(resp_rx)),
        }
    }
    fn send_to_worker(&self, req: WorkerRequest) -> Task<Message> {
        self.worker_tx.send(req).ok();
        let rx = Arc::clone(&self.worker_rx);
        Task::perform(
            async move {
                tokio::task::spawn_blocking(move || {
                    rx.lock()
                        .unwrap()
                        .recv()
                        .unwrap_or(WorkerResponse::Error("Canal fermé".to_string()))
                })
                .await
                .unwrap_or(WorkerResponse::Error("Erreur tokio".to_string()))
            },
            Message::WorkerReply,
        )
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Previous => {
                if self.current_index == 0 {
                    self.current_index = self.liste_images.len() - 1;
                } else {
                    self.current_index -= 1;
                }
                self.bounding_boxes.clear();
                self.loaded_image_path = None;
                self.user_text.clear();
                self.status_text.clear();
                Task::none()
            }
            Message::Next => {
                self.current_index = (self.current_index + 1) % self.liste_images.len();
                self.bounding_boxes.clear();
                self.loaded_image_path = None;
                self.status_text.clear();
                self.user_text.clear();
                Task::none()
            }

            Message::Find => {
                self.status_text = "Recherche en cours…".to_string();
                let path = self.liste_images[self.current_index].clone();
                self.send_to_worker(WorkerRequest::Find { image_path: path })
            }

            Message::Load => {
                self.status_text = "Chargement en cours…".to_string();
                self.send_to_worker(WorkerRequest::Load)
            }
            Message::Save => {
                let content = build_save_content(
                    self.current_index,
                    &self.liste_images,
                    &self.solve_result,
                    &self.user_text,
                );
                let save_path = format!(
                    "result_{}.txt",
                    self.liste_images[self.current_index]
                        .replace('/', "_")
                        .replace(".png", "")
                );
                match std::fs::write(&save_path, &content) {
                    Ok(_) => {
                        self.status_text = format!("Sauvegardé : {save_path}");
                    }
                    Err(e) => {
                        self.status_text = format!("Erreur sauvegarde : {e}");
                    }
                }
                Task::none()
            }

            Message::Solve => {
                self.status_text = "Résolution en cours…".to_string();
                let path = self.liste_images[self.current_index].clone();
                self.send_to_worker(WorkerRequest::Solve { image_path: path })
            }

            Message::WorkerReply(resp) => {
                match resp {
                    WorkerResponse::FindDone(boxes) => {
                        self.status_text =
                            format!("Find : {} rectangle(s) détecté(s)", boxes.len());
                        self.bounding_boxes = boxes;
                    }
                    WorkerResponse::LoadDone(_pixels, copied_orig_path, final_path) => {
                        self.status_text = format!("Chargé avec succès !");
                        
                        if !self.liste_images.contains(&copied_orig_path) {
                            self.liste_images.push(copied_orig_path.clone());
                        }
                        
                        if !self.liste_images.contains(&final_path) {
                            self.liste_images.push(final_path.clone());
                        }

                        self.loaded_image_path = Some(final_path.clone());
                        
                        if let Some(pos) = self.liste_images.iter().position(|x| x == &final_path) {
                            self.current_index = pos;
                        }
                    }
                    WorkerResponse::SolveDone(a, b) => {
                        self.solve_result = Some((a, b));
                        self.status_text =
                            format!("Résultat Solve : valeur_a = {a}, valeur_b = {b}");
                        self.user_text = format!(
                            "la valeur décimal du mot binaire dans le cas d'un entier signé est {} et dans le cas contraire {}",
                            a, b
                        );
                    }
                    WorkerResponse::Error(e) => {
                        self.status_text = format!("Erreur : {e}");
                    }
                }
                Task::none()
            }
            Message::TextInputChanged(val) => {
                self.user_text = val;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let background = image(image::Handle::from_bytes(
            include_bytes!("assets/fond_app.png").to_vec(),
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .content_fit(iced::ContentFit::Cover);

        let arrow_left = button(
            image(image::Handle::from_bytes(
                include_bytes!("assets/blue_arrow_left.png").to_vec(),
            ))
            .width(Length::Fixed(50.0))
            .height(Length::Fixed(50.0)),
        )
        .on_press(Message::Previous)
        .style(|_theme, _status| button::Style {
            background: None,
            border: Border::default(),
            ..Default::default()
        })
        .padding(4);

        let arrow_right = button(
            image(image::Handle::from_bytes(
                include_bytes!("assets/blue_arrow_right.png").to_vec(),
            ))
            .width(Length::Fixed(50.0))
            .height(Length::Fixed(50.0)),
        )
        .on_press(Message::Next)
        .style(|_theme, _status| button::Style {
            background: None,
            border: Border::default(),
            ..Default::default()
        })
        .padding(4);
        let viewer_content: Element<'_, Message> = {
            let img_path = self
                .loaded_image_path
                .clone()
                .unwrap_or_else(|| self.liste_images[self.current_index].clone());

            if img_path.is_empty() {
                text("Aucune image disponible").size(18).into()
            } else if std::path::Path::new(&img_path).exists() {
                // Superposition image + canvas pour les rectangles
                let img_widget = image(image::Handle::from_path(img_path))
                    .width(Length::Fill)
                    .height(Length::Fixed(200.0))
                    .content_fit(iced::ContentFit::Contain);

                if self.bounding_boxes.is_empty() {
                    img_widget.into()
                } else {
                    // Canvas par-dessus pour tracer les rectangles rouges
                    let boxes = self.bounding_boxes.clone();
                    let overlay = canvas(BBoxOverlay { boxes })
                        .width(Length::Fill)
                        .height(Length::Fixed(200.0));
                    stack![img_widget, overlay].into()
                }
            } else {
                text(format!("Image introuvable :\n{img_path}"))
                    .size(16)
                    .color(Color::from_rgb(0.7, 0.2, 0.2))
                    .into()
            }
        };
        let binary_display = container(viewer_content)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(Color::WHITE)),
                border: Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .width(Length::Fill)
            .height(Length::Fixed(250.0)) // Encadre l'image dans une zone fixe
            .padding(12)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);

        let slider_row = row![arrow_left, binary_display, arrow_right,]
            .spacing(12)
            .align_y(Vertical::Center)
            .width(Length::Fill);

        let make_btn = |img_name: &'static str, _label: &'static str, msg: Message| {
            let handle = match img_name {
                "find.png" => image::Handle::from_bytes(include_bytes!("assets/find.png").to_vec()),
                "load.png" => image::Handle::from_bytes(include_bytes!("assets/load.png").to_vec()),
                "save.png" => image::Handle::from_bytes(include_bytes!("assets/save.png").to_vec()),
                "solve.png" => {
                    image::Handle::from_bytes(include_bytes!("assets/solve.png").to_vec())
                }
                _ => image::Handle::from_bytes(Vec::new()),
            };

            button(
                column![image(handle)
                    .width(Length::Fixed(140.0))
                    .height(Length::Fixed(140.0))]
                .align_x(Horizontal::Center),
            )
            .on_press(msg)
            .style(|_theme, _status| button::Style {
                background: None,
                border: Border::default(),
                ..Default::default()
            })
            .padding(4)
        };

        let buttons_row = row![
            make_btn("find.png", "Find", Message::Find),    //
            make_btn("load.png", "Load", Message::Load),    //
            make_btn("save.png", "Save", Message::Save),    //
            make_btn("solve.png", "Solve", Message::Solve), //
        ]
        .spacing(50) //
        .align_y(Vertical::Center);
        let input_field = text_input("Le résultat s'affichera après avoir cliqué sur Solve...", &self.user_text)
            .size(18)
            .padding(8)
            .style(|_theme, _status| text_input::Style {
                background: iced::Background::Color(Color::from_rgba(0.1, 0.1, 0.2, 0.85)),
                border: Border {
                    color: Color::from_rgb(0.2, 0.6, 1.0),
                    width: 1.5,
                    radius: 6.0.into(),
                },
                icon: Color::WHITE,
                placeholder: Color::from_rgb(0.5, 0.5, 0.6),
                value: Color::WHITE,
                selection: Color::from_rgb(0.2, 0.5, 0.9),
            });
        let status = text(&self.status_text)
            .size(Pixels(30.0))
            .font(Font::MONOSPACE)
            .color(Color::from_rgb(0.2, 0.8, 1.0));

        let content = container(
            column![
                Space::new().width(Length::Fill).height(Length::Fill),
                slider_row,
                Space::new().height(Length::Fixed(32.0)),
                buttons_row,
                Space::new().height(Length::Fixed(16.0)),
                input_field,
                Space::new().height(Length::Fixed(16.0)),
                status,
                Space::new().width(Length::Fill).height(Length::Fill),
            ]
            .spacing(0)
            .align_x(Horizontal::Center)
            .width(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding([24, 48]);

        stack![background, content].into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
#[derive(Default)]
struct BBoxOverlay {
    boxes: Vec<(usize, usize, usize, usize)>,
}

impl<Message> canvas::Program<Message> for BBoxOverlay {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        for &(xmin, ymin, xmax, ymax) in &self.boxes {
            let stroke_style = canvas::Stroke {
                style: canvas::stroke::Style::Solid(Color::from_rgb(1.0, 0.15, 0.15)),
                width: 2.5,
                ..Default::default()
            };
            let rect = iced::Rectangle {
                x: xmin as f32,
                y: ymin as f32,
                width: (xmax - xmin) as f32,
                height: (ymax - ymin) as f32,
            };
            let path = canvas::Path::rectangle(rect.position(), rect.size());
            frame.stroke(&path, stroke_style);
        }

        vec![frame.into_geometry()]
    }
}

fn transparent_btn(
    handle: image::Handle,
    size: f32,
    msg: Message,
) -> button::Button<'static, Message> {
    button(
        image(handle)
            .width(Length::Fixed(size))
            .height(Length::Fixed(size)),
    )
    .on_press(msg)
    .style(|_theme, _status| button::Style {
        background: None,
        border: Border::default(),
        ..Default::default()
    })
    .padding(4)
}

fn build_save_content(
    index: usize,
    liste: &[String],
    solve: &Option<(i32, i32)>,
    user_text: &str,
) -> String {
    let mut out = String::new();
    out.push_str(&format!("Image : {}\n", liste[index]));
    out.push_str(&format!("Index : {} / {}\n", index + 1, liste.len()));
    if let Some((a, b)) = solve {
        out.push_str(&format!(
            "Résultat Solve : valeur_a = {a}, valeur_b = {b}\n"
        ));
    } else {
        out.push_str("Résultat Solve : (aucun)\n");
    }
    if !user_text.is_empty() {
        out.push_str(&format!("Note utilisateur : {user_text}\n"));
    }
    out
}

fn worker_loop(rx: mpsc::Receiver<WorkerRequest>, tx: mpsc::Sender<WorkerResponse>) {
    while let Ok(req) = rx.recv() {
        let resp = match req {
            WorkerRequest::Find { image_path } => {
                eprintln!("[Worker] Find sur {image_path}");
                let boxes: Vec<(usize, usize, usize, usize)> =
                    vec![(10, 20, 100, 60), (120, 30, 200, 80)];
                WorkerResponse::FindDone(boxes)
            }
            WorkerRequest::Load => {
                eprintln!("[Worker] Lancement de l'explorateur de fichiers...");

                match load_img() { // Appel sans argument
                    Some((matrix, orig_path, final_path)) => {
                        WorkerResponse::LoadDone(matrix, orig_path, final_path)
                    }
                    None => WorkerResponse::Error(
                        "Sélection d'image annulée ou échec du traitement (PGM/Binarisation)".to_string()
                    ),
                }
            }
            WorkerRequest::Solve { image_path } => {
                eprintln!("[Worker] Solve {image_path}");
                let a: i32 = 42;
                let b: i32 = -7;
                WorkerResponse::SolveDone(a, b)
            }
        };

        if tx.send(resp).is_err() {
            break;
        }
    }
}

fn main() -> iced::Result {
    iced::application(
        BinaryViewer::default,
        BinaryViewer::update,
        BinaryViewer::view,
    )
    .title("BIN_OCR") // Le titre se configure ici !
    .theme(BinaryViewer::theme)
    .window(iced::window::Settings {
        size: iced::Size::new(1100.0, 600.0),
        resizable: true,
        ..Default::default()
    })
    .run()
}
