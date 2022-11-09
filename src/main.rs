mod context;
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
use std::path::PathBuf;
use std::rc::Rc;
use std::{fs, panic};

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
    // todo: stop using Rc!!!
    let filename: Rc<str> = Rc::from(file.to_string_lossy().to_string());

    let src: Rc<str> = Rc::from(match fs::read_to_string(&file) {
        Ok(src) => src,
        Err(_) => {
            return Err(GlassError::FileNotFound {
                filename: Rc::clone(&filename),
            })
        }
    });

    debug!("Read {} bytes from '{}'", &src.len(), &file.display());

    let tokens: VecDeque<(Token, Span)> = Token::lexer(&src).spanned().collect();

    if log_enabled!(Level::Debug) {
        for (token, span) in &tokens {
            debug!("{:?} at {:?}", token, span);
        }
    }

    let mut parser = Parser::new(tokens, Rc::clone(&src), Rc::clone(&filename));
    let ast = parser.parse()?;

    debug!("AST > {:#?}", ast);

    let interpreter = Interpreter::new(Rc::clone(&src), Rc::clone(&filename));
    let result = interpreter.visit_node(ast)?;

    debug!("Result > {:?}", result);

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
