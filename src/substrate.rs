#[derive(Clone)]
pub struct Substrate {
    pub state: Vec<f64>,
}

impl Substrate {
    pub fn new(size: usize) -> Self {
        Substrate {
            state: vec![0.0; size],
        }
    }
}