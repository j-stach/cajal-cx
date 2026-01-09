
use std::net::SocketAddr;

pub mod sender;
pub mod receiver;

pub trait Tract {

    /// Tract names should be reserved for `TractSender` and `TractReceiver` pairs.
    fn tract_name(&self) -> &str;

    /// This is the address to which sender will send and receiver will listen.
    fn tract_address(&self) -> SocketAddr;

    /// This is the number of internal incoming or outgoing fiber connections.
    fn num_fibers(&self) -> usize;
}

