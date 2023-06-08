//! BM1397 Registers structures.

use crate::specifier::ClockSelect;

pub trait RegAddress {
    fn addr(self) -> u8;
}

macro_rules! impl_boilerplate_for {
    ($REG:ident) => {
        impl From<u32> for $REG {
            fn from(val: u32) -> Self {
                Self(val)
            }
        }

        impl From<$REG> for u32 {
            fn from(val: $REG) -> u32 {
                val.0
            }
        }

        impl Default for $REG {
            fn default() -> Self {
                Self::DEFAULT
            }
        }

        impl RegAddress for $REG {
            fn addr(self) -> u8 {
                Self::ADDR
            }
        }
    };
}

/// Chip Address register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ChipAddress(u32);
impl_boilerplate_for!(ChipAddress);

impl ChipAddress {
    /// ## Chip Address register address.
    ///
    /// ## Example
    ///
    /// ```
    /// use bm1397_protocol::register::{ChipAddress, RegAddress};
    ///
    /// assert_eq!(ChipAddress::ADDR, ChipAddress::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x00;

    /// ## Chip Address register reset value.
    pub const RESET: u32 = 0x1397_1800;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ## Example
    ///
    /// ```
    /// use bm1397_protocol::register::ChipAddress;
    ///
    /// assert_eq!(ChipAddress::DEFAULT, ChipAddress::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// Bit offset for the `CHIP_ID` field.
    pub const CHIP_ID_OFFSET: u8 = 16;
    /// Bit offset for the `CORE_NUM` field.
    pub const CORE_NUM_OFFSET: u8 = 8;
    /// Bit offset for the `ADDR` field.
    pub const ADDR_OFFSET: u8 = 0;

    /// Bit mask for the `CHIP_ID` field.
    pub const CHIP_ID_MASK: u32 = 0xffff << Self::CHIP_ID_OFFSET;
    /// Bit mask for the `CORE_NUM` field.
    pub const CORE_NUM_MASK: u32 = 0xff << Self::CORE_NUM_OFFSET;
    /// Bit mask for the `ADDR` field.
    pub const ADDR_MASK: u32 = 0xff << Self::ADDR_OFFSET;

    /// # Get the chip identifier.
    ///
    /// This returns an `u16` with the chip_id value.
    ///
    /// ## Example
    ///
    /// ```
    /// use bm1397_protocol::register::ChipAddress;
    ///
    /// assert_eq!(ChipAddress::DEFAULT.chip_id(), 0x1397);
    /// ```
    pub const fn chip_id(&self) -> u16 {
        (self.0 >> Self::CHIP_ID_OFFSET) as u16
    }

    /// # Get the number of internal cores.
    ///
    /// This returns an `u8` with the core_num value.
    ///
    /// ## Example
    ///
    /// ```
    /// use bm1397_protocol::register::ChipAddress;
    ///
    /// assert_eq!(ChipAddress::DEFAULT.core_num(), 0x18);
    /// ```
    pub const fn core_num(&self) -> u8 {
        (self.0 >> Self::CORE_NUM_OFFSET) as u8
    }

    /// # Get the chip address on the chain.
    ///
    /// This returns an `u8` with the address value.
    ///
    /// ## Example
    ///
    /// ```
    /// use bm1397_protocol::register::ChipAddress;
    ///
    /// assert_eq!(ChipAddress::DEFAULT.chip_addr(), 0x00);
    /// ```
    pub const fn chip_addr(&self) -> u8 {
        (self.0 >> Self::ADDR_OFFSET) as u8
    }
}

impl ::core::fmt::Display for ChipAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ChipAddress")
            .field("chip_id", &self.chip_id())
            .field("core_num", &self.core_num())
            .field("chip_addr", &self.chip_addr())
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChipAddress {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ChipAddress {{ chip_id: {}, core_num: {}, chip_addr: {} }}",
            self.chip_id(),
            self.core_num(),
            self.chip_addr(),
        );
    }
}

/// Clock Order Control 0 register
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ClockOrderControl0(u32);
impl_boilerplate_for!(ClockOrderControl0);

impl ClockOrderControl0 {
    /// ## Clock Order Control 0 register address.
    ///
    /// ## Example
    ///
    /// ```
    /// use bm1397_protocol::register::{ClockOrderControl0, RegAddress};
    ///
    /// assert_eq!(ClockOrderControl0::ADDR, ClockOrderControl0::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x80;

    /// Reset value of the socket mode register.
    pub const RESET: u32 = 0x0000_0000;

    /// # Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ## Example
    ///
    /// ```
    /// use bm1397_protocol::register::ClockOrderControl0;
    ///
    /// assert_eq!(ClockOrderControl0::DEFAULT, ClockOrderControl0::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// Bit length for a `CLKN_SEL` field.
    pub const CLKN_SEL_LENGTH: u8 = 4;

    /// Bit mask for a `CLKN_SEL` field.
    pub const CLKN_SEL_MASK: u32 = 0xF;

    /// # Get the clock select.
    ///
    /// This returns an `Err(u8)` with the clock select bits if the clock select bits
    /// do not match a valid clock select.
    ///
    /// ## Example
    ///
    /// ```
    /// use bm1397_protocol::{specifier::ClockSelect, register::ClockOrderControl0};
    ///
    /// let clk_ord_ctrl: ClockOrderControl0 = ClockOrderControl0::DEFAULT;
    /// assert_eq!(clk_ord_ctrl.clock_select(0), Ok(ClockSelect::Default));
    /// ```
    pub const fn clock_select(&self, clock: u8) -> Result<ClockSelect, u8> {
        if clock > 7 {
            return Err(clock);
        }
        ClockSelect::from_raw(
            ((self.0 >> (clock * Self::CLKN_SEL_LENGTH)) & Self::CLKN_SEL_MASK) as u8,
        )
    }

    /// # Set the clock select.
    ///
    /// ## Example
    ///
    /// ```
    /// use bm1397_protocol::{specifier::ClockSelect, register::ClockOrderControl0};
    ///
    /// const CLK_ORD_CTRL: ClockOrderControl0 = ClockOrderControl0::DEFAULT.set_clock_select(1, ClockSelect::Default);
    /// assert_eq!(CLK_ORD_CTRL.clock_select(1), Ok(ClockSelect::Default));
    /// ```
    pub const fn set_clock_select(mut self, clock: u8, clock_select: ClockSelect) -> Self {
        if clock < 8 {
            self.0 = (self.0 & !(Self::CLKN_SEL_MASK << (clock * Self::CLKN_SEL_LENGTH)))
                | ((((clock_select as u8) & 0xF) as u32) << (clock * Self::CLKN_SEL_LENGTH));
        }
        self
    }
}

impl ::core::fmt::Display for ClockOrderControl0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ClockOrderControl0")
            .field("clock0_select", &self.clock_select(0))
            .field("clock1_select", &self.clock_select(1))
            .field("clock2_select", &self.clock_select(2))
            .field("clock3_select", &self.clock_select(3))
            .field("clock4_select", &self.clock_select(4))
            .field("clock5_select", &self.clock_select(5))
            .field("clock6_select", &self.clock_select(6))
            .field("clock7_select", &self.clock_select(7))
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ClockOrderControl0 {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ClockOrderControl0 {{ clock0_select: {}, clock1_select: {}, clock2_select: {}, clock3_select: {}, clock4_select: {}, clock5_select: {}, clock6_select: {}, clock7_select: {} }}",
            self.clock_select(0),
            self.clock_select(1),
            self.clock_select(2),
            self.clock_select(3),
            self.clock_select(4),
            self.clock_select(5),
            self.clock_select(6),
            self.clock_select(7),
        );
    }
}

/// Clock Order Control 1 register
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ClockOrderControl1(u32);
impl_boilerplate_for!(ClockOrderControl1);

impl ClockOrderControl1 {
    /// ## Clock Order Control 1 register address.
    ///
    /// ## Example
    ///
    /// ```
    /// use bm1397_protocol::register::{ClockOrderControl1, RegAddress};
    ///
    /// assert_eq!(ClockOrderControl1::ADDR, ClockOrderControl1::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x84;

    /// Reset value of the socket mode register.
    pub const RESET: u32 = 0x0000_0000;

    /// # Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ## Example
    ///
    /// ```
    /// use bm1397_protocol::register::ClockOrderControl1;
    ///
    /// assert_eq!(ClockOrderControl1::DEFAULT, ClockOrderControl1::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// Bit length for a `CLKN_SEL` field.
    pub const CLKN_SEL_LENGTH: u8 = 4;

    /// Bit mask for a `CLKN_SEL` field.
    pub const CLKN_SEL_MASK: u32 = 0xF;

    /// # Get the clock select.
    ///
    /// This returns an `Err(u8)` with the clock select bits if the clock select bits
    /// do not match a valid clock select.
    ///
    /// ## Example
    ///
    /// ```
    /// use bm1397_protocol::{specifier::ClockSelect, register::ClockOrderControl1};
    ///
    /// let clk_ord_ctrl: ClockOrderControl1 = ClockOrderControl1::DEFAULT;
    /// assert_eq!(clk_ord_ctrl.clock_select(0), Ok(ClockSelect::Default));
    /// ```
    pub const fn clock_select(&self, clock: u8) -> Result<ClockSelect, u8> {
        if clock > 7 {
            return Err(clock);
        }
        ClockSelect::from_raw(
            ((self.0 >> (clock * Self::CLKN_SEL_LENGTH)) & Self::CLKN_SEL_MASK) as u8,
        )
    }

    /// # Set the clock select.
    ///
    /// ## Example
    ///
    /// ```
    /// use bm1397_protocol::{specifier::ClockSelect, register::ClockOrderControl1};
    ///
    /// const CLK_ORD_CTRL: ClockOrderControl1 = ClockOrderControl1::DEFAULT.set_clock_select(1, ClockSelect::Default);
    /// assert_eq!(CLK_ORD_CTRL.clock_select(1), Ok(ClockSelect::Default));
    /// ```
    pub const fn set_clock_select(mut self, clock: u8, clock_select: ClockSelect) -> Self {
        if clock < 8 {
            self.0 = (self.0 & !(Self::CLKN_SEL_MASK << (clock * Self::CLKN_SEL_LENGTH)))
                | ((((clock_select as u8) & 0xF) as u32) << (clock * Self::CLKN_SEL_LENGTH));
        }
        self
    }
}

impl ::core::fmt::Display for ClockOrderControl1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ClockOrderControl1")
            .field("clock8_select", &self.clock_select(0))
            .field("clock9_select", &self.clock_select(1))
            .field("clock10_select", &self.clock_select(2))
            .field("clock11_select", &self.clock_select(3))
            .field("clock12_select", &self.clock_select(4))
            .field("clock13_select", &self.clock_select(5))
            .field("clock14_select", &self.clock_select(6))
            .field("clock15_select", &self.clock_select(7))
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ClockOrderControl1 {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ClockOrderControl1 {{ clock8_select: {}, clock9_select: {}, clock10_select: {}, clock11_select: {}, clock12_select: {}, clock13_select: {}, clock14_select: {}, clock15_select: {} }}",
            self.clock_select(0),
            self.clock_select(1),
            self.clock_select(2),
            self.clock_select(3),
            self.clock_select(4),
            self.clock_select(5),
            self.clock_select(6),
            self.clock_select(7),
        );
    }
}

#[derive(Debug, PartialEq)]
pub enum Registers {
    ChipAddress(ChipAddress),
    ClockOrderControl0(ClockOrderControl0),
    ClockOrderControl1(ClockOrderControl1),
}
