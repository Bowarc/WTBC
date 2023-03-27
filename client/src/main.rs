// #![windows_subsystem = "windows"]

fn main() {
    let stream = std::net::TcpStream::connect(shared::networking::DEFAULT_ADDRESS).unwrap();

    let mut socket = shared::networking::Socket::<
        shared::networking::DaemonMessage,
        shared::networking::ClientMessage,
    >::new(stream);

    let message = shared::networking::ClientMessage::Text(String::from("Hellow\nAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"));

    println!("Connected, sending: {:?}", message);

    socket.send(message).unwrap();

    println!("Success");
}
