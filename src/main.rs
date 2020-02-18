#![feature(with_options)]

use failure::Error;
use serde::{Serialize, Deserialize};
use structopt::StructOpt;

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

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Store {
    Value(String),
    Map(HashMap<String, Store>),
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

fn read_config(args: &Args) -> Result<Store, Error> {
    let config_path = config_file_path(args)?;

    match File::open(config_path) {
        Ok(file) => Ok(serde_yaml::from_reader(file)?),
        Err(_) => Ok(Store::Map(HashMap::new())),
    }
}

fn write_config(args: &Args, data: &Store) -> Result<(), Error> {
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

            match data {
                Store::Value(_) => panic!("Invalid configuration file"),
                Store::Map(ref mut data) => { data.insert(key.to_owned(), Store::Value(value.to_owned())) },
            };

            write_config(&args, &data)?;

        },
        Command::Get { ref key } => {
            let data = read_config(&args)?;

            match data {
                Store::Value(_) => panic!("Invalid configuration file"),
                Store::Map(data) => {
                    match data.get(key) {
                        Some(value) => {
                            match value {
                                Store::Value(value) => println!("{}", value),
                                Store::Map(_) => panic!("Invalid configuration file"),
                            }
                        },
                        None => {},
                    }
                },
            };
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
