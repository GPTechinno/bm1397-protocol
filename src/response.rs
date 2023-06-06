use crate::crc::crc5;
use crate::registers::Register;
use crate::Error;
use byteorder::{BigEndian, ByteOrder};

#[derive(Debug)]
pub struct RegisterResponse {
    pub value: u32,
    pub chip_addr: u8,
    pub register: Register,
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
    /// - `Err(Error::UnknownRegister(u8))` with the register address if it do not match a known `Register`.
    /// - `Ok(ResponseType::Job(j))` with the `JobResponse`.
    ///
    /// ## Example
    ///
    /// ```
    /// use bm1397_protocol::{Error, Register, Response, ResponseType};
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
    /// // Register::ChipAddress == 0x13971800
    /// let resp = Response::parse(&[0xAA,0x55,0x13,0x97,0x18,0x00,0x00,0x00,0x06]);
    /// assert!(resp.is_ok());
    /// match resp.unwrap() {
    ///     ResponseType::Reg(r) => {
    ///         assert_eq!(r.value, 0x13971800);
    ///         assert_eq!(r.chip_addr, 0);
    ///         assert_eq!(r.register, Register::ChipAddress);
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
        if data[8] & 0x80 == 0x80 {
            return Ok(ResponseType::Job(JobResponse {
                nonce: BigEndian::read_u32(&data[2..]),
                midstate_id: data[6],
                job_id: data[7],
            }));
        }
        match Register::try_from(data[7]) {
            Err(e) => Err(Error::UnknownRegister(e)),
            Ok(r) => Ok(ResponseType::Reg(RegisterResponse {
                value: BigEndian::read_u32(&data[2..]),
                chip_addr: data[6],
                register: r,
            })),
        }
    }
}
