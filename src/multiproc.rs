//! Multiprocessing launcher for SPTL interpreter.

use std::process::Command;

/// Launch N subprocesses (copies of this interpreter) running different scripts or agent groups.
pub fn launch_simulations(n: usize, script_paths: &[&str]) {
    for i in 0..n {
        let script = script_paths.get(i % script_paths.len()).unwrap();
        let mut child = Command::new(std::env::current_exe().unwrap())
            .arg("--script")
            .arg(script)
            .spawn()
            .expect("failed to launch interpreter process");
        println!("Launched simulation process {} (PID={})", i, child.id());
    }
}