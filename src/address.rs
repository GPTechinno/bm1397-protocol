/// Registers of the BM1397 asic.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Register {
    ChipAddress = 0x00,
    HashRate = 0x04,
    PLL0Parameter = 0x08,
    ChipNonceOffset = 0x0C,
    HashCountingNumber = 0x10,
    TicketMask = 0x14,
    MiscControl = 0x18,
    I2CControl = 0x1C,
    OrderedClockEnable = 0x20,
    FastUARTConfiguration = 0x28,
    UARTRelay = 0x2C,
    TicketMask2 = 0x38,
    CoreRegisterControl = 0x3C,
    CoreRegisterValue = 0x40,
    ExternalTemperatureSensorRead = 0x44,
    ErrorFlag = 0x48,
    NonceErrorCounter = 0x4C,
    NonceOverflowCounter = 0x50,
    AnalogMuxControl = 0x54,
    IoDriverStrenghtConfiguration = 0x58,
    TimeOut = 0x5C,
    PLL1Parameter = 0x60,
    PLL2Parameter = 0x64,
    PLL3Parameter = 0x68,
    OrderedClockMonitor = 0x6C,
    Pll0Divider = 0x70,
    Pll1Divider = 0x74,
    Pll2Divider = 0x78,
    Pll3Divider = 0x7C,
    ClockOrderControl0 = 0x80,
    ClockOrderControl1 = 0x84,
    ClockOrderStatus = 0x8C,
    FrequencySweepControl1 = 0x90,
    GoldenNonceForSweepReturn = 0x94,
    ReturnedGroupPatternStatus = 0x98,
    NonceReturnedTimeout = 0x9C,
    ReturnedSinglePatternStatus = 0xA0,
}

impl From<Register> for u8 {
    fn from(r: Register) -> u8 {
        r as u8
    }
}

impl TryFrom<u8> for Register {
    type Error = u8;
    fn try_from(val: u8) -> Result<Register, u8> {
        match val {
            x if x == Register::ChipAddress as u8 => Ok(Register::ChipAddress),
            x if x == Register::HashRate as u8 => Ok(Register::HashRate),
            x if x == Register::PLL0Parameter as u8 => Ok(Register::PLL0Parameter),
            x if x == Register::ChipNonceOffset as u8 => Ok(Register::ChipNonceOffset),
            x if x == Register::HashCountingNumber as u8 => Ok(Register::HashCountingNumber),
            x if x == Register::TicketMask as u8 => Ok(Register::TicketMask),
            x if x == Register::MiscControl as u8 => Ok(Register::MiscControl),
            x if x == Register::I2CControl as u8 => Ok(Register::I2CControl),
            x if x == Register::OrderedClockEnable as u8 => Ok(Register::OrderedClockEnable),
            x if x == Register::FastUARTConfiguration as u8 => Ok(Register::FastUARTConfiguration),
            x if x == Register::UARTRelay as u8 => Ok(Register::UARTRelay),
            x if x == Register::TicketMask2 as u8 => Ok(Register::TicketMask2),
            x if x == Register::CoreRegisterControl as u8 => Ok(Register::CoreRegisterControl),
            x if x == Register::CoreRegisterValue as u8 => Ok(Register::CoreRegisterValue),
            x if x == Register::ExternalTemperatureSensorRead as u8 => {
                Ok(Register::ExternalTemperatureSensorRead)
            }
            x if x == Register::ErrorFlag as u8 => Ok(Register::ErrorFlag),
            x if x == Register::NonceErrorCounter as u8 => Ok(Register::NonceErrorCounter),
            x if x == Register::NonceOverflowCounter as u8 => Ok(Register::NonceOverflowCounter),
            x if x == Register::AnalogMuxControl as u8 => Ok(Register::AnalogMuxControl),
            x if x == Register::IoDriverStrenghtConfiguration as u8 => {
                Ok(Register::IoDriverStrenghtConfiguration)
            }
            x if x == Register::TimeOut as u8 => Ok(Register::TimeOut),
            x if x == Register::PLL1Parameter as u8 => Ok(Register::PLL1Parameter),
            x if x == Register::PLL2Parameter as u8 => Ok(Register::PLL2Parameter),
            x if x == Register::PLL3Parameter as u8 => Ok(Register::PLL3Parameter),
            x if x == Register::OrderedClockMonitor as u8 => Ok(Register::OrderedClockMonitor),
            x if x == Register::Pll0Divider as u8 => Ok(Register::Pll0Divider),
            x if x == Register::Pll1Divider as u8 => Ok(Register::Pll1Divider),
            x if x == Register::Pll2Divider as u8 => Ok(Register::Pll2Divider),
            x if x == Register::Pll3Divider as u8 => Ok(Register::Pll3Divider),
            x if x == Register::ClockOrderControl0 as u8 => Ok(Register::ClockOrderControl0),
            x if x == Register::ClockOrderControl1 as u8 => Ok(Register::ClockOrderControl1),
            x if x == Register::ClockOrderStatus as u8 => Ok(Register::ClockOrderStatus),
            x if x == Register::FrequencySweepControl1 as u8 => {
                Ok(Register::FrequencySweepControl1)
            }
            x if x == Register::GoldenNonceForSweepReturn as u8 => {
                Ok(Register::GoldenNonceForSweepReturn)
            }
            x if x == Register::ReturnedGroupPatternStatus as u8 => {
                Ok(Register::ReturnedGroupPatternStatus)
            }
            x if x == Register::NonceReturnedTimeout as u8 => Ok(Register::NonceReturnedTimeout),
            x if x == Register::ReturnedSinglePatternStatus as u8 => {
                Ok(Register::ReturnedSinglePatternStatus)
            }
            _ => Err(val),
        }
    }
}

/// Core Registers of the BM1397 asic.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CoreRegister {
    ClockDelayCtrl = 0,
    ProcessMonitorCtrl = 1,
    ProcessMonitorData = 2,
    CoreError = 3,
    CoreEnable = 4,
    HashClockCtrl = 5,
    HashClockCounter = 6,
    SweepClockCtrl = 7,
}

impl From<CoreRegister> for u8 {
    fn from(r: CoreRegister) -> u8 {
        r as u8
    }
}

impl TryFrom<u8> for CoreRegister {
    type Error = u8;
    fn try_from(val: u8) -> Result<CoreRegister, u8> {
        match val {
            x if x == CoreRegister::ClockDelayCtrl as u8 => Ok(CoreRegister::ClockDelayCtrl),
            x if x == CoreRegister::ProcessMonitorCtrl as u8 => {
                Ok(CoreRegister::ProcessMonitorCtrl)
            }
            x if x == CoreRegister::ProcessMonitorData as u8 => {
                Ok(CoreRegister::ProcessMonitorData)
            }
            x if x == CoreRegister::CoreError as u8 => Ok(CoreRegister::CoreError),
            x if x == CoreRegister::CoreEnable as u8 => Ok(CoreRegister::CoreEnable),
            x if x == CoreRegister::HashClockCtrl as u8 => Ok(CoreRegister::HashClockCtrl),
            x if x == CoreRegister::HashClockCounter as u8 => Ok(CoreRegister::HashClockCounter),
            x if x == CoreRegister::SweepClockCtrl as u8 => Ok(CoreRegister::SweepClockCtrl),
            _ => Err(val),
        }
    }
}
