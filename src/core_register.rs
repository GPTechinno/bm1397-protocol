//! BM1397 Core Registers.

pub trait CoreRegister {
    fn id(&self) -> u8;
    fn val(&self) -> u8;
}

macro_rules! impl_boilerplate_for {
    ($REG:ident) => {
        impl From<u8> for $REG {
            fn from(val: u8) -> Self {
                Self(val)
            }
        }

        impl From<$REG> for u8 {
            fn from(val: $REG) -> u8 {
                val.0
            }
        }

        impl Default for $REG {
            fn default() -> Self {
                Self::DEFAULT
            }
        }

        impl CoreRegister for $REG {
            fn id(&self) -> u8 {
                Self::ID
            }
            fn val(&self) -> u8 {
                self.0
            }
        }
    };
}

/// # Clock Delay Ctrl core register
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ClockDelayCtrl(u8);
impl_boilerplate_for!(ClockDelayCtrl);

impl ClockDelayCtrl {
    /// ## Clock Delay Ctrl core register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::core_register::{ClockDelayCtrl, CoreRegister};
    ///
    /// assert_eq!(ClockDelayCtrl::ID, ClockDelayCtrl::DEFAULT.id());
    /// ```
    pub const ID: u8 = 0x00;

    /// ## Clock Delay Ctrl core register reset value.
    pub const RESET: u8 = 0x00;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::core_register::ClockDelayCtrl;
    ///
    /// assert_eq!(ClockDelayCtrl::DEFAULT, ClockDelayCtrl::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `CCDLY_SEL` field.
    pub const CCDLY_SEL_OFFSET: u8 = 6;
    /// ## Bit offset for the `PWTH_SEL` field.
    pub const PWTH_SEL_OFFSET: u8 = 4;
    /// ## Bit offset for the `HASH_CLKEN` field.
    pub const HASH_CLKEN_OFFSET: u8 = 3;
    /// ## Bit offset for the `MMEN` field.
    pub const MMEN_OFFSET: u8 = 2;
    /// ## Bit offset for the `SWPF_MODE` field.
    pub const SWPF_MODE_OFFSET: u8 = 0;

    /// ## Bit mask for the `CCDLY_SEL` field.
    pub const CCDLY_SEL_MASK: u8 = 0b11 << Self::CCDLY_SEL_OFFSET;
    /// ## Bit mask for the `PWTH_SEL` field.
    pub const PWTH_SEL_MASK: u8 = 0b11 << Self::PWTH_SEL_OFFSET;
    /// ## Bit mask for the `HASH_CLKEN` field.
    pub const HASH_CLKEN_MASK: u8 = 0b1 << Self::HASH_CLKEN_OFFSET;
    /// ## Bit mask for the `MMEN` field.
    pub const MMEN_MASK: u8 = 0b1 << Self::MMEN_OFFSET;
    /// ## Bit mask for the `SWPF_MODE` field.
    pub const SWPF_MODE_MASK: u8 = 0b1 << Self::SWPF_MODE_OFFSET;

    /// ## Get the Multi Midstate state.
    ///
    /// This returns an `bool` with the Multi Midstate state.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::core_register::ClockDelayCtrl;
    ///
    /// let cdc: ClockDelayCtrl = ClockDelayCtrl::DEFAULT;
    /// assert!(!cdc.mm_enabled());
    /// ```
    pub const fn mm_enabled(&self) -> bool {
        self.0 & Self::MMEN_MASK == Self::MMEN_MASK
    }
    /// ## Enable the Multi Midstate mode (AsicBoost).
    #[must_use = "enable_mm returns a modified ClockDelayCtrl"]
    pub const fn enable_mm(mut self) -> Self {
        self.0 |= Self::MMEN_MASK;
        self
    }
    /// ## Disable the Multi Midstate mode (AsicBoost).
    #[must_use = "disable_mm returns a modified ClockDelayCtrl"]
    pub const fn disable_mm(mut self) -> Self {
        self.0 &= !Self::MMEN_MASK;
        self
    }
}

impl ::core::fmt::Display for ClockDelayCtrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ClockDelayCtrl")
            .field("mm_enabled", &self.mm_enabled())
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ClockDelayCtrl {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ClockDelayCtrl {{ mm_enabled: {} }}",
            self.mm_enabled(),
        );
    }
}

/// # Process Monitor Ctrl core register
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ProcessMonitorCtrl(u8);
impl_boilerplate_for!(ProcessMonitorCtrl);

impl ProcessMonitorCtrl {
    /// ## Process Monitor Ctrl core register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::core_register::{ProcessMonitorCtrl, CoreRegister};
    ///
    /// assert_eq!(ProcessMonitorCtrl::ID, ProcessMonitorCtrl::DEFAULT.id());
    /// ```
    pub const ID: u8 = 0x01;

    /// ## Process Monitor Ctrl core register reset value.
    pub const RESET: u8 = 0x00;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::core_register::ProcessMonitorCtrl;
    ///
    /// assert_eq!(ProcessMonitorCtrl::DEFAULT, ProcessMonitorCtrl::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `PM_START` field.
    pub const PM_START_OFFSET: u8 = 2;
    /// ## Bit offset for the `PM_SEL` field.
    pub const PM_SEL_OFFSET: u8 = 0;

    /// ## Bit mask for the `PM_START` field.
    pub const PM_START_MASK: u8 = 0b1 << Self::PM_START_OFFSET;
    /// ## Bit mask for the `PM_SEL` field.
    pub const PM_SEL_MASK: u8 = 0b11 << Self::PM_SEL_OFFSET;
}

impl ::core::fmt::Display for ProcessMonitorCtrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ProcessMonitorCtrl").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ProcessMonitorCtrl {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ProcessMonitorCtrl {{ }}",);
    }
}

/// # Process Monitor Data core register
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ProcessMonitorData(u8);
impl_boilerplate_for!(ProcessMonitorData);

impl ProcessMonitorData {
    /// ## Process Monitor Data core register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::core_register::{ProcessMonitorData, CoreRegister};
    ///
    /// assert_eq!(ProcessMonitorData::ID, ProcessMonitorData::DEFAULT.id());
    /// ```
    pub const ID: u8 = 0x02;

    /// ## Process Monitor Data core register reset value.
    pub const RESET: u8 = 0x00;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::core_register::ProcessMonitorData;
    ///
    /// assert_eq!(ProcessMonitorData::DEFAULT, ProcessMonitorData::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `DATA` field.
    pub const DATA_OFFSET: u8 = 0;

    /// ## Bit mask for the `DATA` field.
    pub const DATA_MASK: u8 = 0xff << Self::DATA_OFFSET;
}

impl ::core::fmt::Display for ProcessMonitorData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ProcessMonitorData").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ProcessMonitorData {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ProcessMonitorData {{ }}",);
    }
}

/// # Core Error core register
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CoreError(u8);
impl_boilerplate_for!(CoreError);

impl CoreError {
    /// ## Core Error core register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::core_register::{CoreError, CoreRegister};
    ///
    /// assert_eq!(CoreError::ID, CoreError::DEFAULT.id());
    /// ```
    pub const ID: u8 = 0x03;

    /// ## Core Error core register reset value.
    pub const RESET: u8 = 0x00;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::core_register::CoreError;
    ///
    /// assert_eq!(CoreError::DEFAULT, CoreError::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `INI_NONCE_ERR` field.
    pub const INI_NONCE_ERR_OFFSET: u8 = 4;
    /// ## Bit offset for the `CMD_ERR_CNT` field.
    pub const CMD_ERR_CNT_OFFSET: u8 = 0;

    /// ## Bit mask for the `INI_NONCE_ERR` field.
    pub const INI_NONCE_ERR_MASK: u8 = 0b1 << Self::INI_NONCE_ERR_OFFSET;
    /// ## Bit mask for the `CMD_ERR_CNT` field.
    pub const CMD_ERR_CNT_MASK: u8 = 0b1111 << Self::CMD_ERR_CNT_OFFSET;
}

impl ::core::fmt::Display for CoreError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CoreError").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for CoreError {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "CoreError {{ }}",);
    }
}

/// # Core Enable core register
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CoreEnable(u8);
impl_boilerplate_for!(CoreEnable);

impl CoreEnable {
    /// ## Core Enable core register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::core_register::{CoreEnable, CoreRegister};
    ///
    /// assert_eq!(CoreEnable::ID, CoreEnable::DEFAULT.id());
    /// ```
    pub const ID: u8 = 0x04;

    /// ## Core Enable core register reset value.
    pub const RESET: u8 = 0x00;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::core_register::CoreEnable;
    ///
    /// assert_eq!(CoreEnable::DEFAULT, CoreEnable::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `CORE_EN_I` field.
    pub const CORE_EN_I_OFFSET: u8 = 0;

    /// ## Bit mask for the `CORE_EN_I` field.
    pub const CORE_EN_I_MASK: u8 = 0xff << Self::CORE_EN_I_OFFSET;
}

impl ::core::fmt::Display for CoreEnable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CoreEnable").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for CoreEnable {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "CoreEnable {{ }}",);
    }
}

/// # Hash Clock Ctrl core register
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct HashClockCtrl(u8);
impl_boilerplate_for!(HashClockCtrl);

impl HashClockCtrl {
    /// ## Hash Clock Ctrl core register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::core_register::{HashClockCtrl, CoreRegister};
    ///
    /// assert_eq!(HashClockCtrl::ID, HashClockCtrl::DEFAULT.id());
    /// ```
    pub const ID: u8 = 0x05;

    /// ## Hash Clock Ctrl core register reset value.
    pub const RESET: u8 = 0x00;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::core_register::HashClockCtrl;
    ///
    /// assert_eq!(HashClockCtrl::DEFAULT, HashClockCtrl::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `CLOCK_CTRL` field.
    pub const CLOCK_CTRL_OFFSET: u8 = 0;

    /// ## Bit mask for the `CLOCK_CTRL` field.
    pub const CLOCK_CTRL_MASK: u8 = 0xff << Self::CLOCK_CTRL_OFFSET;
}

impl ::core::fmt::Display for HashClockCtrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HashClockCtrl").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for HashClockCtrl {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "HashClockCtrl {{ }}",);
    }
}

/// # Hash Clock Counter core register
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct HashClockCounter(u8);
impl_boilerplate_for!(HashClockCounter);

impl HashClockCounter {
    /// ## Hash Clock Counter core register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::core_register::{HashClockCounter, CoreRegister};
    ///
    /// assert_eq!(HashClockCounter::ID, HashClockCounter::DEFAULT.id());
    /// ```
    pub const ID: u8 = 0x06;

    /// ## Hash Clock Counter core register reset value.
    pub const RESET: u8 = 0x00;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::core_register::HashClockCounter;
    ///
    /// assert_eq!(HashClockCounter::DEFAULT, HashClockCounter::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `CLOCK_CNT` field.
    pub const CLOCK_CNT_OFFSET: u8 = 0;

    /// ## Bit mask for the `CLOCK_CNT` field.
    pub const CLOCK_CNT_MASK: u8 = 0xff << Self::CLOCK_CNT_OFFSET;
}

impl ::core::fmt::Display for HashClockCounter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HashClockCounter").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for HashClockCounter {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "HashClockCounter {{ }}",);
    }
}

/// # Sweep Clock Ctrl core register
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SweepClockCtrl(u8);
impl_boilerplate_for!(SweepClockCtrl);

impl SweepClockCtrl {
    /// ## Sweep Clock Ctrl core register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::core_register::{SweepClockCtrl, CoreRegister};
    ///
    /// assert_eq!(SweepClockCtrl::ID, SweepClockCtrl::DEFAULT.id());
    /// ```
    pub const ID: u8 = 0x07;

    /// ## Sweep Clock Ctrl core register reset value.
    pub const RESET: u8 = 0x00;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::core_register::SweepClockCtrl;
    ///
    /// assert_eq!(SweepClockCtrl::DEFAULT, SweepClockCtrl::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `SWPF_MODE` field.
    pub const SWPF_MODE_OFFSET: u8 = 7;
    /// ## Bit offset for the `CLK_SEL` field.
    pub const CLK_SEL_OFFSET: u8 = 0;

    /// ## Bit mask for the `SWPF_MODE` field.
    pub const SWPF_MODE_MASK: u8 = 0b1 << Self::SWPF_MODE_OFFSET;
    /// ## Bit mask for the `CLK_SEL` field.
    pub const CLK_SEL_MASK: u8 = 0b1111 << Self::CLK_SEL_OFFSET;
}

impl ::core::fmt::Display for SweepClockCtrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SweepClockCtrl").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for SweepClockCtrl {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "SweepClockCtrl {{ }}",);
    }
}

#[derive(Debug, PartialEq)]
pub enum CoreRegisters {
    ClockDelayCtrl(ClockDelayCtrl),
    ProcessMonitorCtrl(ProcessMonitorCtrl),
    ProcessMonitorData(ProcessMonitorData),
    CoreError(CoreError),
    CoreEnable(CoreEnable),
    HashClockCtrl(HashClockCtrl),
    HashClockCounter(HashClockCounter),
    SweepClockCtrl(SweepClockCtrl),
}
