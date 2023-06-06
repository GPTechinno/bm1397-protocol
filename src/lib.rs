#![no_std]
//! BM1397 protocol driver.

pub mod command;
mod crc;
pub mod registers;
pub mod response;

pub use command::*;
pub use registers::*;
pub use response::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidPreamble,
    InvalidCrc,
    UnknownRegister(u8),
}
