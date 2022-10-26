mod error;
mod lexer;
mod node;
mod parser;

use crate::error::GlassError;
use crate::lexer::Token;
use crate::parser::Parser;
use clap::Parser as ClapParser;
use git_version::git_version;
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
            GlassError::UncaughtPanic {
                error_message: err.to_string(),
                glass_version: env!("CARGO_PKG_VERSION").into(),
                git_revision: git_version!(fallback = "<unknown>").into()
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
    setup_logger(args.debug);

    match args.file {
        Some(file) => {
            run_script(file)?;
        }
        None => {
            // todo - run REPL
        }
    }

    Ok(())
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

    let mut parser = Parser::new(tokens, source, file.display().to_string());
    let ast = parser.parse()?;
    println!("{:?}", ast);

    Ok(())
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
            // todo - hand off to delegate error handler
            panic!("{:?}", err);
        }
    }
}
