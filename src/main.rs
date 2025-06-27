mod shell;
mod agents;
mod substrate;
mod symbol;

fn main() {
    let mut shell = shell::Shell::new();
    shell.run();
}