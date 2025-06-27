mod narrative;

fn handle_run_script(&mut self, script: &str) {
    use narrative::{parser, runner};
    let blocks = parser::parse_script(script);
    let mut ctx = runner::ScriptContext::default();
    runner::execute_script(&blocks, &mut ctx);
}