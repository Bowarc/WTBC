// #![windows_subsystem = "windows"]

mod config;

#[derive(Debug)]
struct BgPath {
    // Note, to display path you can use `.as_path().display().to_string().replace("\\\\?\\", "")`
    path: std::path::PathBuf,
    files: Vec<String>, // Do a selection of the usable files in the directory, and keep this update-able
}

struct BgChanger {
    paths: Vec<BgPath>,
    cfg: config::BgChangerConfig,
}

impl BgChanger {
    pub fn new(
        bg_paths: Vec<std::path::PathBuf>,
        wt_config_path: std::path::PathBuf,
        sleep_time: config::SleepTime,
        background_field_loc: &'static str,
    ) -> Self {
        // make sure that paths are corrects
        let bg_files_supported_extensions = ["png"];

        // working but not as efficient
        // for path in bg_paths {
        //     if let Ok(canon_path) = path.canonicalize() {
        //         if let Ok(read_dir) = canon_path.read_dir() {
        //             let mut bgpath = BgPath {
        //                 path,
        //                 files: Vec::new(),
        //             };
        //             for entry in read_dir.flatten() {
        //                 if let Some(ext) = entry.path().extension() {
        //                     if let Some(ext_str) = ext.to_str() {
        //                         if bg_files_supported_extensions.contains(&ext_str) {
        //                             if let Some(file_name) = entry.file_name().to_str() {
        //                                 bgpath.files.push(file_name.to_string())
        //                             }
        //                         }
        //                     }
        //                 }
        //             }
        //             verified_bg_paths.push(bgpath)
        //         }
        //     }
        // }

        // working but not as efficient
        // let paths: Vec<BgPath> = bg_paths
        //     .iter()
        //     .flat_map(|p| p.canonicalize())
        //     .map(|cannon_path| {
        //         let ok_files = cannon_path
        //             .read_dir()
        //             .into_iter()
        //             .flatten()
        //             .flatten()
        //             .filter(|read_dir| {
        //                 read_dir
        //                     .path()
        //                     .extension()
        //                     .and_then(|x| x.to_str())
        //                     .map(|ext_str| bg_files_supported_extensions.contains(&ext_str))
        //                     .unwrap_or(false)
        //             })
        //             .map(|entry| entry.file_name())
        //             .flat_map(|file_name| file_name.to_str().map(|name_str| name_str.to_string()))
        //             .collect::<Vec<String>>();

        //         BgPath {
        //             path: cannon_path,
        //             files: ok_files,
        //         }
        //     })
        //     .collect();

        let mut paths = bg_paths
            .iter()
            .flat_map(|p| p.canonicalize())
            .filter_map(|cannon_path| {
                let files = cannon_path
                    .read_dir()
                    .ok()?
                    .flatten()
                    .filter_map(|entry| {
                        let entry_path = entry.path();
                        let ext_str = entry_path.extension()?.to_str()?;
                        bg_files_supported_extensions
                            .contains(&ext_str)
                            .then(|| entry.file_name().into_string().ok())
                            .flatten()
                    })
                    .collect();
                Some(BgPath {
                    path: cannon_path,
                    files,
                })
            })
            .collect::<Vec<BgPath>>();

        paths.retain(|p| !p.files.is_empty());
        assert!(!paths.is_empty(), "What background do i set ?");

        assert!(wt_config_path.exists());

        let cfg = config::BgChangerConfig::new(
            sleep_time,
            config::BgFieldLoc(background_field_loc),
            wt_config_path,
        );
        assert!(cfg.is_ok());

        Self { paths, cfg }
    }
    pub fn select_random_bg(&self) -> Option<std::path::PathBuf> {
        use rand::prelude::SliceRandom as _;
        let bg_path = self.paths.choose(&mut rand::thread_rng())?;

        let bg_file_name = bg_path.files.choose(&mut rand::thread_rng())?;

        let path =
            std::path::PathBuf::from(format!("{}\\{}", bg_path.path.display(), bg_file_name));

        if !path.exists() {
            return None;
        }
        Some(path)
    }

    pub fn set(&self) -> Result<(), Box<dyn std::error::Error>> {
        use std::io::{Read as _, Write as _};

        // Ugly af, don't like this part
        let new_bg = loop {
            if let Some(bg) = self.select_random_bg() {
                break bg;
            }
        };

        let mut original_content = String::new();

        drop(
            std::fs::OpenOptions::new()
                .read(true)
                .write(false)
                .open(self.cfg.wt_config_path.clone())?
                .read_to_string(&mut original_content),
        );

        let old_json: serde_json::Value = serde_json::from_str(&original_content)?;

        let mut json_content = old_json.clone();

        self.cfg
            .background_field_location
            .set(
                &mut json_content,
                new_bg
                    .as_path()
                    .display()
                    .to_string()
                    .replace("\\\\?\\", ""),
            )
            .ok_or("Can't set bg")?;

        assert!(old_json != json_content);

        let new_content = json_content.to_string();

        let pretty_content = format!("{json_content:#?}");

        // println!("New content:\n{json_content:#?}");

        println!("Writing to {:?}", self.cfg.wt_config_path);

        std::fs::OpenOptions::new()
            .read(false)
            .write(true)
            .open(self.cfg.wt_config_path.clone())?
            .write_all(pretty_content.as_bytes())?;

        Ok(())
    }

    pub fn get(&self) -> Option<String> {
        use std::io::Read as _;
        let mut content = String::new();

        // We don't need to hold the file
        drop(
            std::fs::OpenOptions::new()
                .read(true)
                .write(false)
                .open(self.cfg.wt_config_path.clone())
                .ok()?
                .read_to_string(&mut content),
        );

        let settings_file: serde_json::Value = serde_json::from_str(&content).ok()?;

        // maybe check multiple locations (["profiles"], iter, find the backgroundImage key)
        return self
            .cfg
            .background_field_location
            .get(settings_file)?
            .as_str()
            .map(|bg| bg.to_string());

        // return settings_file["profiles"]["defaults"]["backgroundImage"]
        //     .as_str()
        //     .map(|bg| bg.to_string());

        // but keeping the files comments would be cool
        // deserializing the data removes the coments (as you don't deserialize comments)
        // https://github.com/hjson/hjson-rust can be the solution ?
        // Well, pure JSON doesn't allow comments. you can add some, but you're not supposed to.
        // my project will then follow this route, if you need to store more info, use a `readme.md`
        // in the config's directory

        // working but not as efficient
        // for line in content.lines() {
        //     // Try to split the line in the ':' since WT settigns i just a big Key-Value map
        //     let splitted_line = line.split(':').collect::<Vec<&str>>();

        //     if splitted_line.len() < 2 {
        //         println!("Skipping: '{line}'");
        //         continue;
        //     }
        //     if splitted_line.len() > 2 {
        //         println!(
        //             "Tf is going on: '{line}' --- {splitted_line:?} -- {}",
        //             splitted_line.len()
        //         );
        //         continue;
        //     }

        //     let key = splitted_line[0]
        //         .replace(['\n', '\"', ' ', ':'], "")
        //         .replace(KEYWORD, "");

        //     if key == KEYWORD {
        //         output = splitted_line[1].replace([',', ' ', '"'], "");
        //     }

        //     // if line.replace([' ', '"'], "").starts_with(KEYWORD) {
        //     //     let bg = line
        //     //         .replace(['\n', '\"', ' ', ':'], "")
        //     //         .replace(KEYWORD, "");
        //     //     println!("{bg}");
        //     //     output = bg;
        //     //     break;
        //     // }
        // }

        // match output == String::new() {
        //     true => None,
        //     false => Some(output),
        // }
    }

    fn update(&mut self) {}
}

fn main() {
    // will later be made into a real daemon, (Windows service) but i don't care for now

    let bg_changer = BgChanger::new(
        vec![
            // std::path::PathBuf::from("D:\\Links\\Pictures\\"),
            std::path::PathBuf::from("D:\\Links\\Pictures\\lol_skins\\"),
        ],
        std::path::PathBuf::from("C:\\Users\\Heto\\AppData\\Local\\Packages\\Microsoft.WindowsTerminal_8wekyb3d8bbwe\\LocalState\\settings.json"),
        config::SleepTime::Fix(std::time::Duration::from_secs(45)),
        "/profiles/defaults/backgroundImage",
    );

    dbg!(bg_changer.get());

    dbg!(bg_changer.set().unwrap());

    if false {
        let listener = std::net::TcpListener::bind(shared::networking::DEFAULT_ADDRESS).unwrap();

        let mut clients: Vec<std::thread::JoinHandle<()>> = Vec::new();

        for stream in listener.incoming().flatten() {
            clients.push(std::thread::spawn(move || handle_client(stream)))
        }
    }

    // std::thread::sleep(std::time::Duration::from_secs(20));
}

fn handle_client(stream: std::net::TcpStream) {
    let mut socket = shared::networking::Socket::<
        shared::networking::ClientMessage,
        shared::networking::DaemonMessage,
    >::new(stream);
    let message = socket.recv();
    println!("{:?}", message);
}

#[test]
fn test_bg_changer() {
    let bg_changer = BgChanger::new(
        vec![
            std::path::PathBuf::from("D:\\Links\\Pictures\\"),
            std::path::PathBuf::from("D:\\Links\\Pictures\\lol_skins\\"),
        ],
        std::path::PathBuf::from("C:\\Users\\Heto\\AppData\\Local\\Packages\\Microsoft.WindowsTerminal_8wekyb3d8bbwe\\LocalState\\settings.json"),
        config::SleepTime::Fix(std::time::Duration::from_secs(45)),
        "/profiles/defaults/backgroundImage",
    );

    assert!(!bg_changer.paths.is_empty());

    for path in &bg_changer.paths {
        assert!(!path.files.is_empty())
    }

    dbg!(&bg_changer.paths);

    for _ in 0..=10 {
        let o = bg_changer.select_random_bg();
        let mut msg = format!("{o:?}");

        if let Some(p) = o {
            msg.push_str(&format!("{}", p.exists()));
            msg.push_str(&format!(
                "\n{}",
                p.as_path().display().to_string().replace("\\\\?\\", "")
            ))
        }

        println!("{msg}");
    }

    println!("");
    println!("actual bg: {:?}", bg_changer.get());

    // dbg!(&bg_changer.set());
}
