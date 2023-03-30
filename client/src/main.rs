// #![windows_subsystem = "windows"]

fn main() {
    let stream = std::net::TcpStream::connect(shared::networking::DEFAULT_ADDRESS).unwrap();

    let mut socket = shared::networking::Socket::<
        shared::networking::DaemonMessage,
        shared::networking::ClientMessage,
    >::new(stream);

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let message = match input.replace(['\n', '\r'], "").as_str() {
            "getlog" => shared::networking::ClientMessage::GetLogFile,
            "gethistory" => shared::networking::ClientMessage::GetHistory,
            "setbg" => shared::networking::ClientMessage::SetBg,
            "exit" => std::process::exit(0),
            text => shared::networking::ClientMessage::Text(text.to_string()),
        };

        // let message = shared::networking::ClientMessage::Text(input.replace(['\n', '\r'], "")); //

        println!("Sending: {:?}", message);
        socket.send(message).unwrap();

        println!("Waiting for the server to respond");

        let response = loop {
            std::thread::sleep(std::time::Duration::from_millis(10));

            if let Ok(response) = socket.recv() {
                break response;
            }
        };

        println!("Server sent: {response:?}")
    }
}
