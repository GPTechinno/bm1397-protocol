#![no_std]
//! BM1397 protocol driver.

pub mod command;
mod crc;
pub mod register;
pub mod response;
pub mod specifier;

// pub use command::{Command, Destination, Midstate};
// pub use register::{
//     AnalogMuxControl, ChipAddress, ChipNonceOffset, ClockOrderControl0, ClockOrderControl1,
//     ClockOrderStatus, CoreRegisterControl, CoreRegisterValue, ErrorFlag,
//     ExternalTemperatureSensorRead, FastUARTConfiguration, FrequencySweepControl1,
//     GoldenNonceForSweepReturn, HashCountingNumber, HashRate, I2CControl,
//     IoDriverStrenghtConfiguration, MiscControl, NonceErrorCounter, NonceOverflowCounter,
//     NonceReturnedTimeout, OrderedClockEnable, OrderedClockMonitor, PLL0Divider, PLL0Parameter,
//     PLL1Divider, PLL1Parameter, PLL2Divider, PLL2Parameter, PLL3Divider, PLL3Parameter, Register,
//     Registers, ReturnedGroupPatternStatus, ReturnedSinglePatternStatus, TicketMask, TicketMask2,
//     TimeOut, UARTRelay,
// };
// pub use response::{JobResponse, RegisterResponse, Response, ResponseType};
// pub use specifier::{BaudrateClockSelect, ClockSelect};

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidPreamble,
    InvalidCrc,
    UnknownRegister(u8),
}
