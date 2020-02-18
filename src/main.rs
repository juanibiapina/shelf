#![feature(with_options)]

use structopt::StructOpt;
use failure::{Error, ResultExt};

use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(short, long, env = "SHELF_CONFIG")]
    config_file: Option<String>,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Add {
        entries: Vec<String>,
    },
    List,
}

fn config_file_path(args: &Args) -> Result<PathBuf, Error> {
    match args.config_file {
        Some(ref path) => {
            Ok(fs::canonicalize(path)?)
        },
        None => {
            let xdg_dirs = xdg::BaseDirectories::with_prefix("shelf")?;

            Ok(xdg_dirs.place_config_file("shelf.txt")?)
        },
    }
}

fn actual_main() -> Result<(), Error> {
    let args = Args::from_args();

    match args.cmd {
        Command::Add { ref entries } => {
            let config_path = config_file_path(&args)?;
            let mut file = File::with_options().create(true).append(true).open(config_path).context("Couldn't create config file")?;
            writeln!(&mut file, "{}", entries.join(","))?;

        },
        Command::List => {
            let config_path = config_file_path(&args)?;
            let mut file = File::open(config_path).context("Couldn't read config file")?;

            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;

            print!("{}", buffer);
        }
    }

    Ok(())
}

fn main() {
    match actual_main() {
        Ok(()) => {},
        Err(err) => println!("{}", err),
    }
}
