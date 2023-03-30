// #![windows_subsystem = "windows"]

#[macro_use]
extern crate log;

mod bgchanger;
mod config;
mod error;
mod logger;
mod server;

fn main() {
    // will later be made into a real daemon, (Windows service) but i don't care for now

    logger::init();

    info!("Init");

    let mut bg_changer = bgchanger::BgChanger::new(
        vec![
            // std::path::PathBuf::from("D:\\Links\\Pictures\\"),
            std::path::PathBuf::from("D:\\Links\\Pictures\\lol_skins\\"),
        ],
        config::BgChangerConfig::new(
            config::SleepTime::Fix(std::time::Duration::from_secs(45)),
            config::BgFieldLoc("/profiles/defaults/backgroundImage"),
            std::path::PathBuf::from("C:\\Users\\Heto\\AppData\\Local\\Packages\\Microsoft.WindowsTerminal_8wekyb3d8bbwe\\LocalState\\settings.json"),
        ),
    ).unwrap();

    // dbg!(bg_changer.get());

    // dbg!(bg_changer.set(5));

    // Set up the server

    let mut server = server::Server::new();
    loop {
        std::thread::sleep(std::time::Duration::from_millis(50));

        bg_changer.update();
        // bg_changer.set(5).unwrap();
        server.update(&mut bg_changer);
    }
}

#[test]
fn test_bg_changer() {
    let bg_changer = bgchanger::BgChanger::new(
        vec![
            std::path::PathBuf::from("D:\\Links\\Pictures\\"),
            std::path::PathBuf::from("D:\\Links\\Pictures\\lol_skins\\"),
        ],
        config::BgChangerConfig::new(
            config::SleepTime::Fix(std::time::Duration::from_secs(45)),
            config::BgFieldLoc("/profiles/defaults/backgroundImage"),
            std::path::PathBuf::from("C:\\Users\\Heto\\AppData\\Local\\Packages\\Microsoft.WindowsTerminal_8wekyb3d8bbwe\\LocalState\\settings.json"),
        ),
    ).unwrap();

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

    println!();
    println!("actual bg: {:?}", bg_changer.get());

    // dbg!(&bg_changer.set());
}
