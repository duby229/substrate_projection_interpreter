use std::collections::HashMap;
use crate::substrate::Substrate;
use crate::interpretation::Interpretation;
use crate::projection::project;
use crate::trace::{trace_distance, coherence};
use crate::visualize::print_vector;

#[derive(Debug)]
pub enum Statement {
    Field { name: String, size: usize },
    Interpretation { name: String, values: Vec<f64> },
    Project {
        target: String,
        interp: String,
        alpha: f64,
        noise: f64,
        steps: usize,
    },
    TraceDistance { name: String, field: String, interp: String },
    Meaning { name: String, trace_cmp: String, threshold: f64 },
    NarrateReturn { tokens: Vec<String> },
    LogCoherence(String),
    LogMeaning(String),
    ExpressSymbol { token: String, into_field: String },
    Modulate { token: String, intensity: f64 },
}

pub struct Tokenizer<'a> {
    input: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer { input }
    }

    pub fn tokenize(&mut self) -> Vec<String> {
        self.input
            .split_whitespace()
            .map(|s| s.trim_matches(&['"', ',', '[', ']'][..]).to_string())
            .collect()
    }
}pub struct Parser {
    tokens: Vec<String>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<String>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while self.cursor < self.tokens.len() {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                break;
            }
        }
        statements
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        let t = self.next()?.to_lowercase();
        match t.as_str() {
            "field" => {
                let name = self.next()?;
                let size = self.next()?.parse().ok()?;
                Some(Statement::Field { name, size })
            }
            "interpretation" => {
                let name = self.next()?;
                self.expect("=")?;
                self.expect("[")?;
                let mut values = Vec::new();
                while let Some(tok) = self.peek() {
                    if tok == "]" {
                        self.next();
                        break;
                    }
                    if let Ok(num) = tok.parse::<f64>() {
                        values.push(num);
                        self.next();
                    } else {
                        break;
                    }
                }
                Some(Statement::Interpretation { name, values })
            }
            "project" => {
                let target = self.next()?;
                self.expect("<-")?;
                let interp = self.next()?;
                self.expect("{")?;
                let alpha = self.expect_value("alpha:")?;
                let noise = self.expect_value("noise:")?;
                let steps = self.expect_value("steps:")? as usize;
                self.expect("}")?;
                Some(Statement::Project {
                    target,
                    interp,
                    alpha,
                    noise,
                    steps,
                })
            }
            "trace" => {
                let name = self.next()?;
                self.expect("=")?;
                let func = self.next()?;
                self.expect("(")?;
                let field = self.next()?;
                self.expect(",")?;
                let interp = self.next()?;
                self.expect(")")?;
                Some(Statement::TraceDistance {
                    name,
                    field,
                    interp,
                })
            }
            "meaning" => {
                let name = self.next()?;
                self.expect("=")?;
                let func = self.next()?;
                self.expect("(")?;
                let trace_cmp = self.next()?;
                self.expect(",")?;
                let threshold = self.next()?.parse().ok()?;
                self.expect(")")?;
                Some(Statement::Meaning {
                    name,
                    trace_cmp,
                    threshold,
                })
            }
            "narratereturn" => {
                let mut tokens = Vec::new();
                while let Some(tok) = self.peek() {
                    if tok.starts_with('"') {
                        tokens.push(tok.trim_matches('"').to_string());
                        self.next();
                    } else {
                        break;
                    }
                }
                Some(Statement::NarrateReturn { tokens })
            }
            "logcoherence" => {
                let field = self.next()?;
                Some(Statement::LogCoherence(field))
            }
            "logmeaning" => {
                let name = self.next()?;
                Some(Statement::LogMeaning(name))
            }
            "expresssymbol" => {
                let token = self.next()?;
                let _ = self.next()?; // into_field
                let field = self.next()?;
                Some(Statement::ExpressSymbol {
                    token,
                    into_field: field,
                })
            }
            "modulate" => {
                let token = self.next()?;
                let _ = self.next()?; // intensity
                let val = self.next()?.parse().ok()?;
                Some(Statement::Modulate { token, intensity: val })
            }
            _ => None,
        }
    }

    fn next(&mut self) -> Option<String> {
        if self.cursor < self.tokens.len() {
            let t = self.tokens[self.cursor].clone();
            self.cursor += 1;
            Some(t)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&str> {
        self.tokens.get(self.cursor).map(|s| s.as_str())
    }

    fn expect(&mut self, expected: &str) -> Option<()> {
        let token = self.next()?;
        if token.to_lowercase() == expected.to_lowercase() {
            Some(())
        } else {
            None
        }
    }

    fn expect_value(&mut self, label: &str) -> Option<f64> {
        let l = self.next()?;
        if !l.starts_with(label) {
            return None;
        }
        let val = self.next()?.parse().ok()?;
        Some(val)
    }
}
pub fn execute_program(program: Vec<Statement>) {
    let mut fields: HashMap<String, Substrate> = HashMap::new();
    let mut interps: HashMap<String, Interpretation> = HashMap::new();

    for stmt in program {
        match stmt {
            Statement::Field { name, size } => {
                fields.insert(name, Substrate::new(size));
            }
            Statement::Interpretation { name, values } => {
                interps.insert(name, Interpretation::new(values));
            }
            Statement::Project {
                target,
                interp,
                alpha,
                noise,
                steps,
            } => {
                if let (Some(field), Some(interp_val)) =
                    (fields.get_mut(&target), interps.get(&interp))
                {
                    for _ in 0..steps {
                        project(field, interp_val, alpha, noise);
                    }
                } else {
                    eprintln!("⚠️ Unknown field or interpretation in Project");
                }
            }
            Statement::TraceDistance {
                name,
                field,
                interp,
            } => {
                if let (Some(f), Some(i)) = (fields.get(&field), interps.get(&interp)) {
                    let result = trace_distance(f, i);
                    println!("Trace {} = {:.4}", name, result);
                } else {
                    eprintln!("⚠️ Unknown field or interpretation in TraceDistance");
                }
            }
            Statement::Meaning {
                name,
                trace_cmp,
                threshold,
            } => {
                println!("💡 Meaning {} ← {} < {}", name, trace_cmp, threshold);
            }
            Statement::NarrateReturn { tokens } => {
                println!("🗣 {}", tokens.join(" "));
            }
            Statement::LogCoherence(name) => {
                if let Some(f) = fields.get(&name) {
                    print_vector(&format!("Ψ[{}]", name), &f.state);
                } else {
                    eprintln!("⚠️ Unknown field in LogCoherence");
                }
            }
            Statement::LogMeaning(name) => {
                println!("🧠 Meaning declared: {}", name);
            }
            Statement::ExpressSymbol {
                token,
                into_field,
            } => {
                println!("➕ Expressed {} into {}", token, into_field);
            }
            Statement::Modulate { token, intensity } => {
                println!("🎛 Modulated {} @ {:.2}", token, intensity);
            }
        }
    }
}

