mod error;
mod lexer;
mod parser;

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
}

fn main() {
    panic::set_hook(Box::new(|e| {
        // todo - prettify
        eprintln!("{}", e);
        std::process::exit(1);
    }));

    let args = Args::parse();
    setup_logger(args.debug);

    match args.file {
        Some(file) => {
            run_script(file);
        }
        None => {
            // todo - run REPL
        }
    }
}

fn run_script(file: PathBuf) {
    let source = std::fs::read_to_string(&file).unwrap();
    debug!("Read {} bytes from '{}'", &source.len(), &file.display());

    let tokens: VecDeque<(Token, Span)> = Token::lexer(&source).spanned().collect();

    if log_enabled!(Level::Debug) {
        for (token, span) in &tokens {
            debug!("{:?} at {:?}", token, span);
        }
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
}

fn setup_logger(debug: bool) {
    let level = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    match SimpleLogger::init(level, simplelog::Config::default()) {
        Ok(_) => {}
        Err(err) => {
            // todo - hand off to delegated error handler
            panic!("{:?}", err);
        }
    }
}
