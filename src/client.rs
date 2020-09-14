use crate::error::{Error, ErrorKind};
use crate::{Request, Response, Result};
use serde_json::de::IoRead;
use serde_json::StreamDeserializer;
use std::io::{BufReader, BufWriter, Write};
use std::net::TcpStream;

/// The client of key-value store.
pub struct KvsClient<'de> {
    reader: StreamDeserializer<'de, IoRead<BufReader<TcpStream>>, Response>,
    writer: BufWriter<TcpStream>,
}

impl<'de> KvsClient<'de> {
    /// Connect the remote server, and get a new key-value store client.
    pub fn connect(addr: &str) -> Result<KvsClient<'de>> {
        let stream = TcpStream::connect(addr)?;
        let writer = BufWriter::new(stream.try_clone()?);
        let reader = BufReader::new(stream);
        let reader = serde_json::de::Deserializer::from_reader(reader).into_iter::<Response>();

        Ok(KvsClient { reader, writer })
    }

    /// Sets the value of a string key to a string.
    /// Return an error if the value is not written successfully.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let request = Request::Set { key, value };
        let request = serde_json::to_vec(&request)?;

        self.writer.write_all(&request.as_slice())?;
        self.writer.flush()?;

        let next = self.reader.next().ok_or(ErrorKind::UnexpectedError(
            "Can not deserialize next response",
        ))?;
        match next? {
            Response::Set(Ok(_)) => Ok(()),
            Response::Set(Err(err)) => Err(Error::from(ErrorKind::StringError(err))),
            _ => Err(Error::from(ErrorKind::UnexpectedError(
                "Client received an unexpected response",
            ))),
        }
    }

    /// Gets the string value of the a string key.
    /// If the key does not exist, return None.
    /// Return an error if the value is not read successfully.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let request = Request::Get { key };
        let request = serde_json::to_vec(&request)?;

        self.writer.write_all(&request.as_slice())?;
        self.writer.flush()?;

        let next = self.reader.next().ok_or(ErrorKind::UnexpectedError(
            "Can not deserialize next response",
        ))?;
        match next? {
            Response::Get(Ok(content)) => Ok(content),
            Response::Get(Err(err)) => Err(Error::from(ErrorKind::StringError(err))),
            _ => Err(Error::from(ErrorKind::UnexpectedError(
                "Client received an unexpected response",
            ))),
        }
    }

    /// Removes a given key.
    /// Return an error if the key does not exist or is not removed successfully.
    pub fn remove(&mut self, key: String) -> Result<()> {
        let request = Request::Remove { key };
        let request = serde_json::to_vec(&request)?;

        self.writer.write_all(&request.as_slice())?;
        self.writer.flush()?;

        let next = self.reader.next().ok_or(ErrorKind::UnexpectedError(
            "Can not deserialize next response",
        ))?;
        match next? {
            Response::Remove(Ok(_)) => Ok(()),
            Response::Remove(Err(err)) => Err(Error::from(ErrorKind::StringError(err))),
            _ => Err(Error::from(ErrorKind::UnexpectedError(
                "Client received an unexpected response",
            ))),
        }
    }
}
