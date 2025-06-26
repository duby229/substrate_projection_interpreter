use crate::substrate::Substrate;
use crate::interpretation::Interpretation;
use rand::Rng;

pub fn project(
    substrate: &mut Substrate,
    interpretation: &Interpretation,
    alpha: f64,
    noise: f64,
) {
    let mut rng = rand::thread_rng();
    for (s, i) in substrate.state.iter_mut().zip(&interpretation.data) {
        let n = rng.gen_range(-noise..=noise);
        *s = (1.0 - alpha) * *s + alpha * (*i + n);
    }
}