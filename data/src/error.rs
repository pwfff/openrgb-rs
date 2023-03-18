use alloc::string::String;
use thiserror_no_std::Error;

/// Errors returned by [OpenRGB client](crate::OpenRGB).
#[derive(Error, Debug)]
pub enum OpenRGBError {
    /// Failed opening connection to OpenRGB server.
    ConnectionError(),

    /// Communication failure with OpenRGB server.
    CommunicationError(),

    /// Invalid encountered while communicating with OpenRGB server.
    ProtocolError(String),

    /// Server does not support operation.
    UnsupportedOperation {
        /// Protocol version in use by client.
        current_protocol_version: u32,

        /// Minimum required protocol version to use operation.
        min_protocol_version: u32,
    },

    BadPacketID(u32),

    BadMagic(u8),

    DeviceIDMismatch {
        expected: u32,
        got: u32,
    },

    PacketIDMismatch {
        expected: u32,
        got: u32,
    },
}
