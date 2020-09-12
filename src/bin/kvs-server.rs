use clap::*;
use slog::*;

use kvs::{KvStore, Protocol, Result};
use slog::Logger;
use std::env::current_dir;
use std::net::TcpListener;
use std::process;

fn main() -> Result<()> {
    let logger = get_logger();
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

    info!(logger, "kvs initializing";
        "version" => crate_version!(),
         "ip" => matches.value_of("IP-PORT").unwrap()
    );

    let addr = matches
        .value_of("IP-PORT")
        .expect("IP-PORT argument is missing.");
    let listener = TcpListener::bind(addr)?;

    let store = KvStore::open(current_dir()?)?;

    serve(logger, listener, store)
}

fn get_logger() -> Logger {
    let drain = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(drain).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, o!())
}

fn serve(logger: Logger, listener: TcpListener, mut _store: KvStore) -> Result<()> {
    for stream in listener.incoming() {
        let stream = stream?;
        let request: Protocol = serde_json::from_reader(&stream)?;

        info!(logger, "incoming request"; "request" => format!("{:?}", request));
        match request {
            Protocol::Set {
                key: _key,
                value: _value,
            } => {
                error!(logger, "unimplemented");
            }
            Protocol::Get { key: _key } => {
                error!(logger, "unimplemented");
            }
            Protocol::Remove { key: _key } => {
                error!(logger, "unimplemented");
            }
        }
    }

    Ok(())
}
