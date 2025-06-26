use crate::substrate::Substrate;
use crate::interpretation::Interpretation;

pub fn trace_distance(a: &Substrate, b: &Interpretation) -> f64 {
    a.state.iter()
        .zip(&b.data)
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn coherence(a: &[f64], b: &[f64]) -> f64 {
    let dot = a.iter().zip(b).map(|(x, y)| x * y).sum::<f64>();
    let mag_a = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let mag_b = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    if mag_a == 0.0 || mag_b == 0.0 {
        0.0
    } else {
        dot / (mag_a * mag_b)
    }
}