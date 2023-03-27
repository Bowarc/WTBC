pub enum SleepTime {
    Fix(std::time::Duration),
    Range(std::time::Duration, std::time::Duration),
    // possible values ? Like a Vec<std::time::Duration> ?
}

pub struct BgChangerConfig {
    sleep_time: SleepTime,
}

impl Default for BgChangerConfig {
    fn default() -> Self {
        Self {
            sleep_time: SleepTime::Fix(std::time::Duration::from_secs(45)),
        }
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
