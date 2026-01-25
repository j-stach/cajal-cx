
use crate::error::TractError;
use super::{ Tract, receiver::ReceiverReport };

/// Represents a tract endpoint capable of sending impulses.
///
/// WARNING: Contains async methods. 
/// Use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified.
///
/// Intended for development of `cajal` and `phantom_limb`, not for public implementation.
/// Use at your own discretion.
#[allow(async_fn_in_trait)]
pub trait TractSender: Tract {

    /// Link the sender to receiver if they are compatible.
    /// This method only sets the sender's address based on the receiver;
    /// use the async method `connect` to connect the socket.
    fn link(&mut self, report: ReceiverReport) -> Result<(), TractError> {

        if self.tract_name() == report.tract_name {

            let num_senders = self.num_fibers();
            let num_fibers = report.num_fibers;

            if num_senders == num_fibers {
                *self.address_mut() = report.address;
                Ok(())
            } else {
                return Err(TractError::FiberCountMismatch(
                    num_senders,
                    num_fibers
                ))
            }

        } else {
            return Err(TractError::NameMismatch(
                self.tract_name().to_owned(),
                report.tract_name.to_owned(),
            ))
        }
    }

    /// Connect the sender's UdpSocket to the stored address.
    /// Must be called from within the `tokio` runtime, 
    /// but before starting the neurotransmission loop.
    async fn connect(&mut self) -> Result<(), std::io::Error> {
        self.socket().connect(self.address()).await
    }

}

