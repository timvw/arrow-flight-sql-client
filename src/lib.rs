#[path = "arrow.flight.protocol.rs"]
pub mod arrow_flight_protocol;

#[path = "arrow.flight.protocol.sql.rs"]
pub mod arrow_flight_protocol_sql;

pub const FLIGHT_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("flight_descriptor");
pub const FLIGHT_SQL_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("flight_sql_descriptor");
