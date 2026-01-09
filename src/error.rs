
/// Error that occurs when Sender and Receiver are incompatible for connection.
#[derive(Debug, thiserror::Error)]
pub enum TractError {

    #[error("Couldn't create socket: {0}")]
    SocketFailed(#[from] std::io::Error),

    #[error("Mismatched tract names: Sender \"{0}\" cannot connect to Receiver \"{1}\"")]
    NameMismatch(String, String),

    #[error("Mismatched fiber counts: Sender has {0}, Receiver has {1}")]
    FiberCountMismatch(usize, usize),
}

