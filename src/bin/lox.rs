use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

use rustyline::{error::ReadlineError, Editor};
use structopt::StructOpt;

use lox::compiler::compile;

#[derive(StructOpt, Debug)]
#[structopt(name = "lox")]
struct CommandLineArgs {
    /// Lox source file
    file: Option<PathBuf>,
}

fn repl() {
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                compile(&line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn run_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let source = fs::read_to_string(path)?;

    compile(&source);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CommandLineArgs::from_args();

    if let Some(path) = args.file {
        run_file(path)?;
    } else {
        repl();
    }

    Ok(())
}
