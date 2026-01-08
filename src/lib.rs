
/// This struct holds connection information for targeting an Input via `Output::connect_tract`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReceiverReport {

    /// The name of the Receiver (e.g., Input, Motor) 
    /// should correspond to the name of the Sender (e.g., Output, Sensor).
    pub tract_name: String,

    /// This is the port where the Receiver listens for firing signals.
    /// The signal source is responsible for targeting this socket.
    /// It can be from an Output in another Complex either local or distributed,
    /// or it can be from a phantom_limb::Sensor in another application.
    pub address: std::net::SocketAddr,

    /// The number of fibers in the Sender and Receiver should be consistent.
    pub num_fibers: usize,
}


