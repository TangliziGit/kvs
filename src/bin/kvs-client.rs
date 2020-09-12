#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use kvs::{Protocol, Result};
use std::io::Write;
use std::net::TcpStream;
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
    let mut stream = TcpStream::connect("localhost:4000")?;

    let request = match matches.subcommand() {
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument is missing");
            let value = matches
                .value_of("VALUE")
                .expect("VALUE argument is missing");

            Protocol::Set {
                key: key.to_string(),
                value: value.to_string(),
            }
        }
        ("get", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument is missing");

            Protocol::Get {
                key: key.to_string(),
            }
        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument is missing");

            Protocol::Remove {
                key: key.to_string(),
            }
        }
        _ => unreachable!(),
    };

    let request = serde_json::to_vec(&request)?;
    stream.write_all(request.as_slice())?;

    Ok(())
}
