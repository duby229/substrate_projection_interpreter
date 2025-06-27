//! AST for SPTL narrative DSL with macro support

#[derive(Debug, Clone)]
pub enum Block {
    AtTau(u64, Vec<Action>),
    Repeat(u32, Vec<Action>),
    While(String, Vec<Action>),
    Parallel(Vec<Action>),
    MacroDef { name: String, params: Vec<String>, body: Vec<Action> },
}

#[derive(Debug, Clone)]
pub enum Action {
    Conditional(String, Vec<Action>),
    CreateAgent { name: String, mem: u32, coh: f32 },
    MacroCall { name: String, args: Vec<String> },
    VariableAssignment { name: String, value: String },
    Say { agent: String, token: String, pattern: String },
    Interpret { agent: String, token: String },
    Project { agent: String, token: String },
    Tick(u32),
    Assert(String),
    Comment(String),
}