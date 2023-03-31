#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum HistoryBit {
    BackgroundSet(String, String), // old, new
    ErrorOccured(String),
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct History {
    pub bits: Vec<(std::time::SystemTime, HistoryBit)>,
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

impl History {
    pub fn new() -> Self {
        Self { bits: Vec::new() }
    }

    pub fn add_background_set(&mut self, old: String, new: String) {
        self.bits.push((
            std::time::SystemTime::now(),
            HistoryBit::BackgroundSet(old, new),
        ))
    }

    pub fn add_error_occured(&mut self, error: impl ToString) {
        self.bits.push((
            std::time::SystemTime::now(),
            HistoryBit::ErrorOccured(error.to_string()),
        ))
    }
}
