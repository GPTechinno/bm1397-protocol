#![no_std]
//! BM1397 protocol driver.

mod crc;

pub mod command;
pub mod core_register;
pub mod register;
pub mod response;
pub mod specifier;

// pub use core_register::{
//     ClockDelayCtrl, CoreEnable, CoreError, HashClockCounter, HashClockCtrl, ProcessMonitorCtrl,
//     ProcessMonitorData, SweepClockCtrl,
// };
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
// pub use specifier::{BaudrateClockSelect, ClockSelect, ProcessMonitorSelect};

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidPreamble,
    InvalidCrc,
    UnknownRegister(u8),
    UnknownCoreRegister(u8),
}
