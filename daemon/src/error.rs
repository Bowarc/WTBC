use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackgroundChangerError {
    #[error("uh oh")]
    Serde(#[from] serde_json::Error),
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not find the backgroundImage in the config")]
    Config,
    #[error("Could not convert something")]
    Convertion,
    #[error("A verification check did not go successfully")]
    Verification,
    #[error("Something went wrong during initialisation: {0}")]
    Initialisation(&'static str),
    #[error("An error occured while using the shared::networking::Socket: {0}")]
    SocketError(#[from] shared::networking::SocketError),
}
