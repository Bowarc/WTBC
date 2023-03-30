pub struct Client {
    socket: shared::networking::Socket<
        shared::networking::ClientMessage, // Reading
        shared::networking::DaemonMessage, // Writing
    >,
    pending_commands: Vec<crate::server::Command>,
}

////////////////////////////////////////
//          Client functions          //
////////////////////////////////////////

impl Client {
    pub fn new(stream: std::net::TcpStream) -> Self {
        Self {
            socket: shared::networking::Socket::<
                shared::networking::ClientMessage, // Reading
                shared::networking::DaemonMessage, // Writing
            >::new(stream),
            pending_commands: Vec::new(),
        }
    }

    pub fn update(
        &mut self,
        bgchanger: &mut crate::bgchanger::BgChanger,
    ) -> Result<(), crate::error::BackgroundChangerError> {
        match self.socket.recv() {
            Ok(message) => {
                debug!("Got a message from client: {message:?}",);
                // match message and send commands to the main thread

                let response = match message {
                    shared::networking::ClientMessage::Text(txt) => {
                        println!("Client send a text message: {txt}");
                        shared::networking::DaemonMessage::Text(txt)
                    }
                    shared::networking::ClientMessage::GetLogFile => {
                        shared::networking::DaemonMessage::LogFile(
                            std::path::PathBuf::from(crate::config::LOG_FILE_LOCATION)
                                .canonicalize()?
                                .as_path()
                                .display()
                                .to_string(),
                        )
                    }
                    shared::networking::ClientMessage::GetHistory => {
                        shared::networking::DaemonMessage::History(bgchanger.history.clone())
                    }
                    shared::networking::ClientMessage::SetBg => {
                        let number_of_tries = 5;
                        let return_text = if let Ok(tries_left) = bgchanger.set(number_of_tries) {
                            format!(
                                "Success, {tries_left} {tries} left ({prcentage_left}%)",
                                tries = {
                                    if tries_left > 1 {
                                        "tries"
                                    } else {
                                        "try"
                                    }
                                },
                                prcentage_left =
                                    ((tries_left as f32 / number_of_tries as f32) * 100.)
                            )
                        } else {
                            String::from("Couldn't set the background, please query the history for more information")
                        };
                        shared::networking::DaemonMessage::Text(return_text)
                    }
                };
                self.socket.send(response)?;
            }
            Err(e) => {
                if if let shared::networking::SocketError::IoError(ref a) = e {
                    a.kind() == std::io::ErrorKind::WouldBlock
                } else {
                    false
                } {

                    // println!("hehe");
                    // continue;
                } else {
                    error!("Error while listening for message: {e}");
                    Err(e)?;
                }
            }
        }

        Ok(())
    }
}

pub fn handle_client(stream: std::net::TcpStream) {
    let mut socket = shared::networking::Socket::<
        shared::networking::ClientMessage, // Reading
        shared::networking::DaemonMessage, // Writing
    >::new(stream);

    loop {

        // debug!("{:?}", message);
    }

    // match on messages, and send commands to the receiver
}
