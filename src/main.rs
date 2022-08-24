mod lexer;

use crate::lexer::Token;
use clap::Parser;
use log::debug;
use logos::Logos;
use simplelog::SimpleLogger;
use std::panic;
use std::path::PathBuf;

#[derive(Parser, Debug)]
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

    let mut lexer: logos::Lexer<Token> = Token::lexer(&source);

    while let Some(token) = lexer.next() {
        debug!("{:?}", token);
    }
}

fn setup_logger(debug: bool) {
    SimpleLogger::init(
        if debug {
            simplelog::LevelFilter::Debug
        } else {
            simplelog::LevelFilter::Info
        },
        simplelog::Config::default(),
    )
    .unwrap();
}
