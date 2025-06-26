#[derive(Clone)]
pub struct Interpretation {
    pub data: Vec<f64>,
}

impl Interpretation {
    pub fn new(data: Vec<f64>) -> Self {
        Interpretation { data }
    }
}