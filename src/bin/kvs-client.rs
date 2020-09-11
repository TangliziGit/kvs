#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
use kvs::KvStore;
use kvs::{Error, Result};
use std::env::current_dir;
use std::process;

fn main() -> Result<()> {
    let matches = App::new("kvs-client")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("a client for the key value store")
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
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
                )
                .arg(
                    Arg::with_name("IP-PORT")
                        .short("a")
                        .long("addr")
                        .default_value("127.0.0.1:4000")
                        .help("a v4 or v6 IP address with a port number"),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").required(true).help("a string key"))
                .arg(
                    Arg::with_name("IP-PORT")
                        .short("a")
                        .long("addr")
                        .default_value("127.0.0.1:4000")
                        .help("a v4 or v6 IP address with a port number"),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").required(true).help("a string key"))
                .arg(
                    Arg::with_name("IP-PORT")
                        .short("a")
                        .long("addr")
                        .default_value("127.0.0.1:4000")
                        .help("a v4 or v6 IP address with a port number"),
                ),
        )
        .get_matches();

    if matches.is_present("version") {
        println!(crate_version!());
        process::exit(0);
    }

    match matches.subcommand() {
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument is missing");
            let value = matches
                .value_of("VALUE")
                .expect("VALUE argument is missing");

            let mut store = KvStore::open(current_dir()?)?;
            store.set(key.to_string(), value.to_string())?;
        }
        ("get", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument is missing");

            let mut store = KvStore::open(current_dir()?)?;
            let value = store.get(key.to_string())?;

            if let Some(value) = value {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument is missing");

            let mut store = KvStore::open(current_dir()?)?;
            match store.remove(key.to_string()) {
                Ok(_) => {}
                Err(Error::KeyNotFound) => {
                    println!("Key not found");
                    process::exit(1);
                }
                Err(e) => return Err(e),
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}