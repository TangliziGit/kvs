#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
use kvs::KvStore;
use kvs::Result;
use std::env::current_dir;
use std::process;

fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
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

    match matches.subcommand() {
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument missing");
            let value = matches.value_of("VALUE").expect("VALUE argument missing");

            let mut store = KvStore::open(current_dir()?)?;
            store.set(key.to_string(), value.to_string())?;
        }
        ("get", Some(_matches)) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        ("rm", Some(_matches)) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        _ => unreachable!(),
    }

    Ok(())
}
