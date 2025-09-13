use colored::*;

use std::{env, fs::File, io::Read, process};

use ice::{lexer::Lexer, parser};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("{}", "failed to compile 'ice' program".red().bold());
        eprintln!("{}", "usage:".bright_blue());
        eprintln!("\t{} {}", args[0].green(), "<your-file.ic>".blue().bold());
        process::exit(1);
    }

    let mut file = File::open(args[1].clone()).expect(
        format!(
            "{}: {}",
            "failed to open file".red().bold(),
            args[1].green(),
        )
        .as_str(),
    );

    let mut src = String::new();
    file.read_to_string(&mut src)
        .expect(format!("{}", "failed to read file contents".red(),).as_str());

    let mut lexer = Lexer::new(src.into());
    match lexer.lex() {
        Ok(tokens) => {
            let mut parser = parser::Parser::new(tokens);
            let tree = parser.parse().unwrap();
            for stmt in tree {
                println!("stmt: {}", stmt);
            }
        }
        Err(err) => {
            eprintln!("{}: {}", "lexical error".red().bold(), err.bright_red());
            process::exit(1);
        }
    }
}
