//! Parser for SPTL narrative DSL with macro support

use super::ast::{Block, Action};
use std::collections::VecDeque;

struct LineCursor<'a> {
    lines: VecDeque<(usize, &'a str)>,
}
impl<'a> LineCursor<'a> {
    fn from(script: &'a str) -> Self {
        let mut lines = VecDeque::new();
        for line in script.lines() {
            let trimmed = line.trim_start();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let indent = line.len() - trimmed.len();
            lines.push_back((indent, trimmed));
        }
        Self { lines }
    }
    fn peek(&self) -> Option<&(usize, &'a str)> {
        self.lines.front()
    }
    fn next(&mut self) -> Option<(usize, &'a str)> {
        self.lines.pop_front()
    }
}

pub fn parse_script(script: &str) -> Vec<Block> {
    let mut cursor = LineCursor::from(script);
    let mut blocks = Vec::new();
    while let Some((_, line)) = cursor.peek() {
        if line.starts_with("macro ") {
            blocks.push(parse_macro_def(&mut cursor));
        } else if line.starts_with("at τ=") {
            blocks.push(parse_at_tau(&mut cursor));
        } else if line.starts_with("repeat ") {
            blocks.push(parse_repeat(&mut cursor));
        } else if line.starts_with("while ") {
            blocks.push(parse_while(&mut cursor));
        } else if line.starts_with("parallel:") {
            blocks.push(parse_parallel(&mut cursor));
        } else {
            blocks.push(parse_at_tau(&mut cursor));
        }
    }
    blocks
}

fn parse_macro_def(cursor: &mut LineCursor) -> Block {
    let (base_indent, header) = cursor.next().unwrap();
    let header = header.trim_start_matches("macro").trim();
    let open_paren = header.find('(').unwrap();
    let close_paren = header.find(')').unwrap();
    let name = header[..open_paren].trim().to_string();
    let params: Vec<String> = header[open_paren + 1..close_paren]
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let mut body = Vec::new();
    while let Some((indent, _)) = cursor.peek() {
        if *indent <= base_indent {
            break;
        }
        body.append(&mut parse_action_block(cursor, base_indent + 2));
    }
    Block::MacroDef { name, params, body }
}

fn parse_at_tau(cursor: &mut LineCursor) -> Block {
    let (base_indent, header) = cursor.next().unwrap();
    let tau: u64 = header.trim_start_matches("at τ=").split(':').next().unwrap().trim().parse().unwrap();
    let mut actions = Vec::new();
    while let Some((indent, _)) = cursor.peek() {
        if *indent <= base_indent {
            break;
        }
        actions.append(&mut parse_action_block(cursor, base_indent + 2));
    }
    Block::AtTau(tau, actions)
}

fn parse_repeat(cursor: &mut LineCursor) -> Block {
    let (base_indent, header) = cursor.next().unwrap();
    let n: u32 = header.trim_start_matches("repeat")
        .split("times").next().unwrap().trim().parse().unwrap();
    let mut actions = Vec::new();
    while let Some((indent, _)) = cursor.peek() {
        if *indent <= base_indent {
            break;
        }
        actions.append(&mut parse_action_block(cursor, base_indent + 2));
    }
    Block::Repeat(n, actions)
}

fn parse_while(cursor: &mut LineCursor) -> Block {
    let (base_indent, header) = cursor.next().unwrap();
    let cond = header.trim_start_matches("while").trim_end_matches(':').trim().to_string();
    let mut actions = Vec::new();
    while let Some((indent, _)) = cursor.peek() {
        if *indent <= base_indent {
            break;
        }
        actions.append(&mut parse_action_block(cursor, base_indent + 2));
    }
    Block::While(cond, actions)
}

fn parse_parallel(cursor: &mut LineCursor) -> Block {
    let (base_indent, _) = cursor.next().unwrap();
    let mut actions = Vec::new();
    while let Some((indent, _)) = cursor.peek() {
        if *indent <= base_indent {
            break;
        }
        actions.append(&mut parse_action_block(cursor, base_indent + 2));
    }
    Block::Parallel(actions)
}

fn parse_action_block(cursor: &mut LineCursor, min_indent: usize) -> Vec<Action> {
    let (indent, line) = cursor.next().unwrap();
    if line.starts_with("if ") && line.ends_with(':') {
        let cond = line.trim_start_matches("if").trim_end_matches(':').trim().to_string();
        let mut subactions = Vec::new();
        while let Some((next_indent, _)) = cursor.peek() {
            if *next_indent <= indent {
                break;
            }
            subactions.append(&mut parse_action_block(cursor, indent + 2));
        }
        vec![Action::Conditional(cond, subactions)]
    } else {
        vec![parse_action(line)]
    }
}

fn parse_action(line: &str) -> Action {
    if let Some(rest) = line.strip_prefix("create agent ") {
        let mut parts = rest.split_whitespace();
        let name = parts.next().unwrap().to_string();
        let mem: u32 = parts.next().unwrap().parse().unwrap();
        let coh: f32 = parts.next().unwrap().parse().unwrap();
        Action::CreateAgent { name, mem, coh }
    } else if let Some(rest) = line.strip_prefix("let ") {
        let (name, value) = rest.split_once('=').unwrap();
        Action::VariableAssignment {
            name: name.trim().to_string(),
            value: value.trim().to_string(),
        }
    } else if let Some(rest) = line.strip_prefix("tick ") {
        let n = rest.trim().parse().unwrap();
        Action::Tick(n)
    } else if let Some(rest) = line.strip_prefix("assert ") {
        Action::Assert(rest.trim().to_string())
    } else if let Some((agent, rest)) = line.split_once(" says: ") {
        let (token, pattern) = rest.split_once(" → ").unwrap();
        Action::Say {
            agent: agent.trim().to_string(),
            token: token.trim().to_string(),
            pattern: pattern.trim().to_string(),
        }
    } else if let Some((agent, rest)) = line.split_once(" hears: ") {
        let (token, _) = rest.split_once(" → ").unwrap();
        Action::Interpret {
            agent: agent.trim().to_string(),
            token: token.trim().to_string(),
        }
    } else if let Some((agent, rest)) = line.split_once(" interprets: ") {
        Action::Interpret {
            agent: agent.trim().to_string(),
            token: rest.trim().to_string(),
        }
    } else if line.contains('(') && line.ends_with(')') {
        let open_paren = line.find('(').unwrap();
        let close_paren = line.find(')').unwrap();
        let name = line[..open_paren].trim().to_string();
        let argstr = &line[open_paren + 1..close_paren];
        let args: Vec<String> = argstr.split(',').map(|s| s.trim().to_string()).collect();
        Action::MacroCall { name, args }
    } else if line.starts_with('#') {
        Action::Comment(line[1..].trim().to_string())
    } else {
        panic!("Unrecognized action: {}", line);
    }
}