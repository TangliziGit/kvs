use clap::{App, Arg, SubCommand};
use std::process;

fn main() {
    let matches = App::new("Key Value Store")
        .version("0.0.1")
        .author("TangliziGit <tanglizimail@foxmail.com>")
        .about("A key-value store")
        .arg(Arg::with_name("version")
             .short("V")
             .long("version")
             .help("Print the version"))
        .subcommand(SubCommand::with_name("set")
            .about("Set the value of a string key to a string")
            .arg(Arg::with_name("KEY")
                .required(true)
                .help("a string key"))
            .arg(Arg::with_name("VALUE")
                .required(true)
                .help("a string value")))
        .subcommand(SubCommand::with_name("get")
            .about("Get the string value of a given string key")
            .arg(Arg::with_name("KEY")
                .required(true)
                .help("a string key")))
        .subcommand(SubCommand::with_name("rm")
            .about("Remove a given key")
            .arg(Arg::with_name("KEY")
                .required(true)
                .help("a string key")))
        .get_matches();

    if matches.is_present("version") {
        println!("0.0.1");
    }

    match matches.subcommand() {
        ("set", Some(matches)) => {
            eprintln!("unimplemented");
            process::exit(1);
        },
        ("get", Some(matches)) => {
            eprintln!("unimplemented");
            process::exit(1);
        },
        ("rm",  Some(matches)) => {
            eprintln!("unimplemented");
            process::exit(1);
        },
        _ => {},
    }
}
