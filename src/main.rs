#![feature(with_options)]

use structopt::StructOpt;
use failure::Error;

use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::process;

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
        key: String,
        value: String,
    },
    Get {
        key: String,
    },
}

fn config_file_path(args: &Args) -> Result<PathBuf, Error> {
    match args.config_file {
        Some(ref path) => {
            Ok(PathBuf::from(path))
        },
        None => {
            let xdg_dirs = xdg::BaseDirectories::with_prefix("shelf")?;

            Ok(xdg_dirs.place_config_file("shelf.yml")?)
        },
    }
}

fn read_config(args: &Args) -> Result<HashMap<String, String>, Error> {
    let config_path = config_file_path(args)?;

    match File::open(config_path) {
        Ok(file) => Ok(serde_yaml::from_reader(file)?),
        Err(_) => Ok(HashMap::new()),
    }
}

fn write_config(args: &Args, data: &HashMap<String, String>) -> Result<(), Error> {
    let config_path = config_file_path(args)?;

    let file = File::create(config_path)?;

    serde_yaml::to_writer(file, data)?;

    Ok(())
}

fn actual_main() -> Result<(), Error> {
    let args = Args::from_args();

    match args.cmd {
        Command::Add { ref key, ref value } => {
            let mut data = read_config(&args)?;

            data.insert(key.to_owned(), value.to_owned());

            write_config(&args, &data)?;

        },
        Command::Get { ref key } => {
            let data = read_config(&args)?;

            match data.get(key) {
                Some(value) => println!("{}", value),
                None => {},
            }
        }
    }

    Ok(())
}

fn main() {
    match actual_main() {
        Ok(()) => {},
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        },
    }
}
