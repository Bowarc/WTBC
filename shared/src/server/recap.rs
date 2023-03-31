#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Recap {
    pub errors: Vec<(std::time::SystemTime, String)>,
    pub number_of_bgset: usize,
    pub time_until_next_bgset: std::time::Duration,
    pub actual_bg: String,
}
