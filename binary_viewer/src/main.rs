use crate::{actions::*, load_image::*};
use iced::futures::SinkExt;
use iced::widget::canvas;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, image, row, stack, text, text_input, Space},
    Border, Color, Element, Font, Length, Pixels, Task, Theme,
};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use BIN_OCR::*;

const BINARY_ITEMS: &[&str] = &[
    "src/images/BINimg1.png",
    "src/images/BINimg2.png",
    "src/images/BINimg3.png",
];

#[derive(Debug, Clone)]
enum Message {
    Previous,
    Next,
    Find,
    Load,
    Save,
    Solve,
    WorkerReply(WorkerResponse),
}
#[derive(Debug, Clone)]
enum WorkerResponse {
    FindDone(Vec<(usize, usize, usize, usize)>),
    LoadDone(Vec<Vec<u32>>, String, String),
    SolveDone(i32, i32),
    Error(String),
}

#[derive(Debug)]
enum WorkerRequest {
    Find {
        image_path: String,
    },
    Load,
    Solve {
        image_path: String,
    },
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
    image_size: (u32, u32),
    tx: Sender<WorkerRequest>,
    rx: Arc<Mutex<Receiver<WorkerResponse>>>,
}

impl Default for BinaryViewer {
    fn default() -> Self {
        let (dummy_tx, _dummy_rx) = mpsc::channel();
        let (_dummy_tx_resp, dummy_rx_resp) = mpsc::channel();
        Self {
            current_index: 0,
            liste_images: BINARY_ITEMS.iter().map(|s| s.to_string()).collect(),
            bounding_boxes: Vec::new(),
            loaded_image_path: None,
            solve_result: None,
            status_text: String::new(),
            user_text: String::new(),
            image_size: (0, 0),
            tx: dummy_tx,
            rx: Arc::new(Mutex::new(dummy_rx_resp)),
        }
    }
}
impl BinaryViewer {
    pub fn new() -> (Self, Task<Message>) {
        let (tx_req, rx_req) = mpsc::channel();
        let (tx_resp, rx_resp) = mpsc::channel();

        std::thread::spawn(move || worker_loop(rx_req, tx_resp));

        let app = Self {
            current_index: 0,
            liste_images: BINARY_ITEMS.iter().map(|s| s.to_string()).collect(),
            bounding_boxes: Vec::new(),
            loaded_image_path: None,
            solve_result: None,
            status_text: String::new(),
            user_text: String::new(),
            image_size: (0, 0),
            tx: tx_req,
            rx: Arc::new(Mutex::new(rx_resp)),
        };

        let listen_task = listen_worker(&app.rx);

        (app, listen_task)
    }
    fn send_to_worker(&self, req: WorkerRequest) -> Task<Message> {
        let _ = self.tx.send(req);
        Task::none()
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
                eprintln!("[UI] Clic sur Find pour l'image: {}", path);
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
                eprintln!("[UI] Clic sur Solve pour l'image: {}", path);
                self.send_to_worker(WorkerRequest::Solve { image_path: path })
            }

            Message::WorkerReply(resp) => {
                match resp {
                    WorkerResponse::FindDone(boxes) => {
                        self.status_text = format!("Find : {} rectangle(s) détecté(s)", boxes.len());
                        self.bounding_boxes = boxes;
                        let img_path = self.loaded_image_path.clone()
                            .unwrap_or_else(|| self.liste_images[self.current_index].clone());
                        if let Ok(img) = ::image::open(&img_path) {
                            self.image_size = (img.width(), img.height()); 
                        }
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
                        eprintln!("[UI] SolveDone reçu avec a={}, b={}", a, b);
                        self.solve_result = Some((a, b));
                        self.status_text = format!("Résultat Solve : valeur_a = {a}, valeur_b = {b}");
                        self.user_text = format!(
                            "la valeur décimal du mot binaire dans le cas d'un entier signé est {} et dans le cas contraire {}",
                            a, b
                        );
                    }
                    WorkerResponse::Error(e) => {
                        self.status_text = format!("Erreur : {e}");
                    }
                }
                eprintln!("[UI] Relancement de listen_worker...");
                return listen_worker(&self.rx);
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
        let img_w = if self.image_size.0 > 0 { self.image_size.0 as f32 } else { 1.0 };
        let img_h = if self.image_size.1 > 0 { self.image_size.1 as f32 } else { 1.0 };

        let overlay = BBoxOverlay {
            boxes: self.bounding_boxes.clone(),
            img_w,
            img_h,
        };
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
                    stack![
                        img_widget, 
                        canvas(overlay)
                            .width(Length::Fill)
                            .height(Length::Fixed(200.0))
                    ].into()
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
        let input_field = container(
            text(if self.user_text.is_empty() {
                "Le résultat s'affichera après avoir cliqué sur Solve..."
            } else {
                &self.user_text
            })
            .size(18)
            .color(if self.user_text.is_empty() {
                Color::from_rgb(0.5, 0.5, 0.6)
            } else {
                Color::WHITE
            }),
        )
        .padding(8)
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(0.1, 0.1, 0.2, 0.85))),
            border: Border {
                color: Color::from_rgb(0.2, 0.6, 1.0),
                width: 1.5,
                radius: 6.0.into(),
            },
            ..Default::default()
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
    img_w: f32, 
    img_h: f32, 
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
 
        let view_w = bounds.width;
        let view_h = bounds.height;
 
        let img_w = if self.img_w > 0.0 { self.img_w } else { 1.0 };
        let img_h = if self.img_h > 0.0 { self.img_h } else { 1.0 };
 
        let scale = (view_w / img_w).min(view_h / img_h);
 
        let rendered_w = img_w * scale;
        let rendered_h = img_h * scale;
 
        let offset_x = (view_w - rendered_w) / 2.0;
        let offset_y = (view_h - rendered_h) / 2.0;
 
        for &(xmin, ymin, xmax, ymax) in &self.boxes {
            let stroke_style = canvas::Stroke {
                style: canvas::stroke::Style::Solid(Color::from_rgb(1.0, 0.15, 0.15)),
                width: 2.0,
                ..Default::default()
            };
 
            let x = offset_x + xmin as f32 * scale;
            let y = offset_y + ymin as f32 * scale;
            let w = (xmax - xmin) as f32 * scale;
            let h = (ymax - ymin) as f32 * scale;
 
            let rect = iced::Rectangle { x, y, width: w, height: h };
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

fn worker_loop(rx: Receiver<WorkerRequest>, tx: Sender<WorkerResponse>) {
    while let Ok(request) = rx.recv() {
        let response = match request {
            WorkerRequest::Find { image_path } => {
                eprintln!("[Worker] Find sur {image_path}");
                match extract_position(&image_path) {
                    Some(boxes) => WorkerResponse::FindDone(boxes),
                    None => WorkerResponse::FindDone(vec![]),
                }
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
                match run_ocr(&image_path) {
                    Some(result) => WorkerResponse::SolveDone(result.0, result.1),
                    None => WorkerResponse::SolveDone(0, 0),
                }
            }
        };
        if tx.send(response).is_err() {
            break;
        }
    }
}
fn poll_worker(rx: &std::sync::Mutex<Receiver<WorkerResponse>>) -> Option<Message> {
    rx.lock().ok()?.try_recv().ok().map(Message::WorkerReply)
}
fn listen_worker(rx: &Arc<Mutex<Receiver<WorkerResponse>>>) -> Task<Message> {
    let rx_clone = Arc::clone(rx);

    Task::run(
        iced::stream::channel(100, move |mut iced_tx:iced::futures::channel::mpsc::Sender<Message>| async move {
            use iced::futures::SinkExt;
            use iced::futures::StreamExt; 
            let (sync_tx, mut iced_rx) = iced::futures::channel::mpsc::channel::<Message>(100);

            std::thread::spawn(move || {
                if let Ok(lock) = rx_clone.lock() {
                    while let Ok(response) = lock.recv() {
                        if sync_tx.clone().try_send(Message::WorkerReply(response)).is_err() {
                            break;
                        }
                    }
                }
            });
            while let Some(msg) = iced_rx.next().await {
                let _ = iced_tx.send(msg).await;
            }
        }),
        |msg| msg,
    )
}
fn main() -> iced::Result {
    iced::application(
        || BinaryViewer::new(),
        BinaryViewer::update,
        BinaryViewer::view,
    )
    .title("BIN_OCR") 
    .theme(BinaryViewer::theme)
    .window(iced::window::Settings {
        size: iced::Size::new(1100.0, 600.0),
        resizable: true,
        ..Default::default()
    })
    .run()
}
