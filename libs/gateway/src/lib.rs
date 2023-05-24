//! Main connlib library for gateway.
use control::ControlPlane;
use messages::EgressMessages;
use messages::IngressMessages;

mod control;
mod messages;

/// Session type for gateway.
///
/// For more information see libs_common docs on [Session][libs_common::Session].
pub type Session<C> = libs_common::Session<ControlPlane<C>, IngressMessages, EgressMessages>;

pub use libs_common::{error_type::ErrorType, Callbacks, Error, ResourceList, TunnelAddresses};
