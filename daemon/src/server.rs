pub enum Command {}

pub struct Server {
    clients: Vec<std::thread::JoinHandle<()>>,
    tcp_listener: std::net::TcpListener,
    command_channel: (
        std::sync::mpsc::Sender<Command>,
        std::sync::mpsc::Receiver<Command>,
    ),
}

impl Server {
    pub fn new() -> Self {
        let listener = std::net::TcpListener::bind(shared::networking::DEFAULT_ADDRESS).unwrap();
        listener.set_nonblocking(true).unwrap();

        Self {
            clients: Vec::new(),
            tcp_listener: listener,
            command_channel: std::sync::mpsc::channel::<Command>(),
        }
    }

    pub fn update(&mut self) {
        match self.tcp_listener.accept() {
            Ok((stream, addr)) => {
                debug!("New client {addr:?}");

                let clonned_sender = self.command_channel.0.clone();
                self.clients.push(std::thread::spawn(move || {
                    handle_client(stream, clonned_sender)
                }))
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                // println!("Would block");
                // continue;
            }

            Err(e) => {
                error!("Error while listening for clients: {e}");
            }
        }
    }

    pub fn harvest_commands(&mut self) -> Vec<Command> {
        let mut o = Vec::new();

        while let Ok(command) = self.command_channel.1.try_recv() {
            o.push(command)
        }

        o
    }
}

fn handle_client(stream: std::net::TcpStream, _command_sender: std::sync::mpsc::Sender<Command>) {
    let mut socket = shared::networking::Socket::<
        shared::networking::ClientMessage,
        shared::networking::DaemonMessage,
    >::new(stream);
    let message = socket.recv();
    debug!("{:?}", message);
    // match on messages, and send commands to the receiver
}
