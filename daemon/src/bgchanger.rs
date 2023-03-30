#[derive(Debug)]
pub struct BgPath {
    // Note, to display path you can use `.as_path().display().to_string().replace("\\\\?\\", "")`
    pub path: std::path::PathBuf,
    pub files: Vec<String>, // Do a selection of the usable files in the directory, and keep this update-able
}

pub struct BgChanger {
    pub paths: Vec<BgPath>,
    pub cfg: crate::config::BgChangerConfig,
    pub history: shared::server::history::History,
}

impl BgChanger {
    // init functions

    pub fn new(
        bg_paths: Vec<std::path::PathBuf>,
        cfg: crate::config::BgChangerConfig,
    ) -> Result<Self, crate::error::BackgroundChangerError> {
        // make sure that paths are corrects
        let bg_files_supported_extensions = ["png"];

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

        let mut bg_changer = Self {
            paths,
            cfg,
            history: shared::server::history::History::new(),
        };

        bg_changer.init()?;
        Ok(bg_changer)
    }
    fn init(&mut self) -> Result<(), crate::error::BackgroundChangerError> {
        if self.create_backup().is_err() {
            // Can't impl Clone on `crate::error::BackgroundChangerError` koz some Error type doesn't impl it HAHAHAH
            self.history
                .add_error_occured(crate::error::BackgroundChangerError::Initialisation(
                    "Backup creation failled",
                ));
            return Err(crate::error::BackgroundChangerError::Initialisation(
                "Backup creation failled",
            ));
        }

        // check for potential cancer
        if self.paths.is_empty() {
            // Can't impl Clone on `crate::error::BackgroundChangerError` koz some Error type doesn't impl it 00110 10101 00011 01011 00000 11001 01111 10101

            self.history
                .add_error_occured(crate::error::BackgroundChangerError::Initialisation(
                    "Paths are empty",
                ));
            return Err(crate::error::BackgroundChangerError::Initialisation(
                "Paths are empty",
            ));
        }
        if !self.cfg.is_ok() {
            // Can't impl Clone on `crate::error::BackgroundChangerError` koz some Error type doesn't impl it bruh
            self.history
                .add_error_occured(crate::error::BackgroundChangerError::Initialisation(
                    "Cfg not ok",
                ));
            return Err(crate::error::BackgroundChangerError::Initialisation(
                "Cfg not ok",
            ));
        }

        Ok(())
    }

    fn create_backup(&self) -> Result<(), crate::error::BackgroundChangerError> {
        // This function is supposed to be called ONLY ONE TIME at the program init

        let mut backup_path = self.cfg.wt_config_path.clone();
        backup_path.set_file_name(crate::config::BACKUP_FILE_NAME);
        println!("{backup_path:?}");

        std::fs::copy(self.cfg.wt_config_path.clone(), backup_path)
            .map(|_| ())
            .map_err(|e| e.into())
    }
}

impl BgChanger {
    // Other functions
    pub fn update(&mut self) {}
}

impl BgChanger {
    /// Core functions

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
    pub fn get(&self) -> Result<String, crate::error::BackgroundChangerError> {
        use std::io::Read as _;
        let mut content = String::new();

        // We don't need to hold the file
        drop(
            std::fs::OpenOptions::new()
                .read(true)
                .write(false)
                .open(self.cfg.wt_config_path.clone())?
                .read_to_string(&mut content),
        );

        let settings_file: serde_json::Value = serde_json::from_str(&content)?;

        // maybe check multiple locations (["profiles"], iter, find the backgroundImage key)
        return self
            .cfg
            .background_field_location
            .get(settings_file)?
            .as_str()
            .ok_or(crate::error::BackgroundChangerError::Convertion)
            .map(|bg| bg.to_string());
    }

    pub fn set(&mut self, mut tries: i32) -> Result<i32, crate::error::BackgroundChangerError> {
        loop {
            if tries == 0 {
                return Err(crate::error::BackgroundChangerError::Config);
            }

            match self.__set() {
                Ok((old, new)) => {
                    self.history.add_background_set(old, new);
                    return Ok(tries);
                }
                Err(e) => {
                    self.history.add_error_occured(e);

                    // restore config from backu
                    let backup_file_path = {
                        let mut o = self.cfg.wt_config_path.clone();
                        o.set_file_name(crate::config::BACKUP_FILE_NAME);
                        o
                    };

                    // Here we returns if this fails as there is not point continuing the same mistake on the same file
                    std::fs::copy(backup_file_path, self.cfg.wt_config_path.clone()).map(|_| ())?;

                    tries -= 1;
                }
            }
        }
    }
    fn __set(&self) -> Result<(String, String), crate::error::BackgroundChangerError> {
        use std::io::Read as _;

        let old_bg = self.get()?;

        // Ugly af, don't like this part
        let new_bg = loop {
            if let Some(bg) = self.select_random_bg() {
                break bg;
            }
        }
        .as_path()
        .display()
        .to_string()
        .replace("\\\\?\\", "")
        .replace('\\', "/");

        let mut original_content = String::new();

        drop(
            std::fs::OpenOptions::new()
                .read(true)
                .write(false)
                .open(self.cfg.wt_config_path.clone())?
                .read_to_string(&mut original_content),
        );

        debug!("Replacing '{old_bg}' by '{new_bg}'");

        let new_content = original_content.replace(&old_bg, &new_bg);

        std::fs::write(self.cfg.wt_config_path.clone(), new_content)?;

        // verify that the file has be written successfully
        self.get().and_then(|bg| {
            if bg == new_bg {
                Ok((old_bg, new_bg))
            } else {
                Err(crate::error::BackgroundChangerError::Verification)
            }
        })
    }
}
