#![no_std]
//! BM1397 protocol driver.

pub mod command;
mod crc;
pub mod register;
pub mod response;
pub mod specifier;

// pub use command::{Command, Destination, Midstate};
// pub use register::{
//     ChipAddress, ClockOrderControl0, ClockOrderControl1, PLL0Parameter, RegAddress, Registers,
// };
// pub use response::{JobResponse, RegisterResponse, Response, ResponseType};
// pub use specifier::ClockSelect;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidPreamble,
    InvalidCrc,
    UnknownRegister(u8),
}
