use slog::{Logger, info, o};
use std::net::{TcpStream, TcpListener};
use std::io::{BufReader, Write, BufWriter};
use crate::{KvStore, Response, Request};
use crate::{Result};

/// The server of key-value store.
pub struct KvsServer {
    store: KvStore,
}

impl KvsServer {

    /// Create a new key-value store server.
    #[inline]
    pub fn new(store: KvStore) -> Self {
        KvsServer { store }
    }

    /// Run the server listening on a given ip address working with a slog logger.
    pub fn run(&mut self, addr: &str, logger: &Logger) -> Result<()> {
        let listener = TcpListener::bind(addr)?;

        for stream in listener.incoming() {
            let stream = stream?;
            let peer_addr = stream.peer_addr()?;

            let client = logger.new(o!("address" => peer_addr));
            info!(client, "incoming client");
            self.serve(stream, &client)?;
        }

        Ok(())
    }

    fn serve(&mut self, stream: TcpStream, logger: &Logger) -> Result<()> {
        let mut writer = BufWriter::new(&stream);
        let mut reader = BufReader::new(&stream);
        let reader = serde_json::de::Deserializer::from_reader(&mut reader).into_iter::<Request>();

        for request in reader {
            let request = request?;
            info!(logger, "request came"; "request" => format!("{:?}", request));

            let response = match request {
                Request::Set { key, value } => Response::set(self.store.set(key, value)),
                Request::Get { key } => Response::get(self.store.get(key)),
                Request::Remove { key } => Response::remove(self.store.remove(key)),
            };

            let content = serde_json::to_vec(&response)?;
            writer.write_all(&content)?;
            writer.flush()?;
        }

        Ok(())
    }
}
