use clap::*;
use slog::*;

use kvs::{KvStore, Request, Response, Result};
use slog::Logger;
use std::env::current_dir;
use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
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
                .possible_values(&["kvs"])
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

    let mut store = KvStore::open(current_dir()?)?;

    for stream in listener.incoming() {
        serve(&logger, stream?, &mut store)?
    }

    Ok(())
}

fn get_logger() -> Logger {
    let drain = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(drain).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, o!())
}

// TODO: design and implement a kvs server in lib crate
fn serve(logger: &Logger, mut stream: TcpStream, store: &mut KvStore) -> Result<()> {
    let mut reader = BufReader::new(&stream);
    let mut reader = serde_json::de::Deserializer::from_reader(&mut reader).into_iter::<Request>();

    let request: Request = reader.next().unwrap()?; // serde_json::from_reader(&mut reader)?;
    info!(logger, "incoming request"; "request" => format!("{:?}", request));

    let response = match request {
        Request::Set { key, value } => Response::set(store.set(key, value)),
        Request::Get { key } => Response::get(store.get(key)),
        Request::Remove { key } => Response::remove(store.remove(key)),
    };

    let content = serde_json::to_vec(&response)?;
    stream.write_all(&content)?;
    stream.flush()?;

    Ok(())
}
