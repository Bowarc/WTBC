use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

pub const HEADER_SIZE: usize = std::mem::size_of::<PacketHeader>();

pub const DEFAULT_ADDRESS: std::net::SocketAddr = std::net::SocketAddr::V4(
    std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 1509),
);

#[derive(Serialize, Deserialize, Debug)]
// ofc don't use type that can change size (such as Vec) so the size of the struct stays the same as the constant
pub struct PacketHeader {
    size: usize,
}

// I don't like how streams work so i'll make a simple socket-like, packet-based struct wrapper
pub struct Socket<R, W> {
    stream: std::net::TcpStream,
    buffer: Vec<u8>,
    read_type: std::marker::PhantomData<R>,
    write_type: std::marker::PhantomData<W>,
}

#[derive(Error, Debug)]
pub enum SocketError {
    #[error("Not enough data")]
    NotEnoughData,
    #[error("meh")]
    Unknown,
    #[error("uh oh")]
    SerializeError(#[from] bincode::Error),
    #[error("ee")]
    IoError(#[from] std::io::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Text(String),
}
#[derive(Serialize, Deserialize, Debug)]
pub enum DaemonMessage {}

impl<R: DeserializeOwned + std::fmt::Debug, W: Serialize + std::fmt::Debug> Socket<R, W> {
    pub fn new(stream: std::net::TcpStream) -> Self {
        Self {
            stream,
            buffer: Vec::new(),
            read_type: std::marker::PhantomData,
            write_type: std::marker::PhantomData,
        }
    }
    pub fn send(&mut self, message: W) -> Result<(), SocketError> {
        use std::io::Write as _;

        println!("Serializing message..");
        let message_bytes = bincode::serialize(&message)?;
        println!("Serializing message.. Done, {} bytes", message_bytes.len());

        println!("Creating header..");
        let header = PacketHeader::new(message_bytes.len());
        println!("Creating header.. Done, {header:?}");

        println!("Serializing header..");
        let header_bytes = bincode::serialize(&header)?;
        println!("Serializing header.. Done, {} bytes", header_bytes.len());

        assert_eq!(header_bytes.len(), HEADER_SIZE);

        println!("Writing header to stream..");
        self.stream.write_all(&header_bytes)?;
        println!("Writing header to stream.. Ok");
        println!("Writing message to stream..");
        self.stream.write_all(&message_bytes)?;
        println!("Writing message to stream.. Ok");

        println!("Exiting send function");
        Ok(())
    }
    pub fn recv(&mut self) -> Result<R, SocketError> {
        use std::io::Read as _;

        let mut header_buffer: [u8; HEADER_SIZE] = [0; HEADER_SIZE];

        println!("Reading header..");
        self.stream.read_exact(&mut header_buffer)?;
        println!("Reading header.. Done, {} bytes", header_buffer.len());

        println!("Deserializing header..");
        let header: PacketHeader = bincode::deserialize(&header_buffer)?;
        println!("Deserializing header.. Done: {header:?}");

        let mut message_buffer = vec![0; header.size];

        println!("Reading message ({} bytes)..", header.size);
        self.stream.read_exact(&mut message_buffer)?;
        println!(
            "Reading message ({} bytes).. Done, {} bytes",
            header.size,
            message_buffer.len()
        );

        println!("Deserializing message..");
        let message = bincode::deserialize(&message_buffer)?;
        println!("Deserializing message.. Done, {message:?}");

        Ok(message)
    }
}

impl PacketHeader {
    pub fn new(size: usize) -> Self {
        Self { size }
    }
}
