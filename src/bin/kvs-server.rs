#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg};
use kvs::Result;
use std::process;

fn main() -> Result<()> {
    let matches = App::new("kvs-server")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("the server for the key value store")
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(
            Arg::with_name("version")
                .short("V")
                .long("version")
                .help("Print the version"),
        )
        .arg(
            Arg::with_name("IP-PORT")
                .short("a")
                .long("addr")
                .default_value("127.0.0.1:4000")
                .help("a v4 or v6 IP address with a port number"),
        )
        .arg(
            Arg::with_name("ENGINE-NAME")
                .short("e")
                .long("engine")
                .possible_values(&["kvs", "sled"])
                .default_value("kvs")
                .help("a v4 or v6 IP address with a port number"),
        )
        .get_matches();

    if matches.is_present("version") {
        println!(crate_version!());
        process::exit(0);
    }

    Ok(())
}
