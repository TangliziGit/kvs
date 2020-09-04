#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};
use std::process;

fn main() {
    let matches = App::new("Key Value Store - kvs")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(
            Arg::with_name("version")
                .short("V")
                .long("version")
                .help("Print the version"),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").required(true).help("a string key"))
                .arg(
                    Arg::with_name("VALUE")
                        .required(true)
                        .help("a string value"),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").required(true).help("a string key")),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").required(true).help("a string key")),
        )
        .get_matches();

    if matches.is_present("version") {
        println!(crate_version!());
    }

    match matches.subcommand() {
        ("set", Some(matches)) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        ("get", Some(matches)) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        ("rm", Some(matches)) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        _ => process::exit(1),
    }
}
