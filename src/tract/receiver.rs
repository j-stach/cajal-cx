
use std::net::SocketAddr;
use super::Tract;

/// Represents a tract endpoint capable of receiving impulses.
pub trait TractReceiver: Tract {

    /// Generate a ReceiverReport for use in tract connection.
    fn receiver_info(&self) -> ReceiverInfo {
        ReceiverInfo {
            tract_name: self.tract_name().to_owned(),
            address: self.address().clone(),
            num_fibers: self.num_fibers(),
        }
    }
}

/// This struct holds connection information for targeting an Input via `Output::connect_tract`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ReceiverInfo {

    /// The name of the Receiver (e.g., Input, Motor) 
    /// should correspond to the name of the Sender (e.g., Output, Sensor).
    pub tract_name: String,

    /// This is the port where the Receiver listens for firing signals.
    /// The signal source is responsible for targeting this socket.
    /// It can be from an Output in another Complex either local or distributed,
    /// or it can be from a phantom_limb::Sensor in another application.
    pub address: SocketAddr,

    /// The number of fibers in the Sender and Receiver should be consistent.
    pub num_fibers: usize,
}

