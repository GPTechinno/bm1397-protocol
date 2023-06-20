#![no_std]
//! BM1397 protocol driver.

pub mod command;
mod crc;
pub mod register;
pub mod response;
pub mod specifier;

// pub use command::{Command, Destination, Midstate};
// pub use register::{
//     ChipAddress, ClockOrderControl0, ClockOrderControl1, FastUARTConfiguration, MiscControl,
//     PLL0Parameter, PLL1Parameter, PLL2Parameter, PLL3Parameter, Register, Registers,
// };
// pub use response::{JobResponse, RegisterResponse, Response, ResponseType};
// pub use specifier::{BaudrateClockSelect, ClockSelect};

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidPreamble,
    InvalidCrc,
    UnknownRegister(u8),
}
