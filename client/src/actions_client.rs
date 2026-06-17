/// Messages envoyés par le CLIENT vers le SERVER
#[derive(Debug, Clone)]
pub enum ClientMsg {
    LoadImage { filename: String, data: Vec<u8> },
    FindPositions { image_index: usize },

    SolveOcr { image_index: usize },
}
#[derive(Debug, Clone)]
pub enum ServerMsg {
    ImageLoaded { new_index: usize },

    Positions(Vec<(usize, usize, usize, usize)>),

    OcrResult(String),

    Error(String),
}
