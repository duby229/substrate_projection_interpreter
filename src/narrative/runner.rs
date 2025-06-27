//! Runner for SPTL narrative DSL with macros

use super::ast::{Block, Action};
use std::collections::HashMap;

#[derive(Default)]
pub struct ScriptContext {
    pub vars: HashMap<String, String>,
    pub macros: HashMap<String, (Vec<String>, Vec<Action>)>,
    pub agents: HashMap<String, AgentState>,
    pub tau: u64,
}

#[derive(Default, Debug, Clone)]
pub struct AgentState {
    pub memory: Vec<String>,
    pub activation: HashMap<String, f32>,
}

pub fn execute_script(blocks: &[Block], ctx: &mut ScriptContext) {
    // First pass: register macros
    for block in blocks {
        if let Block::MacroDef { name, params, body } = block {
            ctx.macros.insert(name.clone(), (params.clone(), body.clone()));
        }
    }
    // Second pass: execute non-macro blocks
    for block in blocks {
        match block {
            Block::MacroDef { .. } => {},
            _ => execute_block(block, ctx),
        }
    }
}

fn execute_block(block: &Block, ctx: &mut ScriptContext) {
    match block {
        Block::AtTau(tau, actions) => {
            ctx.tau = *tau;
            println!("--- at τ={} ---", tau);
            for action in actions {
                execute_action(action, ctx);
            }
        }
        Block::Repeat(n, actions) => {
            for i in 0..*n {
                println!("Repeat iteration {}/{}", i + 1, n);
                for action in actions {
                    execute_action(action, ctx);
                }
            }
        }
        Block::While(cond, actions) => {
            let mut count = 0;
            while eval_condition(cond, ctx) {
                println!("While iteration {}", count + 1);
                for action in actions {
                    execute_action(action, ctx);
                }
                count += 1;
                if count > 1000 {
                    println!("Breaking infinite while loop: more than 1000 iterations.");
                    break;
                }
            }
        }
        Block::Parallel(actions) => {
            println!("-- Parallel block --");
            for action in actions {
                execute_action(action, ctx);
            }
        }
        Block::MacroDef { .. } => {}
    }
}

fn execute_action(action: &Action, ctx: &mut ScriptContext) {
    match action {
        Action::Conditional(cond, subactions) => {
            if eval_condition(cond, ctx) {
                println!("Condition '{}' passed.", cond);
                for sub in subactions {
                    execute_action(sub, ctx);
                }
            } else {
                println!("Condition '{}' failed.", cond);
            }
        }
        Action::CreateAgent { name, mem, coh } => {
            println!("Create agent {} mem={} coh={}", name, mem, coh);
            ctx.agents.insert(name.clone(), AgentState::default());
        }
        Action::VariableAssignment { name, value } => {
            let val = expand_vars(value, ctx);
            println!("Set variable {} = {}", name, val);
            ctx.vars.insert(name.clone(), val);
        }
        Action::Say { agent, token, pattern } => {
            let token = expand_vars(token, ctx);
            let pattern = expand_vars(pattern, ctx);
            println!("{} says: {} → {}", agent, token, pattern);
            ctx.agents.entry(agent.clone()).or_default().memory.push(token.clone());
        }
        Action::Interpret { agent, token } => {
            let token = expand_vars(token, ctx);
            println!("{} interprets: {}", agent, token);
            ctx.agents.entry(agent.clone()).or_default().memory.push(token.clone());
        }
        Action::Project { agent, token } => {
            let token = expand_vars(token, ctx);
            println!("{} projects: {}", agent, token);
        }
        Action::Tick(n) => {
            println!("Advance τ by {}", n);
            ctx.tau += *n as u64;
        }
        Action::Assert(expr) => {
            println!("Assert: {}", expr);
        }
        Action::Comment(text) => {
            println!("# {}", text);
        }
        Action::MacroCall { name, args } => {
            if let Some((params, body)) = ctx.macros.get(name) {
                if params.len() != args.len() {
                    println!("Macro {} expects {} arguments, got {}", name, params.len(), args.len());
                    return;
                }
                let old_vars = ctx.vars.clone();
                for (p, a) in params.iter().zip(args.iter()) {
                    ctx.vars.insert(p.clone(), expand_vars(a, ctx));
                }
                for act in body {
                    execute_action(act, ctx);
                }
                ctx.vars = old_vars;
            } else {
                println!("Macro '{}' not found.", name);
            }
        }
    }
}

fn eval_condition(cond: &str, ctx: &ScriptContext) -> bool {
    if cond == "always" {
        return true;
    }
    let tokens: Vec<&str> = cond.split_whitespace().collect();
    if tokens.len() == 3 && tokens[1] == "knows" {
        if let Some(agent) = ctx.agents.get(tokens[0]) {
            return agent.memory.contains(&tokens[2].to_string());
        }
    }
    if tokens.len() == 3 && tokens[1] == "memory" && tokens[2].starts_with("contains") {
        let agent = tokens[0];
        let item = cond.split("contains").nth(1).unwrap().trim();
        if let Some(agent) = ctx.agents.get(agent) {
            return agent.memory.contains(&item.to_string());
        }
    }
    println!("Condition '{}' not recognized, default false.", cond);
    false
}

fn expand_vars(text: &str, ctx: &ScriptContext) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '$' {
            let mut name = String::new();
            while let Some(&n) = chars.peek() {
                if !n.is_alphanumeric() && n != '_' { break; }
                name.push(n);
                chars.next();
            }
            if let Some(val) = ctx.vars.get(&name) {
                result.push_str(val);
            } else {
                result.push('$');
                result.push_str(&name);
            }
        } else {
            result.push(c);
        }
    }
    result
}