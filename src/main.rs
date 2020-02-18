#![feature(with_options)]

use structopt::StructOpt;
use failure::{Error, ResultExt};

use std::fs::File;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
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
        message: Vec<String>,
    },
    List {
        #[structopt(short)]
        tag_filter: Option<String>
    }
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
        Command::Add { ref message } => {
            let config_path = config_file_path(&args)?;
            let mut file = File::with_options().create(true).append(true).open(config_path).context("Couldn't create config file")?;
            writeln!(&mut file, "{}", message.join(" "))?;

        },
        Command::List { ref tag_filter } => {
            let config_path = config_file_path(&args)?;
            let file = BufReader::new(File::open(config_path).context("Couldn't read config file")?);

            for line in file.lines() {
                let line = line?;

                match tag_filter {
                    Some(tag) => {
                        if line.contains(&format!("#{}", tag)) {
                            println!("{}", line)
                        }
                    },
                    None => { println!("{}", line) },
                }
            }
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
