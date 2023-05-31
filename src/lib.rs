#![no_std]
//! BM1397 protocol driver.

mod crc;
pub mod registers;
pub mod response;

pub use registers::*;
pub use response::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidPreamble,
    InvalidCrc,
    UnknownRegister(u8),
}
