use packet::ProtocolId;

pub mod client;
pub mod server;
pub mod packet;
mod unit_tests;

/// default port the application runs
pub const DEFAULT_PORT: u16 = 30000;

/// Random protocol id for packets
pub const PROTOCOL_ID: ProtocolId = 8106;
