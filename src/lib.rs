#![no_std]
//! BM1397 protocol driver.

pub mod address;
pub mod command;
mod crc;
pub mod response;

pub use address::{CoreRegister, Register};
pub use command::{Command, Destination, Midstate};
pub use response::{JobResponse, RegisterResponse, Response, ResponseType};

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidPreamble,
    InvalidCrc,
    UnknownRegister(u8),
}
