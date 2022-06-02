#[path = "arrow.flight.protocol.rs"]
pub mod arrow_flight_protocol;

#[path = "arrow.flight.protocol.sql.rs"]
pub mod arrow_flight_protocol_sql;

pub const FLIGHT_DESCRIPTOR_SET: &[u8] =
    include_bytes!("../file_descriptors/flight_descriptor.bin");
pub const FLIGHT_SQL_DESCRIPTOR_SET: &[u8] =
    include_bytes!("../file_descriptors/flight_sql_descriptor.bin");

pub mod client;
