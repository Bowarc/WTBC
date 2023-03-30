pub mod client;

pub struct Server {
    client: Option<client::Client>,
    tcp_listener: std::net::TcpListener,
}

#[derive(Debug)]
pub enum Command {}

#[derive(Debug)]
pub enum ServerMsg {}

impl Server {
    pub fn new() -> Self {
        let listener = std::net::TcpListener::bind(shared::networking::DEFAULT_ADDRESS).unwrap();
        listener.set_nonblocking(true).unwrap();

        Self {
            client: None,
            tcp_listener: listener,
        }
    }

    pub fn update(&mut self, bgchanger: &mut crate::bgchanger::BgChanger) {
        if let Some(client) = &mut self.client {
            if client.update(bgchanger).is_err() {
                self.client = None
            }
        } else {
            match self.tcp_listener.accept() {
                Ok((stream, addr)) => {
                    debug!("New client {addr:?}");
                    stream.set_nodelay(true).unwrap();
                    // We can't afford to block the main thread to wait for clients, but as each client
                    // has it's own thread, there is no problem blocking the client's thread.
                    // Unless we want the client thread to receive infos about the outside of it's thread (which it will later..)
                    // stream.set_nonblocking(false).unwrap();

                    self.client = Some(client::Client::new(stream));
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP
                    // println!("Would block");
                    // continue;
                }

                Err(e) => {
                    error!("Error while listening for clients: {e:?}");
                }
            }
        }
    }
}

// wait until network socket is ready, typically implemented
// via platform-specific APIs such as epoll or IOCP
// println!("Would block");
