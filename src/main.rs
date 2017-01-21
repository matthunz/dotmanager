extern crate clap;

use clap::{App, Arg};
use std::env;
use std::path::PathBuf;

fn get_config() -> Option<PathBuf> {
    let xdg_path = env::var("XDG_CONFIG_HOME").ok()
        .map(|x| PathBuf::from(x).join("dotmanager").join("config.toml"));

    let dot_path = env::var("HOME").ok()
        .map(|x| PathBuf::from(x).join(".config").join("dotmanager").join("config.toml"));

    xdg_path.or(dot_path)
}

fn main() {
    let matches = App::new("Dotmanager")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .help("Sets a custom config file")
             .takes_value(true))
        .get_matches();

    let config = match matches.value_of("config") {
        Some(path) => PathBuf::from(path),
        None => get_config().expect("Config file not found")
    };

    println!("{:?}", config);
}
