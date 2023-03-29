pub enum HistoryBit {
    BackgroundSet(String, String), // old, new
    ErrorOccured(crate::error::BackgroundChangerError),
}

pub struct History {
    pub bits: Vec<HistoryBit>,
}

impl History {
    pub fn new() -> Self {
        Self { bits: Vec::new() }
    }

    pub fn add_background_set(&mut self, old: String, new: String) {
        self.bits.push(HistoryBit::BackgroundSet(old, new))
    }

    pub fn add_error_occured(&mut self, error: crate::error::BackgroundChangerError) {
        self.bits.push(HistoryBit::ErrorOccured(error))
    }
}
