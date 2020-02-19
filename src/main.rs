#![feature(with_options)]

#[macro_use] extern crate failure;

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
        #[structopt(min_values(3), required(true))]
        values: Vec<String>,
    },
    Get {
        #[structopt(min_values(1), required(true))]
        keys: Vec<String>,
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
        Command::Add { ref values } => {
            let mut data = read_config(&args)?;

            let mut current = &mut data;

            let (keys, values) = values.split_at(values.len() - 2);

            for key in keys {
                match current {
                    Store::Value(_) => {
                        return Err(format_err!("Invalid path"))
                    },
                    Store::Map(ref mut data) => {
                        if !data.contains_key(key) {
                            data.insert(key.to_owned(), Store::Map(HashMap::new()));
                        }

                        current = data.get_mut(key).unwrap()
                    },
                }
            }

            match current {
                Store::Value(_) => {
                    return Err(format_err!("Invalid path"))
                },
                Store::Map(ref mut data) => {
                    data.insert(values.get(0).unwrap().to_owned(), Store::Value(values.get(1).unwrap().to_owned()));
                },
            }

            write_config(&args, &data)?;
        },
        Command::Get { ref keys } => {
            let data = read_config(&args)?;

            let mut result = &data;

            for key in keys {
                result = match result {
                    Store::Value(_) => return Err(format_err!("Invalid path")),
                    Store::Map(data) => {
                        match data.get(key) {
                            Some(v) => v,
                            None => return Err(format_err!("Entry not found")),
                        }
                    },
                }
            }

            match result {
                Store::Value(v) => println!("{}", v),
                Store::Map(_) => unimplemented!(),
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
