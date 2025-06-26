use std::io::{self, Write};
use crate::sptl::{Tokenizer, Parser, execute_program};

pub fn start_shell() {
    println!("🧬 SPT Shell initialized. Type ':exit' or 'exit' to quit.\n");

    loop {
        print!("σ̂> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("⚠️ Failed to read input.");
            continue;
        }

        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case(":exit") {
            println!("👋 Exiting shell. Farewell.");
            break;
        }

        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse();

        execute_program(program);
    }
}