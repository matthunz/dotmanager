extern crate clap;
extern crate tera;
extern crate toml;

use clap::{App, Arg};
use tera::Context;
use std::env;
use std::fs::File;
use std::io::Read;
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
        .arg(Arg::with_name("theme")
             .short("t")
             .long("theme")
             .help("Use a theme defined in the config")
             .takes_value(true))
        .get_matches();

    let config_file = match matches.value_of("config") {
        Some(path) => PathBuf::from(path),
        None => get_config().expect("Config file not found")
    };

    let mut buffer = String::new();
    File::open(&config_file).expect("Could not open config file")
        .read_to_string(&mut buffer).expect("Could not read config file");

    let config = toml::Parser::new(&buffer).parse().expect("Config is invalid toml");
    let mut context = Context::new();

    if let Some(section) = config.get("global").and_then(toml::Value::as_table) {
        for (key, val) in section {
            context.add(&key, &val.as_str().expect("Invalid string found in [global]"))
        }
    }

    if let Some(theme) = matches.value_of("theme") {
        if let Some(section) = config.get(theme).and_then(toml::Value::as_table) {
            for (key, val) in section {
                context.add(&key, &val.as_str().expect("Invalid string found in section"))
            }
        }
    }

    println!("{:?}", context);
}
