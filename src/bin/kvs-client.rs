#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use kvs::{Result, KvsClient};
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

    run(matches)
}

fn run(matches: ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("set", Some(matches)) => {
            let key = matches
                .value_of("KEY")
                .map(ToString::to_string)
                .expect("KEY argument is missing");
            let value = matches
                .value_of("VALUE")
                .map(ToString::to_string)
                .expect("VALUE argument is missing");
            let address = matches
                .value_of("IP-PORT")
                .expect("IP-PORT argument is missing");

            let mut client = KvsClient::connect(address)?;
            if let Err(err) = client.set(key, value) {
                eprintln!("{}", err);
                process::exit(1);
            }
        }
        ("get", Some(matches)) => {
            let key = matches
                .value_of("KEY")
                .map(ToString::to_string)
                .expect("KEY argument is missing");
            let address = matches
                .value_of("IP-PORT")
                .expect("IP-PORT argument is missing");

            let mut client = KvsClient::connect(address)?;
            match client.get(key) {
                Ok(Some(value)) => println!("{}", value),
                Ok(None) => println!("Key not found"),
                Err(err) => {
                    eprintln!("{}", err);
                    process::exit(1);
                }
            }
        }
        ("rm", Some(matches)) => {
            let key = matches
                .value_of("KEY")
                .map(ToString::to_string)
                .expect("KEY argument is missing");
            let address = matches
                .value_of("IP-PORT")
                .expect("IP-PORT argument is missing");

            let mut client = KvsClient::connect(address)?;
            if let Err(err) = client.remove(key) {
                eprintln!("{}", err);
                process::exit(1);
            }
        }
        _ => unreachable!(),
    };

    Ok(())
}
