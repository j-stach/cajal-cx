
pub mod sender;
pub mod receiver;

pub trait Tract {

    /// Tract names should be unique for `TractSender` and `TractReceiver` pairs.
    fn tract_name(&self) -> &str;

    /// Address to which sender will send 
    /// and where the receiver will listen.
    fn address(&self) -> std::net::SocketAddr;

    /// Change the address to which sender will send 
    /// and where the receiver will listen.
    fn address_mut(&mut self) -> &mut std::net::SocketAddr;

    /// Socket used by the tract.
    fn socket(&self) -> std::sync::Arc<tokio::net::UdpSocket>;

    /// Number of internal incoming or outgoing fiber connections.
    fn num_fibers(&self) -> usize;
}

