mod error;
mod interpreter;
mod lexer;
mod node;
mod parser;
mod value;

use crate::error::GlassError;
use crate::interpreter::Interpreter;
use crate::lexer::Token;
use crate::parser::Parser;
use clap::Parser as ClapParser;
use log::{debug, log_enabled, Level, LevelFilter};
use logos::{Logos, Span};
use simplelog::SimpleLogger;
use std::collections::VecDeque;
use std::panic;
use std::path::PathBuf;

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(help = "The script file to run", index = 1)]
    file: Option<PathBuf>,

    #[clap(short = 'd', long = "debug", help = "Enable debug mode")]
    debug: bool,

    #[clap(short = 'v', long = "verbose", help = "Enable verbose mode")]
    verbose: bool,
}

fn main() {
    panic::set_hook(Box::new(|err| {
        eprintln!(
            "{}",
            GlassError::UnknownError {
                error_message: err.to_string(),
            }
        );
    }));

    if let Err(err) = try_main() {
        eprintln!("Fatal exception during execution -> {}", err);
        /*

            Fatal exception during runtime -> "Unknown character '$' encountered at

                el oh el $ lol
                         ^
            [test.glass(Ln:6 Col:10)]"

        */
    }
}

fn try_main() -> Result<(), GlassError> {
    let args = Args::parse();
    setup_logger(args.debug)?;

    match args.file {
        Some(file) => run_script(file),
        None => Err(GlassError::PlaceholderError {
            message: "REPL not implemented yet".into(),
        }),
    }
}

fn run_script(file: PathBuf) -> Result<(), GlassError> {
    let source = std::fs::read_to_string(&file).unwrap();
    debug!("Read {} bytes from '{}'", &source.len(), &file.display());

    let tokens: VecDeque<(Token, Span)> = Token::lexer(&source).spanned().collect();

    if log_enabled!(Level::Debug) {
        for (token, span) in &tokens {
            debug!("{:?} at {:?}", token, span);
        }
    }

    let mut parser = Parser::new(tokens, source.into(), file.display().to_string().into());
    let ast = parser.parse()?;

    if log_enabled!(Level::Debug) {
        debug!("AST > {:#?}", ast);
    }

    let interpreter = Interpreter::new();
    let result = interpreter.visit_node(ast)?;

    if log_enabled!(Level::Debug) {
        debug!("Result > {:?}", result);
    }

    Ok(())
}

fn setup_logger(debug: bool) -> Result<(), GlassError> {
    let level = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    match SimpleLogger::init(level, simplelog::Config::default()) {
        Ok(_) => Ok(()),
        Err(err) => Err(GlassError::UnknownError {
            error_message: err.to_string(),
        }),
    }
}
