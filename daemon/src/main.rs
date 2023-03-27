// #![windows_subsystem = "windows"]

mod config;

struct BgPath {
    path: std::path::PathBuf,
    files: Vec<String>, // Do a selection of the usable files in the directory, and keep this update-able
}

struct BgChanger {
    paths: Vec<BgPath>,
    wt_config_path: std::path::PathBuf,
    config: config::BgChangerConfig,
}

impl BgChanger {
    pub fn new(wt_config_path: std::path::PathBuf) -> Self {
        Self {
            paths: Vec::new(),
            wt_config_path,
            config: config::BgChangerConfig::default(),
        }
    }

    pub fn set(&mut self) {}

    pub fn get(&mut self) {}

    fn update(&mut self) {}
}

fn main() {
    // will later be made into a real daemon, (Windows service) but i don't care for now

    let bg_changer = BgChanger::new(std::path::PathBuf::from("C:\\Users\\Heto\\AppData\\Local\\Packages\\Microsoft.WindowsTerminal_8wekyb3d8bbwe\\LocalState\\settings.json"));

    let listener = std::net::TcpListener::bind(shared::networking::DEFAULT_ADDRESS).unwrap();

    let mut clients: Vec<std::thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming().flatten() {
        clients.push(std::thread::spawn(move || handle_client(stream)))
    }
}

fn handle_client(stream: std::net::TcpStream) {
    let mut socket = shared::networking::Socket::<
        shared::networking::ClientMessage,
        shared::networking::DaemonMessage,
    >::new(stream);
    let message = socket.recv();
    println!("{:?}", message);
}
