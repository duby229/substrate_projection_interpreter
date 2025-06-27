mod shell;
mod agents;
mod substrate;
mod symbol;
mod symmetry;
mod multiproc;

use std::sync::{Arc, Mutex};
use agents::Agent;

fn create_agents() -> Vec<Arc<Mutex<Agent>>> {
    (0..8)
        .map(|i| Arc::new(Mutex::new(Agent::new(format!("agent{}", i), 128, 0.2))))
        .collect()
}

fn load_scripts() -> Vec<String> {
    // Stub: implement to load scripts from files or config
    vec!["slm.sptl".to_string()]
}

fn main() {
    // Multiprocessing: launch N separate interpreters
    let num_procs = 2;
    let scripts = vec!["slm.sptl"];
    multiproc::launch_simulations(num_procs, &scripts);

    // Multithreading: run all agents in parallel
    let mut agents = create_agents();
    agents.par_iter().for_each(|agent| {
        let mut agent = agent.lock().unwrap();
        agent.tick_parallel();
    });

    // Run scripts in parallel
    let shell = shell::Shell::new();
    let scripts = load_scripts();
    shell.run_scripts_in_parallel(scripts);
}