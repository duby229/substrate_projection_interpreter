pub fn print_vector(name: &str, vec: &[f64]) {
    let body = vec.iter().map(|v| format!("{:.2}", v)).collect::<Vec<_>>().join(", ");
    println!("{} = [{}]", name, body);
}