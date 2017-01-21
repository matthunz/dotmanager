extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Dotmanager")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .help("Sets a custom config file")
             .takes_value(true))
        .get_matches();

    println!("{}", matches.value_of("config").unwrap_or("none"));
}
