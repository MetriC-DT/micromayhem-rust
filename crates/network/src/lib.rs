pub mod client;
pub mod server;
pub mod message;
mod unit_tests;

/// default port the application runs
pub const DEFAULT_PORT: u16 = 30000;

/// Random protocol id for packets
pub const PROTOCOL_ID: u16 = 8106;
