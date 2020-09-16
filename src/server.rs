use crate::engine::KvsEngine;
use crate::Result;
use crate::{Request, Response};
use slog::{info, error, o, Logger};
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use crate::thread_pool::{ThreadPool};
use std::sync::Arc;

/// The server of key-value store.
pub struct KvsServer<E: KvsEngine, T: ThreadPool + Send> {
    engine: E,
    thread_pool: T,
}

impl<E: KvsEngine, T: ThreadPool + Send> KvsServer<E, T> {
    /// Create a new key-value store server.
    #[inline]
    pub fn new(engine: E, thread_pool: T) -> Self {
        KvsServer { engine, thread_pool }
    }

    /// Run the server listening on a given ip address working with a slog logger.
    pub fn run(&mut self, addr: &str, logger: Logger) -> Result<()> {
        let logger = Arc::new(logger);
        let listener = TcpListener::bind(addr)?;

        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                let peer_addr = stream.peer_addr()?;
                let logger = Arc::clone(&logger);

                let store = self.engine.clone();
                self.thread_pool.spawn(move || {
                    let client = logger.new(o!("address" => peer_addr));
                    info!(client, "incoming client");

                    if let Err(err) = serve(store, stream, &client) {
                        error!(client, "Error on serving client"; "error" => format!("{}", err));
                    }
                });
            } else if let Err(err) = stream {
                error!(logger, "Connection failed"; "error" => format!("{}", err));
            }
        }

        Ok(())
    }
}

fn serve<E: KvsEngine>(store: E, stream: TcpStream, logger: &Logger) -> Result<()> {
    let mut writer = BufWriter::new(&stream);
    let mut reader = BufReader::new(&stream);
    let reader = serde_json::de::Deserializer::from_reader(&mut reader).into_iter::<Request>();

    for request in reader {
        if let Ok(request) = request {
            info!(logger, "request came"; "request" => format!("{:?}", request));

            let response = match request {
                Request::Set { key, value } => Response::set(store.set(key, value)),
                Request::Get { key } => Response::get(store.get(key)),
                Request::Remove { key } => Response::remove(store.remove(key)),
            };

            info!(logger, "reply"; "response" => format!("{:?}", response));
            let content = serde_json::to_vec(&response)?;
            writer.write_all(&content)?;
            writer.flush()?;
        } else {
            error!(logger, "can not parse the request");
        }
    }

    Ok(())
}
