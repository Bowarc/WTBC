pub const LOG_FILE_LOCATION: &str = "logs.wtbc.log";
pub const BACKUP_FILE_NAME: &str = "settings.json.wtbc.backup";

pub enum SleepTime {
    Fix(std::time::Duration),
    Range(std::time::Duration, std::time::Duration),
    // possible values ? Like a Vec<std::time::Duration> ?
}

pub struct BgFieldLoc(pub &'static str);

pub struct BgChangerConfig {
    pub sleep_time: SleepTime,
    pub background_field_location: BgFieldLoc, // location of the targeted field in the WTconfig file
    pub wt_config_path: std::path::PathBuf,
}

impl BgChangerConfig {
    pub fn new(
        sleep_time: SleepTime,
        background_field_location: BgFieldLoc,
        wt_config_path: std::path::PathBuf,
    ) -> Self {
        Self {
            sleep_time,
            background_field_location,
            wt_config_path,
        }
    }

    pub fn is_ok(&self) -> bool {
        // more later ?
        self.wt_config_path.exists()
    }
}
impl SleepTime {
    pub fn get(&self) -> std::time::Duration {
        match self {
            SleepTime::Fix(d) => *d,
            SleepTime::Range(min, max) => {
                use rand::Rng;
                let min_nanos = min.as_nanos();
                let max_nanos = max.as_nanos();

                let rdm_value = rand::thread_rng().gen_range(min_nanos..max_nanos);

                std::time::Duration::from_nanos(rdm_value as u64)
            }
        }
    }
}

impl BgFieldLoc {
    pub fn get(
        &self,
        value: serde_json::Value,
    ) -> Result<serde_json::Value, crate::error::BackgroundChangerError> {
        value
            .pointer(self.0)
            .cloned()
            .ok_or(crate::error::BackgroundChangerError::Config)
    }
}
