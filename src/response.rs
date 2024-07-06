//! BM1397 Responses.

use crate::crc::crc5;
use crate::register::*;
use crate::Error;

#[derive(Debug)]
pub struct RegisterResponse {
    pub chip_addr: u8,
    pub register: Registers,
}

#[derive(Debug)]
pub struct JobResponse {
    pub nonce: u32,
    pub job_id: u8,
    pub midstate_id: u8,
}

#[derive(Debug)]
pub enum ResponseType {
    Reg(RegisterResponse),
    Job(JobResponse),
}

pub struct Response;

impl Response {
    /// # Parse Response
    ///
    /// Parse raw bytes from RO signal of BM1397.
    ///
    /// The packet must have a lenght of 9 bytes.
    ///
    /// ## Return
    /// - `Err(Error::InvalidPreamble)` if it first 2 bytes are not `[0xAA, 0x55]`.
    /// - `Err(Error::InvalidCrc)` if the CRC5 is not valid.
    /// - `Ok(ResponseType::Reg(r))` with the `RegisterResponse`.
    /// - `Err(Error::UnknownRegister(u8))` with the register address if it do not match a known `Registers`.
    /// - `Ok(ResponseType::Job(j))` with the `JobResponse`.
    ///
    /// ## Example
    ///
    /// ```
    /// use bm1397_protocol::Error;
    /// use bm1397_protocol::register::{Registers, ChipAddress};
    /// use bm1397_protocol::response::{Response, ResponseType};
    ///
    /// // Error::InvalidPreamble
    /// let resp = Response::parse(&[0x00,0x55,0x13,0x97,0x18,0x00,0x00,0x00,0x06]);
    /// assert!(resp.is_err());
    /// assert_eq!(resp.unwrap_err(), Error::InvalidPreamble);
    ///
    /// let resp = Response::parse(&[0xAA,0x00,0x13,0x97,0x18,0x00,0x00,0x00,0x06]);
    /// assert!(resp.is_err());
    /// assert_eq!(resp.unwrap_err(), Error::InvalidPreamble);
    ///
    /// let resp = Response::parse(&[0x00,0x00,0x13,0x97,0x18,0x00,0x00,0x00,0x06]);
    /// assert!(resp.is_err());
    /// assert_eq!(resp.unwrap_err(), Error::InvalidPreamble);
    ///
    /// // Error::InvalidCrc
    /// let resp = Response::parse(&[0xAA,0x55,0x13,0x97,0x18,0x00,0x00,0x00,0x00]);
    /// assert!(resp.is_err());
    /// assert_eq!(resp.unwrap_err(), Error::InvalidCrc);
    ///
    /// // ChipAddress == 0x13971800
    /// let resp = Response::parse(&[0xAA,0x55,0x13,0x97,0x18,0x00,0x00,0x00,0x06]);
    /// assert!(resp.is_ok());
    /// match resp.unwrap() {
    ///     ResponseType::Reg(r) => {
    ///         assert_eq!(r.chip_addr, 0);
    ///         assert_eq!(r.register, Registers::ChipAddress(ChipAddress::default()));
    ///     },
    ///     _ => panic!(),
    /// };
    ///
    /// // Error::UnknownRegister(0xF0)
    /// let resp = Response::parse(&[0xAA,0x55,0x00,0x00,0x00,0x00,0x04,0xF0,0x03]);
    /// assert!(resp.is_err());
    /// assert_eq!(resp.unwrap_err(), Error::UnknownRegister(0xF0));
    ///
    /// // Nonce == 0x97C328B6
    /// let resp = Response::parse(&[0xAA,0x55,0x97,0xC3,0x28,0xB6,0x01,0x63,0x9C]);
    /// assert!(resp.is_ok());
    /// match resp.unwrap() {
    ///     ResponseType::Job(j) => {
    ///         assert_eq!(j.nonce, 0x97C328B6);
    ///         assert_eq!(j.midstate_id, 1);
    ///         assert_eq!(j.job_id, 0x63);
    ///     },
    ///     _ => panic!(),
    /// };
    /// ```
    pub fn parse(data: &[u8; 9]) -> Result<ResponseType, Error> {
        if data[0] != 0xAA || data[1] != 0x55 {
            return Err(Error::InvalidPreamble);
        }
        if crc5(&data[2..9]) != 0x00 {
            return Err(Error::InvalidCrc);
        }
        let reg_val = u32::from_be_bytes(data[2..6].try_into().unwrap());
        if data[8] & 0x80 == 0x80 {
            return Ok(ResponseType::Job(JobResponse {
                nonce: reg_val,
                midstate_id: data[6],
                job_id: data[7],
            }));
        }
        Ok(ResponseType::Reg(RegisterResponse {
            chip_addr: data[6],
            register: match data[7] {
                ChipAddress::ADDR => Registers::ChipAddress(ChipAddress::from(reg_val)),
                HashRate::ADDR => Registers::HashRate(HashRate::from(reg_val)),
                PLL0Parameter::ADDR => Registers::PLL0Parameter(PLL0Parameter::from(reg_val)),
                ChipNonceOffset::ADDR => Registers::ChipNonceOffset(ChipNonceOffset::from(reg_val)),
                HashCountingNumber::ADDR => {
                    Registers::HashCountingNumber(HashCountingNumber::from(reg_val))
                }
                TicketMask::ADDR => Registers::TicketMask(TicketMask::from(reg_val)),
                MiscControl::ADDR => Registers::MiscControl(MiscControl::from(reg_val)),
                I2CControl::ADDR => Registers::I2CControl(I2CControl::from(reg_val)),
                OrderedClockEnable::ADDR => {
                    Registers::OrderedClockEnable(OrderedClockEnable::from(reg_val))
                }
                FastUARTConfiguration::ADDR => {
                    Registers::FastUARTConfiguration(FastUARTConfiguration::from(reg_val))
                }
                UARTRelay::ADDR => Registers::UARTRelay(UARTRelay::from(reg_val)),
                TicketMask2::ADDR => Registers::TicketMask2(TicketMask2::from(reg_val)),
                CoreRegisterControl::ADDR => {
                    Registers::CoreRegisterControl(CoreRegisterControl::from(reg_val))
                }
                CoreRegisterValue::ADDR => {
                    Registers::CoreRegisterValue(CoreRegisterValue::from(reg_val))
                }
                ExternalTemperatureSensorRead::ADDR => Registers::ExternalTemperatureSensorRead(
                    ExternalTemperatureSensorRead::from(reg_val),
                ),
                ErrorFlag::ADDR => Registers::ErrorFlag(ErrorFlag::from(reg_val)),
                NonceErrorCounter::ADDR => {
                    Registers::NonceErrorCounter(NonceErrorCounter::from(reg_val))
                }
                NonceOverflowCounter::ADDR => {
                    Registers::NonceOverflowCounter(NonceOverflowCounter::from(reg_val))
                }
                AnalogMuxControl::ADDR => {
                    Registers::AnalogMuxControl(AnalogMuxControl::from(reg_val))
                }
                IoDriverStrenghtConfiguration::ADDR => Registers::IoDriverStrenghtConfiguration(
                    IoDriverStrenghtConfiguration::from(reg_val),
                ),
                TimeOut::ADDR => Registers::TimeOut(TimeOut::from(reg_val)),
                PLL1Parameter::ADDR => Registers::PLL1Parameter(PLL1Parameter::from(reg_val)),
                PLL2Parameter::ADDR => Registers::PLL2Parameter(PLL2Parameter::from(reg_val)),
                PLL3Parameter::ADDR => Registers::PLL3Parameter(PLL3Parameter::from(reg_val)),
                OrderedClockMonitor::ADDR => {
                    Registers::OrderedClockMonitor(OrderedClockMonitor::from(reg_val))
                }
                PLL0Divider::ADDR => Registers::PLL0Divider(PLL0Divider::from(reg_val)),
                PLL1Divider::ADDR => Registers::PLL1Divider(PLL1Divider::from(reg_val)),
                PLL2Divider::ADDR => Registers::PLL2Divider(PLL2Divider::from(reg_val)),
                PLL3Divider::ADDR => Registers::PLL3Divider(PLL3Divider::from(reg_val)),
                ClockOrderControl0::ADDR => {
                    Registers::ClockOrderControl0(ClockOrderControl0::from(reg_val))
                }
                ClockOrderControl1::ADDR => {
                    Registers::ClockOrderControl1(ClockOrderControl1::from(reg_val))
                }
                ClockOrderStatus::ADDR => {
                    Registers::ClockOrderStatus(ClockOrderStatus::from(reg_val))
                }
                FrequencySweepControl1::ADDR => {
                    Registers::FrequencySweepControl1(FrequencySweepControl1::from(reg_val))
                }
                GoldenNonceForSweepReturn::ADDR => {
                    Registers::GoldenNonceForSweepReturn(GoldenNonceForSweepReturn::from(reg_val))
                }
                ReturnedGroupPatternStatus::ADDR => {
                    Registers::ReturnedGroupPatternStatus(ReturnedGroupPatternStatus::from(reg_val))
                }
                NonceReturnedTimeout::ADDR => {
                    Registers::NonceReturnedTimeout(NonceReturnedTimeout::from(reg_val))
                }
                ReturnedSinglePatternStatus::ADDR => Registers::ReturnedSinglePatternStatus(
                    ReturnedSinglePatternStatus::from(reg_val),
                ),
                addr => return Err(Error::UnknownRegister(addr)),
            },
        }))
    }
}
