use clap::*;
use slog::*;

use kvs::{KvStore, Result, KvsServer, SledKvsEngine, KvsEngine};
use slog::Logger;
use std::env::current_dir;
use std::process;
use std::fs;
use std::path::PathBuf;

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

    let addr = matches
        .value_of("IP-PORT")
        .expect("IP-PORT argument is missing.");

    let engine = matches
        .value_of("ENGINE-NAME")
        .expect("ENGINE-NAME argument is missing.");

    run(addr, engine, logger)
}

fn get_logger() -> Logger {
    let drain = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(drain).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, o!())
}

fn run(addr: &str, engine: &str, logger: Logger) -> Result<()> {
    info!(logger, "kvs initializing";
        "version" => crate_version!(),
        "engine" => engine,
         "ip" => addr
    );

    let current_dir = current_dir()?;
    match current_engine(&current_dir)? {
        Some(e) if e != engine => {
            error!(logger, "wrong engine in this directory");
            process::exit(1);
        }
        _ => ()
    }

    fs::write(current_dir.join("engine"), engine)?;

    match engine {
        "kvs" => run_with_engine(KvStore::open(current_dir)?, addr, logger),
        "sled" => run_with_engine(SledKvsEngine::open(current_dir)?, addr, logger),
        _ => {
            eprintln!("Unsupported engine");
            process::exit(1);
        }
    }
}

fn run_with_engine(engine: impl KvsEngine, addr: &str, logger: Logger) -> Result<()> {
    let mut server = KvsServer::new(engine);
    server.run(addr, &logger)
}

fn current_engine(path: &PathBuf) -> Result<Option<String>> {
    let path = path.join("engine");
    if !path.exists() {
        return Ok(None)
    }

    let engine = fs::read_to_string(path)?;
    Ok(Some(engine))
}