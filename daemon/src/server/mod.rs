pub mod client;

pub struct Server {
    client: Option<client::Client>,
    tcp_listener: std::net::TcpListener,
}

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
                    stream.set_nodelay(true).unwrap(); // ?

                    self.client = Some(client::Client::new(stream));
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP
                    // println!("Would block");
                    // continue;

                    // About this part, as the implementation is non-blocking,
                    // i'll assume that the program will do some other job before getting back to this part,
                    // therefore the socket will have time to do it's things
                }

                Err(e) => {
                    error!("Error while listening for clients: {e:?}");
                }
            }
        }
    }
}
