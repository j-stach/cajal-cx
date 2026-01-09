
use std::net::SocketAddr;

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
    
    /// Set the target address to reflect the address of the Receiver.
    /// Do not use this method directly, opt instead for `connect_tract`.
    async fn set_target_address(&mut self, addr: SocketAddr) -> Result<(), std::io::Error>;

    /// Connect the sender to receiver if they are compatible.
    async fn connect_tract(&mut self, report: ReceiverReport) -> Result<(), TractError> {

        if self.tract_name() == report.tract_name {

            let num_senders = self.num_fibers();
            let num_fibers = report.num_fibers;

            if num_senders == num_fibers {
                self.set_target_address(report.address).await?;
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

}

