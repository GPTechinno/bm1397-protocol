//! BM1397 Registers.

use crate::core_register::*;
use crate::specifier::{BaudrateClockSelect, ClockSelect};
use crate::Error;
use fugit::HertzU32;

pub trait Register {
    fn addr(&self) -> u8;
    fn val(&self) -> u32;
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

        impl Register for $REG {
            fn addr(&self) -> u8 {
                Self::ADDR
            }
            fn val(&self) -> u32 {
                self.0
            }
        }
    };
}

/// # Chip Address register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ChipAddress(u32);
impl_boilerplate_for!(ChipAddress);

impl ChipAddress {
    /// ## Chip Address register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{ChipAddress, Register};
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
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::ChipAddress;
    ///
    /// assert_eq!(ChipAddress::DEFAULT, ChipAddress::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `CHIP_ID` field.
    pub const CHIP_ID_OFFSET: u8 = 16;
    /// ## Bit offset for the `CORE_NUM` field.
    pub const CORE_NUM_OFFSET: u8 = 8;
    /// ## Bit offset for the `ADDR` field.
    pub const ADDR_OFFSET: u8 = 0;

    /// ## Bit mask for the `CHIP_ID` field.
    pub const CHIP_ID_MASK: u32 = 0xffff << Self::CHIP_ID_OFFSET;
    /// ## Bit mask for the `CORE_NUM` field.
    pub const CORE_NUM_MASK: u32 = 0xff << Self::CORE_NUM_OFFSET;
    /// ## Bit mask for the `ADDR` field.
    pub const ADDR_MASK: u32 = 0xff << Self::ADDR_OFFSET;

    /// ## Get the chip identifier.
    ///
    /// This returns an `u16` with the chip_id value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::ChipAddress;
    ///
    /// assert_eq!(ChipAddress::DEFAULT.chip_id(), 0x1397);
    /// ```
    pub const fn chip_id(&self) -> u16 {
        (self.0 >> Self::CHIP_ID_OFFSET) as u16
    }

    /// ## Get the number of internal cores.
    ///
    /// This returns an `u8` with the core_num value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::ChipAddress;
    ///
    /// assert_eq!(ChipAddress::DEFAULT.core_num(), 0x18);
    /// ```
    pub const fn core_num(&self) -> u8 {
        (self.0 >> Self::CORE_NUM_OFFSET) as u8
    }

    /// ## Get the chip address on the chain.
    ///
    /// This returns an `u8` with the address value.
    ///
    /// ### Example
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

/// # Hash Rate register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct HashRate(u32);
impl_boilerplate_for!(HashRate);

impl HashRate {
    /// ## Hash Rate register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{HashRate, Register};
    ///
    /// assert_eq!(HashRate::ADDR, HashRate::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x04;

    /// ## Hash Rate register reset value.
    pub const RESET: u32 = 0x8000_0000;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::HashRate;
    ///
    /// assert_eq!(HashRate::DEFAULT, HashRate::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `LONG` field.
    pub const LONG_OFFSET: u8 = 31;
    /// ## Bit offset for the `HASHRATE` field.
    pub const HASHRATE_OFFSET: u8 = 0;

    /// ## Bit mask for the `LONG` field.
    pub const LONG_MASK: u32 = 0b1 << Self::LONG_OFFSET;
    /// ## Bit mask for the `HASHRATE` field.
    pub const HASHRATE_MASK: u32 = 0x7fff_ffff << Self::HASHRATE_OFFSET;
}

impl ::core::fmt::Display for HashRate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HashRate").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for HashRate {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "HashRate {{  }}",);
    }
}

/// # PLL0 Parameter register
///
/// Used to set PLL0 frequency.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PLL0Parameter(u32);
impl_boilerplate_for!(PLL0Parameter);

impl PLL0Parameter {
    /// ## PLL0 Parameter register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{PLL0Parameter, Register};
    ///
    /// assert_eq!(PLL0Parameter::ADDR, PLL0Parameter::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x08;

    /// ## PLL0 Parameter register reset value.
    pub const RESET: u32 = 0xC060_0161;

    /// ### Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL0Parameter;
    ///
    /// assert_eq!(PLL0Parameter::DEFAULT, PLL0Parameter::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `LOCKED` field.
    pub const LOCKED_OFFSET: u8 = 31;
    /// ## Bit offset for the `PLLEN` field.
    pub const PLLEN_OFFSET: u8 = 30;
    /// ## Bit offset for the `FBDIV` field.
    pub const FBDIV_OFFSET: u8 = 16;
    /// ## Bit offset for the `REFDIV` field.
    pub const REFDIV_OFFSET: u8 = 8;
    /// ## Bit offset for the `POSTDIV1` field.
    pub const POSTDIV1_OFFSET: u8 = 4;
    /// ## Bit offset for the `POSTDIV2` field.
    pub const POSTDIV2_OFFSET: u8 = 0;

    /// ## Bit mask for the `LOCKED` field.
    pub const LOCKED_MASK: u32 = 0x1 << Self::LOCKED_OFFSET;
    /// ## Bit mask for the `PLLEN` field.
    pub const PLLEN_MASK: u32 = 0x1 << Self::PLLEN_OFFSET;
    /// ## Bit mask for the `FBDIV` field.
    pub const FBDIV_MASK: u32 = 0xfff << Self::FBDIV_OFFSET;
    /// ## Bit mask for the `REFDIV` field.
    pub const REFDIV_MASK: u32 = 0x3f << Self::REFDIV_OFFSET;
    /// ## Bit mask for the `POSTDIV1` field.
    pub const POSTDIV1_MASK: u32 = 0x7 << Self::POSTDIV1_OFFSET;
    /// ## Bit mask for the `POSTDIV2` field.
    pub const POSTDIV2_MASK: u32 = 0x7 << Self::POSTDIV2_OFFSET;

    /// ## Get the PLL0 locked state.
    ///
    /// This returns an `bool` with the locked state.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL0Parameter;
    ///
    /// let pll0: PLL0Parameter = PLL0Parameter::DEFAULT;
    /// assert!(pll0.locked());
    /// let pll0: PLL0Parameter = pll0.lock();
    /// assert!(pll0.locked());
    /// let pll0: PLL0Parameter = pll0.unlock();
    /// assert!(!pll0.locked());
    /// ```
    pub const fn locked(&self) -> bool {
        self.0 & Self::LOCKED_MASK == Self::LOCKED_MASK
    }
    /// ## Lock the PLL0.
    #[must_use = "lock returns a modified PLL0Parameter"]
    pub const fn lock(mut self) -> Self {
        self.0 |= Self::LOCKED_MASK;
        self
    }
    /// ## Disable the PLL0.
    #[must_use = "unlock returns a modified PLL0Parameter"]
    pub const fn unlock(mut self) -> Self {
        self.0 &= !Self::LOCKED_MASK;
        self
    }

    /// ## Get the PLL0 enabled state.
    ///
    /// This returns an `bool` with the PLL0 enabled state.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL0Parameter;
    ///
    /// let pll0: PLL0Parameter = PLL0Parameter::DEFAULT;
    /// assert!(pll0.enabled());
    /// let pll0: PLL0Parameter = pll0.enable();
    /// assert!(pll0.enabled());
    /// let pll0: PLL0Parameter = pll0.disable();
    /// assert!(!pll0.enabled());
    /// ```
    pub const fn enabled(&self) -> bool {
        self.0 & Self::PLLEN_MASK == Self::PLLEN_MASK
    }
    /// ## Enable the PLL0.
    #[must_use = "enable returns a modified PLL0Parameter"]
    pub const fn enable(mut self) -> Self {
        self.0 |= Self::PLLEN_MASK;
        self
    }
    /// ## Disable the PLL0.
    #[must_use = "disable returns a modified PLL0Parameter"]
    pub const fn disable(mut self) -> Self {
        self.0 &= !Self::PLLEN_MASK;
        self
    }

    /// ## Get the PLL0 FB Divider.
    ///
    /// This returns an `u16` with the PLL0 FB Divider.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL0Parameter;
    ///
    /// let pll0: PLL0Parameter = PLL0Parameter::DEFAULT;
    /// assert_eq!(pll0.fbdiv(), 0x0060);
    /// let pll0: PLL0Parameter = pll0.set_fbdiv(0xAAA);
    /// assert_eq!(pll0.fbdiv(), 0x0AAA);
    /// let pll0: PLL0Parameter = pll0.set_fbdiv(0xF555);
    /// assert_eq!(pll0.fbdiv(), 0x0555);
    /// ```
    pub const fn fbdiv(&self) -> u16 {
        ((self.0 & Self::FBDIV_MASK) >> Self::FBDIV_OFFSET) as u16
    }
    /// ## Set the PLL0 FB Divider.
    #[must_use = "set_fbdiv returns a modified PLL0Parameter"]
    pub const fn set_fbdiv(mut self, fbdiv: u16) -> Self {
        self.0 &= !Self::FBDIV_MASK;
        self.0 |= ((fbdiv as u32) << Self::FBDIV_OFFSET) & Self::FBDIV_MASK;
        self
    }

    /// ## Get the PLL0 REF Divider.
    ///
    /// This returns an `u8` with the PLL0 REF Divider.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL0Parameter;
    ///
    /// let pll0: PLL0Parameter = PLL0Parameter::DEFAULT;
    /// assert_eq!(pll0.refdiv(), 0x01);
    /// let pll0: PLL0Parameter = pll0.set_refdiv(0xAA);
    /// assert_eq!(pll0.refdiv(), 0x2A);
    /// let pll0: PLL0Parameter = pll0.set_refdiv(0xF5);
    /// assert_eq!(pll0.refdiv(), 0x35);
    /// ```
    pub const fn refdiv(&self) -> u8 {
        ((self.0 & Self::REFDIV_MASK) >> Self::REFDIV_OFFSET) as u8
    }
    /// ## Set the PLL0 REF Divider.
    #[must_use = "set_refdiv returns a modified PLL0Parameter"]
    pub const fn set_refdiv(mut self, refdiv: u8) -> Self {
        self.0 &= !Self::REFDIV_MASK;
        self.0 |= ((refdiv as u32) << Self::REFDIV_OFFSET) & Self::REFDIV_MASK;
        self
    }

    /// ## Get the PLL0 POST Divider 1.
    ///
    /// This returns an `u8` with the PLL0 POST Divider 1.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL0Parameter;
    ///
    /// let pll0: PLL0Parameter = PLL0Parameter::DEFAULT;
    /// assert_eq!(pll0.postdiv1(), 0x06);
    /// let pll0: PLL0Parameter = pll0.set_postdiv1(0x07);
    /// assert_eq!(pll0.postdiv1(), 0x07);
    /// let pll0: PLL0Parameter = pll0.set_postdiv1(0xF5);
    /// assert_eq!(pll0.postdiv1(), 0x05);
    /// ```
    pub const fn postdiv1(&self) -> u8 {
        ((self.0 & Self::POSTDIV1_MASK) >> Self::POSTDIV1_OFFSET) as u8
    }
    /// ## Set the PLL0 POST Divider 1.
    #[must_use = "set_postdiv1 returns a modified PLL0Parameter"]
    pub const fn set_postdiv1(mut self, postdiv1: u8) -> Self {
        self.0 &= !Self::POSTDIV1_MASK;
        self.0 |= ((postdiv1 as u32) << Self::POSTDIV1_OFFSET) & Self::POSTDIV1_MASK;
        self
    }

    /// ## Get the PLL0 POST Divider 2.
    ///
    /// This returns an `u8` with the PLL0 POST Divider 2.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL0Parameter;
    ///
    /// let pll0: PLL0Parameter = PLL0Parameter::DEFAULT;
    /// assert_eq!(pll0.postdiv2(), 0x01);
    /// let pll0: PLL0Parameter = pll0.set_postdiv2(0x07);
    /// assert_eq!(pll0.postdiv2(), 0x07);
    /// let pll0: PLL0Parameter = pll0.set_postdiv2(0xF5);
    /// assert_eq!(pll0.postdiv2(), 0x05);
    /// ```
    pub const fn postdiv2(&self) -> u8 {
        ((self.0 & Self::POSTDIV2_MASK) >> Self::POSTDIV2_OFFSET) as u8
    }
    /// ## Set the PLL0 POST Divider 2.
    #[must_use = "set_postdiv2 returns a modified PLL0Parameter"]
    pub const fn set_postdiv2(mut self, postdiv2: u8) -> Self {
        self.0 &= !Self::POSTDIV2_MASK;
        self.0 |= ((postdiv2 as u32) << Self::POSTDIV2_OFFSET) & Self::POSTDIV2_MASK;
        self
    }

    /// ## Get the PLL0 Frequency.
    ///
    /// This returns an `HertzU32` with the PLL0 Frequency according to the clki_freq parameter.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL0Parameter;
    /// use fugit::HertzU32;
    ///
    /// let clki_freq = HertzU32::MHz(25);
    /// assert_eq!(PLL0Parameter::DEFAULT.frequency(clki_freq), HertzU32::MHz(400u32));
    /// ```
    pub const fn frequency(&self, clki_freq: HertzU32) -> HertzU32 {
        HertzU32::from_raw(
            clki_freq.raw() * (self.fbdiv() as u32)
                / ((self.refdiv() as u32) * (self.postdiv1() as u32) * (self.postdiv2() as u32)),
        )
    }
}

impl ::core::fmt::Display for PLL0Parameter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PLL0Parameter")
            .field("locked", &self.locked())
            .field("enabled", &self.enabled())
            .field("fbdiv", &self.fbdiv())
            .field("refdiv", &self.refdiv())
            .field("postdiv1", &self.postdiv1())
            .field("postdiv2", &self.postdiv2())
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for PLL0Parameter {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "PLL0Parameter {{ locked: {}, enabled: {}, fbdiv: {}, refdiv: {}, postdiv1: {}, postdiv2: {} }}",
            self.locked(),
            self.enabled(),
            self.fbdiv(),
            self.refdiv(),
            self.postdiv1(),
            self.postdiv2(),
        );
    }
}

/// # Chip Nonce Offset register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ChipNonceOffset(u32);
impl_boilerplate_for!(ChipNonceOffset);

impl ChipNonceOffset {
    /// ## Chip Nonce Offset register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{ChipNonceOffset, Register};
    ///
    /// assert_eq!(ChipNonceOffset::ADDR, ChipNonceOffset::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x0C;

    /// ## Chip Nonce Offset register reset value.
    pub const RESET: u32 = 0x0000_0000;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::ChipNonceOffset;
    ///
    /// assert_eq!(ChipNonceOffset::DEFAULT, ChipNonceOffset::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `CNOV` field.
    pub const CNOV_OFFSET: u8 = 31;
    /// ## Bit offset for the `CNO` field.
    pub const CNO_OFFSET: u8 = 0;

    /// ## Bit mask for the `CNOV` field.
    pub const CNOV_MASK: u32 = 0b1 << Self::CNOV_OFFSET;
    /// ## Bit mask for the `CNO` field.
    pub const CNO_MASK: u32 = 0b111 << Self::CNO_OFFSET;
}

impl ::core::fmt::Display for ChipNonceOffset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ChipNonceOffset").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChipNonceOffset {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ChipNonceOffset {{  }}",);
    }
}

/// # Hash Counting Number register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct HashCountingNumber(u32);
impl_boilerplate_for!(HashCountingNumber);

impl HashCountingNumber {
    /// ## Hash Counting Number register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{HashCountingNumber, Register};
    ///
    /// assert_eq!(HashCountingNumber::ADDR, HashCountingNumber::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x10;

    /// ## Hash Counting Number register reset value.
    pub const RESET: u32 = 0x0000_0000;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::HashCountingNumber;
    ///
    /// assert_eq!(HashCountingNumber::DEFAULT, HashCountingNumber::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `HCN` field.
    pub const HCN_OFFSET: u8 = 0;

    /// ## Bit mask for the `HCN` field.
    pub const HCN_MASK: u32 = 0xffff_ffff << Self::HCN_OFFSET;
}

impl ::core::fmt::Display for HashCountingNumber {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HashCountingNumber").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for HashCountingNumber {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "HashCountingNumber {{  }}",);
    }
}

/// # Ticket Mask register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TicketMask(u32);
impl_boilerplate_for!(TicketMask);

impl TicketMask {
    /// ## Ticket Mask register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{TicketMask, Register};
    ///
    /// assert_eq!(TicketMask::ADDR, TicketMask::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x14;

    /// ## Ticket Mask register reset value.
    pub const RESET: u32 = 0x0000_0000;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::TicketMask;
    ///
    /// assert_eq!(TicketMask::DEFAULT, TicketMask::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `TM3` field.
    pub const TM3_OFFSET: u8 = 24;
    /// ## Bit offset for the `TM2` field.
    pub const TM2_OFFSET: u8 = 16;
    /// ## Bit offset for the `TM1` field.
    pub const TM1_OFFSET: u8 = 8;
    /// ## Bit offset for the `TM0` field.
    pub const TM0_OFFSET: u8 = 0;

    /// ## Bit mask for the `TM3` field.
    pub const TM3_MASK: u32 = 0xff << Self::TM3_OFFSET;
    /// ## Bit mask for the `TM2` field.
    pub const TM2_MASK: u32 = 0xff << Self::TM2_OFFSET;
    /// ## Bit mask for the `TM1` field.
    pub const TM1_MASK: u32 = 0xff << Self::TM1_OFFSET;
    /// ## Bit mask for the `TM0` field.
    pub const TM0_MASK: u32 = 0xff << Self::TM0_OFFSET;
}

impl ::core::fmt::Display for TicketMask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TicketMask").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for TicketMask {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "TicketMask {{  }}",);
    }
}

/// # Misc Control register
///
/// Used to control various settings.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MiscControl(u32);
impl_boilerplate_for!(MiscControl);

impl MiscControl {
    /// ## Misc Control register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{MiscControl, Register};
    ///
    /// assert_eq!(MiscControl::ADDR, MiscControl::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x18;

    /// ## Misc Control register reset value.
    pub const RESET: u32 = 0x0000_3A01;

    /// ### Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::MiscControl;
    ///
    /// assert_eq!(MiscControl::DEFAULT, MiscControl::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `BT8D_8_5` field.
    pub const BT8D_8_5_OFFSET: u8 = 24;
    /// ## Bit offset for the `CORE_SRST` field.
    pub const CORE_SRST_OFFSET: u8 = 22;
    /// ## Bit offset for the `SPAT_NOD` field.
    pub const SPAT_NOD_OFFSET: u8 = 21;
    /// ## Bit offset for the `RVS_K0` field.
    pub const RVS_K0_OFFSET: u8 = 20;
    /// ## Bit offset for the `DSCLK_SEL` field.
    pub const DSCLK_SEL_OFFSET: u8 = 18;
    /// ## Bit offset for the `TOP_CLK_SEL` field.
    pub const TOP_CLK_SEL_OFFSET: u8 = 17;
    /// ## Bit offset for the `BCK_SEL` field.
    pub const BCK_SEL_OFFSET: u8 = 16;
    /// ## Bit offset for the `RET_ERR_NONCE` field.
    pub const RET_ERR_NONCE_OFFSET: u8 = 15;
    /// ## Bit offset for the `RFS` field.
    pub const RFS_OFFSET: u8 = 14;
    /// ## Bit offset for the `INV_CLKO` field.
    pub const INV_CLKO_OFFSET: u8 = 13;
    /// ## Bit offset for the `BT8D_4_0` field.
    pub const BT8D_4_0_OFFSET: u8 = 8;
    /// ## Bit offset for the `RET_WORK_ERR_FLAG` field.
    pub const RET_WORK_ERR_FLAG_OFFSET: u8 = 7;
    /// ## Bit offset for the `TFS` field.
    pub const TFS_OFFSET: u8 = 4;
    /// ## Bit offset for the `HASHRATE_TWS` field.
    pub const HASHRATE_TWS_OFFSET: u8 = 0;

    /// ## Bit mask for the `BT8D_8_5` field.
    pub const BT8D_8_5_MASK: u32 = 0b1111 << Self::BT8D_8_5_OFFSET;
    /// ## Bit mask for the `CORE_SRST` field.
    pub const CORE_SRST_MASK: u32 = 0b1 << Self::CORE_SRST_OFFSET;
    /// ## Bit mask for the `SPAT_NOD` field.
    pub const SPAT_NOD_MASK: u32 = 0b1 << Self::SPAT_NOD_OFFSET;
    /// ## Bit mask for the `RVS_K0` field.
    pub const RVS_K0_MASK: u32 = 0b1 << Self::RVS_K0_OFFSET;
    /// ## Bit mask for the `DSCLK_SEL` field.
    pub const DSCLK_SEL_MASK: u32 = 0b11 << Self::DSCLK_SEL_OFFSET;
    /// ## Bit mask for the `TOP_CLK_SEL` field.
    pub const TOP_CLK_SEL_MASK: u32 = 0b1 << Self::TOP_CLK_SEL_OFFSET;
    /// ## Bit mask for the `BCK_SEL` field.
    pub const BCK_SEL_MASK: u32 = 0b1 << Self::BCK_SEL_OFFSET;
    /// ## Bit mask for the `RET_ERR_NONCE` field.
    pub const RET_ERR_NONCE_MASK: u32 = 0b1 << Self::RET_ERR_NONCE_OFFSET;
    /// ## Bit mask for the `RFS` field.
    pub const RFS_MASK: u32 = 0b1 << Self::RFS_OFFSET;
    /// ## Bit mask for the `INV_CLKO` field.
    pub const INV_CLKO_MASK: u32 = 0b1 << Self::INV_CLKO_OFFSET;
    /// ## Bit mask for the `BT8D_4_0` field.
    pub const BT8D_4_0_MASK: u32 = 0b11111 << Self::BT8D_4_0_OFFSET;
    /// ## Bit mask for the `RET_WORK_ERR_FLAG` field.
    pub const RET_WORK_ERR_FLAG_MASK: u32 = 0b1 << Self::RET_WORK_ERR_FLAG_OFFSET;
    /// ## Bit mask for the `TFS` field.
    pub const TFS_MASK: u32 = 0xb111 << Self::TFS_OFFSET;
    /// ## Bit mask for the `HASHRATE_TWS` field.
    pub const HASHRATE_TWS_MASK: u32 = 0xb11 << Self::HASHRATE_TWS_OFFSET;

    /// ## Get the BT8D.
    ///
    /// This returns an `u16` with the 9-bits BT8D value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::MiscControl;
    ///
    /// let misc: MiscControl = MiscControl::DEFAULT;
    /// assert_eq!(misc.bt8d(), 0x001A);
    /// let misc: MiscControl = misc.set_bt8d(0x1AA);
    /// assert_eq!(misc.bt8d(), 0x01AA);
    /// let misc: MiscControl = misc.set_bt8d(0xFF55);
    /// assert_eq!(misc.bt8d(), 0x0155);
    /// ```
    pub const fn bt8d(&self) -> u16 {
        ((((self.0 & Self::BT8D_8_5_MASK) >> Self::BT8D_8_5_OFFSET) as u16) << 5)
            | (((self.0 & Self::BT8D_4_0_MASK) >> Self::BT8D_4_0_OFFSET) as u16)
    }
    /// ## Set the BT8D.
    #[must_use = "set_bt8d returns a modified MiscControl"]
    pub const fn set_bt8d(mut self, bt8d: u16) -> Self {
        self.0 &= !Self::BT8D_8_5_MASK;
        self.0 &= !Self::BT8D_4_0_MASK;
        self.0 |= (((bt8d >> 5) as u32) << Self::BT8D_8_5_OFFSET) & Self::BT8D_8_5_MASK;
        self.0 |= ((bt8d as u32) << Self::BT8D_4_0_OFFSET) & Self::BT8D_4_0_MASK;
        self
    }

    /// ## Reset the Core.
    ///
    /// This returns an `bool` with the Core Reset state.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::MiscControl;
    ///
    /// let misc: MiscControl = MiscControl::DEFAULT;
    /// assert!(!misc.core_srst());
    /// let misc: MiscControl = misc.reset_core();
    /// assert!(misc.core_srst());
    /// ```
    pub const fn core_srst(&self) -> bool {
        self.0 & Self::CORE_SRST_MASK == Self::CORE_SRST_MASK
    }
    /// ## Reset the Core.
    #[must_use = "reset_core returns a modified MiscControl"]
    pub const fn reset_core(mut self) -> Self {
        self.0 |= Self::CORE_SRST_MASK;
        self
    }

    /// ## Get the Baudrate Clock Select.
    ///
    /// This returns an `BaudrateClockSelect` with the current Baudrate Clock Select.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::MiscControl;
    /// use bm1397_protocol::specifier::BaudrateClockSelect;
    ///
    /// let misc: MiscControl = MiscControl::DEFAULT;
    /// assert_eq!(misc.bclk_sel(), BaudrateClockSelect::Clki);
    /// let misc: MiscControl = misc.set_bclk_sel(BaudrateClockSelect::Clki);
    /// assert_eq!(misc.bclk_sel(), BaudrateClockSelect::Clki);
    /// let misc: MiscControl = misc.set_bclk_sel(BaudrateClockSelect::Pll3);
    /// assert_eq!(misc.bclk_sel(), BaudrateClockSelect::Pll3);
    /// ```
    pub const fn bclk_sel(&self) -> BaudrateClockSelect {
        match self.0 & Self::BCK_SEL_MASK == Self::BCK_SEL_MASK {
            true => BaudrateClockSelect::Pll3,
            false => BaudrateClockSelect::Clki,
        }
    }
    /// ## Set the Baudrate Clock Select.
    #[must_use = "set_bclk_sel returns a modified MiscControl"]
    pub const fn set_bclk_sel(mut self, bclk_sel: BaudrateClockSelect) -> Self {
        self.0 &= !Self::BCK_SEL_MASK;
        match bclk_sel {
            BaudrateClockSelect::Pll3 => self.0 |= Self::BCK_SEL_MASK,
            BaudrateClockSelect::Clki => self.0 &= !Self::BCK_SEL_MASK,
        }
        self
    }
}

impl ::core::fmt::Display for MiscControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MiscControl")
            .field("bt8d", &self.bt8d())
            .field("core_srst", &self.core_srst())
            .field("bclk_sel", &self.bclk_sel())
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for MiscControl {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "MiscControl {{ bt8d: {}, core_srst: {}, bclk_sel: {} }}",
            self.bt8d(),
            self.core_srst(),
            self.bclk_sel(),
        );
    }
}

/// # I2C Control register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct I2CControl(u32);
impl_boilerplate_for!(I2CControl);

impl I2CControl {
    /// ## I2C Control register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{I2CControl, Register};
    ///
    /// assert_eq!(I2CControl::ADDR, I2CControl::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x1C;

    /// ## I2C Control register reset value.
    pub const RESET: u32 = 0x0100_0000;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::I2CControl;
    ///
    /// assert_eq!(I2CControl::DEFAULT, I2CControl::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `BUSY` field.
    pub const BUSY_OFFSET: u8 = 31;
    /// ## Bit offset for the `DO_CMD` field.
    pub const DO_CMD_OFFSET: u8 = 24;
    /// ## Bit offset for the `I2C_ADDR` field.
    pub const I2C_ADDR_OFFSET: u8 = 17;
    /// ## Bit offset for the `RD_WR` field.
    pub const RD_WR_OFFSET: u8 = 16;
    /// ## Bit offset for the `I2C_REG_ADDR` field.
    pub const I2C_REG_ADDR_OFFSET: u8 = 8;
    /// ## Bit offset for the `I2C_REG_VAL` field.
    pub const I2C_REG_VAL_OFFSET: u8 = 0;

    /// ## Bit mask for the `BUSY` field.
    pub const BUSY_MASK: u32 = 0b1 << Self::BUSY_OFFSET;
    /// ## Bit mask for the `DO_CMD` field.
    pub const DO_CMD_MASK: u32 = 0b1 << Self::DO_CMD_OFFSET;
    /// ## Bit mask for the `I2C_ADDR` field.
    pub const I2C_ADDR_MASK: u32 = 0x7f << Self::I2C_ADDR_OFFSET;
    /// ## Bit mask for the `RD_WR` field.
    pub const RD_WR_MASK: u32 = 0b1 << Self::RD_WR_OFFSET;
    /// ## Bit mask for the `I2C_REG_ADDR` field.
    pub const I2C_REG_ADDR_MASK: u32 = 0xff << Self::I2C_REG_ADDR_OFFSET;
    /// ## Bit mask for the `I2C_REG_VAL` field.
    pub const I2C_REG_VAL_MASK: u32 = 0xff << Self::I2C_REG_VAL_OFFSET;
}

impl ::core::fmt::Display for I2CControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("I2CControl").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for I2CControl {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "I2CControl {{  }}",);
    }
}

/// # Ordered Clock Enable register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct OrderedClockEnable(u32);
impl_boilerplate_for!(OrderedClockEnable);

impl OrderedClockEnable {
    /// ## Ordered Clock Enable register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{OrderedClockEnable, Register};
    ///
    /// assert_eq!(OrderedClockEnable::ADDR, OrderedClockEnable::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x20;

    /// ## Ordered Clock Enable register reset value.
    pub const RESET: u32 = 0x0000_ffff;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::OrderedClockEnable;
    ///
    /// assert_eq!(OrderedClockEnable::DEFAULT, OrderedClockEnable::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `CLKEN` field.
    pub const CLKEN_OFFSET: u8 = 0;

    /// ## Bit mask for the `CLKEN` field.
    pub const CLKEN_MASK: u32 = 0xffff << Self::CLKEN_OFFSET;
}

impl ::core::fmt::Display for OrderedClockEnable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OrderedClockEnable").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for OrderedClockEnable {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "OrderedClockEnable {{  }}",);
    }
}

/// # Fast UART Configuration register
///
/// Used to configure UART settings.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FastUARTConfiguration(u32);
impl_boilerplate_for!(FastUARTConfiguration);

impl FastUARTConfiguration {
    /// ## Fast UART Configuration register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{FastUARTConfiguration, Register};
    ///
    /// assert_eq!(FastUARTConfiguration::ADDR, FastUARTConfiguration::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x28;

    /// ## Fast UART Configuration register reset value.
    pub const RESET: u32 = 0x0600_000F;

    /// ### Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::FastUARTConfiguration;
    ///
    /// assert_eq!(FastUARTConfiguration::DEFAULT, FastUARTConfiguration::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `DIV4_ODDSET` field.
    pub const DIV4_ODDSET_OFFSET: u8 = 30;
    /// ## Bit offset for the `PLL3_DIV4` field.
    pub const PLL3_DIV4_OFFSET: u8 = 24;
    /// ## Bit offset for the `USRC_ODDSET` field.
    pub const USRC_ODDSET_OFFSET: u8 = 22;
    /// ## Bit offset for the `USRC_DIV` field.
    pub const USRC_DIV_OFFSET: u8 = 16;
    /// ## Bit offset for the `FORCE_CORE_EN` field.
    pub const FORCE_CORE_EN_OFFSET: u8 = 15;
    /// ## Bit offset for the `CLKO_SEL` field.
    pub const CLKO_SEL_OFFSET: u8 = 14;
    /// ## Bit offset for the `CLKO_ODDSET` field.
    pub const CLKO_ODDSET_OFFSET: u8 = 12;
    /// ## Bit offset for the `CLKO_DIV` field.
    pub const CLKO_DIV_OFFSET: u8 = 0;

    /// ## Bit mask for the `DIV4_ODDSET` field.
    pub const DIV4_ODDSET_MASK: u32 = 0b11 << Self::DIV4_ODDSET_OFFSET;
    /// ## Bit mask for the `PLL3_DIV4` field.
    pub const PLL3_DIV4_MASK: u32 = 0b1111 << Self::PLL3_DIV4_OFFSET;
    /// ## Bit mask for the `USRC_ODDSET` field.
    pub const USRC_ODDSET_MASK: u32 = 0b11 << Self::USRC_ODDSET_OFFSET;
    /// ## Bit mask for the `USRC_DIV` field.
    pub const USRC_DIV_MASK: u32 = 0x3f << Self::USRC_DIV_OFFSET;
    /// ## Bit mask for the `FORCE_CORE_EN` field.
    pub const FORCE_CORE_EN_MASK: u32 = 0b1 << Self::FORCE_CORE_EN_OFFSET;
    /// ## Bit mask for the `CLKO_SEL` field.
    pub const CLKO_SEL_MASK: u32 = 0b1 << Self::CLKO_SEL_OFFSET;
    /// ## Bit mask for the `CLKO_ODDSET` field.
    pub const CLKO_ODDSET_MASK: u32 = 0b11 << Self::CLKO_ODDSET_OFFSET;
    /// ## Bit mask for the `CLKO_DIV` field.
    pub const CLKO_DIV_MASK: u32 = 0xff << Self::CLKO_DIV_OFFSET;

    /// ## Get the PLL3_DIV4.
    ///
    /// This returns an `u8` with the PLL3_DIV4 value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::FastUARTConfiguration;
    ///
    /// let uart_conf: FastUARTConfiguration = FastUARTConfiguration::DEFAULT;
    /// assert_eq!(uart_conf.pll3_div4(), 0x06);
    /// let uart_conf: FastUARTConfiguration = uart_conf.set_pll3_div4(0x0A);
    /// assert_eq!(uart_conf.pll3_div4(), 0x0A);
    /// let uart_conf: FastUARTConfiguration = uart_conf.set_pll3_div4(0xF5);
    /// assert_eq!(uart_conf.pll3_div4(), 0x05);
    /// ```
    pub const fn pll3_div4(&self) -> u8 {
        ((self.0 & Self::PLL3_DIV4_MASK) >> Self::PLL3_DIV4_OFFSET) as u8
    }
    /// ## Set the PLL3_DIV4.
    #[must_use = "set_pll3_div4 returns a modified FastUARTConfiguration"]
    pub const fn set_pll3_div4(mut self, pll3_div4: u8) -> Self {
        self.0 &= !Self::PLL3_DIV4_MASK;
        self.0 |= ((pll3_div4 as u32) << Self::PLL3_DIV4_OFFSET) & Self::PLL3_DIV4_MASK;
        self
    }
}

impl ::core::fmt::Display for FastUARTConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FastUARTConfiguration")
            .field("pll3_div4", &self.pll3_div4())
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for FastUARTConfiguration {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "FastUARTConfiguration {{ pll3_div4: {} }}",
            self.pll3_div4(),
        );
    }
}

/// # UART Relay register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct UARTRelay(u32);
impl_boilerplate_for!(UARTRelay);

impl UARTRelay {
    /// ## UART Relay register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{UARTRelay, Register};
    ///
    /// assert_eq!(UARTRelay::ADDR, UARTRelay::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x2C;

    /// ## UART Relay register reset value.
    pub const RESET: u32 = 0x000f_0000;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::UARTRelay;
    ///
    /// assert_eq!(UARTRelay::DEFAULT, UARTRelay::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `GAP_CNT` field.
    pub const GAP_CNT_OFFSET: u8 = 16;
    /// ## Bit offset for the `RO_REL_EN` field.
    pub const RO_REL_EN_OFFSET: u8 = 1;
    /// ## Bit offset for the `CO_REL_EN` field.
    pub const CO_REL_EN_OFFSET: u8 = 0;

    /// ## Bit mask for the `GAP_CNT` field.
    pub const GAP_CNT_MASK: u32 = 0xffff << Self::GAP_CNT_OFFSET;
    /// ## Bit mask for the `RO_REL_EN` field.
    pub const RO_REL_EN_MASK: u32 = 0b1 << Self::RO_REL_EN_OFFSET;
    /// ## Bit mask for the `CO_REL_EN` field.
    pub const CO_REL_EN_MASK: u32 = 0b1 << Self::CO_REL_EN_OFFSET;
}

impl ::core::fmt::Display for UARTRelay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UARTRelay").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for UARTRelay {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "UARTRelay {{  }}",);
    }
}

/// # Ticket Mask 2 register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TicketMask2(u32);
impl_boilerplate_for!(TicketMask2);

impl TicketMask2 {
    /// ## Ticket Mask 2 register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{TicketMask2, Register};
    ///
    /// assert_eq!(TicketMask2::ADDR, TicketMask2::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x38;

    /// ## Ticket Mask 2 register reset value.
    pub const RESET: u32 = 0x0000_0000;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::TicketMask2;
    ///
    /// assert_eq!(TicketMask2::DEFAULT, TicketMask2::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `TM` field.
    pub const TM_OFFSET: u8 = 0;

    /// ## Bit mask for the `TM` field.
    pub const TM_MASK: u32 = 0xffff_ffff << Self::TM_OFFSET;
}

impl ::core::fmt::Display for TicketMask2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TicketMask2").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for TicketMask2 {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "TicketMask2 {{  }}",);
    }
}

/// # Core Register Control register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CoreRegisterControl(u32);
impl_boilerplate_for!(CoreRegisterControl);

impl CoreRegisterControl {
    /// ## Core Register Control register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{CoreRegisterControl, Register};
    ///
    /// assert_eq!(CoreRegisterControl::ADDR, CoreRegisterControl::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x3C;

    /// ## Core Register Control register reset value.
    pub const RESET: u32 = 0x0000_0000;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::CoreRegisterControl;
    ///
    /// assert_eq!(CoreRegisterControl::DEFAULT, CoreRegisterControl::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `RD_WR1` field.
    pub const RD_WR1_OFFSET: u8 = 31;
    /// ## Bit offset for the `CORE_ID` field.
    pub const CORE_ID_OFFSET: u8 = 16;
    /// ## Bit offset for the `RD_WR2` field.
    pub const RD_WR2_OFFSET: u8 = 15;
    /// ## Bit offset for the `CORE_REG_ID` field.
    pub const CORE_REG_ID_OFFSET: u8 = 8;
    /// ## Bit offset for the `CORE_REG_VAL` field.
    pub const CORE_REG_VAL_OFFSET: u8 = 0;

    /// ## Bit mask for the `RD_WR` field.
    pub const RD_WR_MASK: u32 = 0b1 << Self::RD_WR1_OFFSET | 0b1 << Self::RD_WR2_OFFSET;
    /// ## Bit mask for the `CORE_ID` field.
    pub const CORE_ID_MASK: u32 = 0xff << Self::CORE_ID_OFFSET;
    /// ## Bit mask for the `CORE_REG_ID` field.
    pub const CORE_REG_ID_MASK: u32 = 0b1111 << Self::CORE_REG_ID_OFFSET;
    /// ## Bit mask for the `CORE_REG_VAL` field.
    pub const CORE_REG_VAL_MASK: u32 = 0xff << Self::CORE_REG_VAL_OFFSET;

    /// ## Set CoreRegisterControl for a Core Register Read.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{CoreRegisterControl, Register};
    /// use bm1397_protocol::core_register::{ClockDelayCtrl};
    ///
    /// let crc: CoreRegisterControl = CoreRegisterControl::DEFAULT;
    /// assert_eq!(crc.val(), 0x0000_0000);
    /// let cdc: ClockDelayCtrl = ClockDelayCtrl::default();
    /// let crc: CoreRegisterControl = crc.read(0, cdc);
    /// assert_eq!(crc.val(), 0x0000_00ff);
    /// let cdc: ClockDelayCtrl = cdc.enable_multi_midstate();
    /// let crc: CoreRegisterControl = crc.write(0, cdc);
    /// assert_eq!(crc.val(), 0x8000_8004);
    /// ```
    #[must_use = "read returns a modified CoreRegisterControl"]
    pub fn read(mut self, core_id: u8, core_reg: impl CoreRegister) -> Self {
        self.0 &= !Self::RD_WR_MASK;
        self.0 &= !Self::CORE_ID_MASK;
        self.0 |= ((core_id as u32) << Self::CORE_ID_OFFSET) & Self::CORE_ID_MASK;
        self.0 &= !Self::CORE_REG_ID_MASK;
        self.0 |= ((core_reg.id() as u32) << Self::CORE_REG_ID_OFFSET) & Self::CORE_REG_ID_MASK;
        self.0 |= Self::CORE_REG_VAL_MASK;
        self
    }
    /// ## Set CoreRegisterControl for a Core Register Write.
    #[must_use = "write returns a modified CoreRegisterControl"]
    pub fn write(mut self, core_id: u8, core_reg: impl CoreRegister) -> Self {
        self.0 |= Self::RD_WR_MASK;
        self.0 &= !Self::CORE_ID_MASK;
        self.0 |= ((core_id as u32) << Self::CORE_ID_OFFSET) & Self::CORE_ID_MASK;
        self.0 &= !Self::CORE_REG_ID_MASK;
        self.0 |= ((core_reg.id() as u32) << Self::CORE_REG_ID_OFFSET) & Self::CORE_REG_ID_MASK;
        self.0 &= !Self::CORE_REG_VAL_MASK;
        self.0 |= ((core_reg.val() as u32) << Self::CORE_REG_VAL_OFFSET) & Self::CORE_REG_VAL_MASK;
        self
    }
}

impl ::core::fmt::Display for CoreRegisterControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CoreRegisterControl").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for CoreRegisterControl {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "CoreRegisterControl {{  }}",);
    }
}

/// # Core Register Value register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CoreRegisterValue(u32);
impl_boilerplate_for!(CoreRegisterValue);

impl CoreRegisterValue {
    /// ## Core Register Value register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{CoreRegisterValue, Register};
    ///
    /// assert_eq!(CoreRegisterValue::ADDR, CoreRegisterValue::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x40;

    /// ## Core Register Value register reset value.
    pub const RESET: u32 = 0x0000_0000;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::CoreRegisterValue;
    ///
    /// assert_eq!(CoreRegisterValue::DEFAULT, CoreRegisterValue::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `CORE_ID` field.
    pub const CORE_ID_OFFSET: u8 = 16;
    /// ## Bit offset for the `FOUND` field.
    pub const FOUND_OFFSET: u8 = 8;
    /// ## Bit offset for the `CORE_REG_VAL` field.
    pub const CORE_REG_VAL_OFFSET: u8 = 0;

    /// ## Bit mask for the `CORE_ID` field.
    pub const CORE_ID_MASK: u32 = 0x1ff << Self::CORE_ID_OFFSET;
    /// ## Bit mask for the `FOUND` field.
    pub const FOUND_MASK: u32 = 0xff << Self::FOUND_OFFSET;
    /// ## Bit mask for the `CORE_REG_VAL` field.
    pub const CORE_REG_VAL_MASK: u32 = 0xff << Self::CORE_REG_VAL_OFFSET;

    /// ## Get the CORE_ID.
    ///
    /// This returns an `u16` with the CORE_ID value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::CoreRegisterValue;
    ///
    /// let crv: CoreRegisterValue = CoreRegisterValue::from(0x0001_1234);
    /// assert_eq!(crv.core_id(), 0x0001);
    /// ```
    pub const fn core_id(&self) -> u16 {
        ((self.0 & Self::CORE_ID_MASK) >> Self::CORE_ID_OFFSET) as u16
    }

    /// ## Get the FOUND.
    ///
    /// This returns an `u8` with the FOUND value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::CoreRegisterValue;
    ///
    /// let crv: CoreRegisterValue = CoreRegisterValue::from(0x0001_1234);
    /// assert_eq!(crv.found(), 0x12);
    /// ```
    pub const fn found(&self) -> u8 {
        ((self.0 & Self::FOUND_MASK) >> Self::FOUND_OFFSET) as u8
    }

    /// ## Get the CORE_REG_VAL.
    ///
    /// This returns an `u8` with the CORE_REG_VAL value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::CoreRegisterValue;
    ///
    /// let crv: CoreRegisterValue = CoreRegisterValue::from(0x0001_1234);
    /// assert_eq!(crv.core_reg_val(), 0x34);
    /// ```
    pub const fn core_reg_val(&self) -> u8 {
        ((self.0 & Self::CORE_REG_VAL_MASK) >> Self::CORE_REG_VAL_OFFSET) as u8
    }

    /// ## Get the CoreRegister according to the given core_reg_id
    /// and the current CORE_REG_VAL.
    ///
    /// ## Return
    /// - `Ok(CoreRegisters)` with the corresponding `CoreRegister`.
    /// - `Err(Error::UnknownCoreRegister(u8))` with the core register id
    ///    if it do not match a known `CoreRegisters`.
    ///
    /// ### Examples
    /// ```
    /// use bm1397_protocol::core_register::{ProcessMonitorData, CoreRegisters};
    /// use bm1397_protocol::Error;
    /// use bm1397_protocol::register::CoreRegisterValue;
    ///
    /// let crv: CoreRegisterValue = CoreRegisterValue::from(0x0001_0234);
    /// // ProcessMonitorData
    /// let resp = crv.core_reg(0x02);
    /// assert!(resp.is_ok());
    /// assert_eq!(resp.unwrap(), CoreRegisters::ProcessMonitorData(ProcessMonitorData::from(0x34)));
    ///
    /// // Error::UnknownCoreRegister(0xF0)
    /// let resp = crv.core_reg(0xF0);
    /// assert!(resp.is_err());
    /// assert_eq!(resp.unwrap_err(), Error::UnknownCoreRegister(0xF0));
    /// ```
    pub fn core_reg(&self, core_reg_id: u8) -> Result<CoreRegisters, Error> {
        let core_reg = match core_reg_id {
            ClockDelayCtrl::ID => {
                CoreRegisters::ClockDelayCtrl(ClockDelayCtrl::from(self.core_reg_val()))
            }
            ProcessMonitorCtrl::ID => {
                CoreRegisters::ProcessMonitorCtrl(ProcessMonitorCtrl::from(self.core_reg_val()))
            }
            ProcessMonitorData::ID => {
                CoreRegisters::ProcessMonitorData(ProcessMonitorData::from(self.core_reg_val()))
            }
            CoreError::ID => CoreRegisters::CoreError(CoreError::from(self.core_reg_val())),
            CoreEnable::ID => CoreRegisters::CoreEnable(CoreEnable::from(self.core_reg_val())),
            HashClockCtrl::ID => {
                CoreRegisters::HashClockCtrl(HashClockCtrl::from(self.core_reg_val()))
            }
            HashClockCounter::ID => {
                CoreRegisters::HashClockCounter(HashClockCounter::from(self.core_reg_val()))
            }
            SweepClockCtrl::ID => {
                CoreRegisters::SweepClockCtrl(SweepClockCtrl::from(self.core_reg_val()))
            }
            id => return Err(Error::UnknownCoreRegister(id)),
        };
        Ok(core_reg)
    }
}

impl ::core::fmt::Display for CoreRegisterValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CoreRegisterValue").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for CoreRegisterValue {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "CoreRegisterValue {{  }}",);
    }
}

/// # External Temperature Sensor Read register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ExternalTemperatureSensorRead(u32);
impl_boilerplate_for!(ExternalTemperatureSensorRead);

impl ExternalTemperatureSensorRead {
    /// ## External Temperature Sensor Read register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{ExternalTemperatureSensorRead, Register};
    ///
    /// assert_eq!(ExternalTemperatureSensorRead::ADDR, ExternalTemperatureSensorRead::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x44;

    /// ## External Temperature Sensor Read register reset value.
    pub const RESET: u32 = 0x0000_0100;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::ExternalTemperatureSensorRead;
    ///
    /// assert_eq!(ExternalTemperatureSensorRead::DEFAULT, ExternalTemperatureSensorRead::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `LOCAL_TEMP_ADDR` field.
    pub const LOCAL_TEMP_ADDR_OFFSET: u8 = 24;
    /// ## Bit offset for the `LOCAL_TEMP_DATA` field.
    pub const LOCAL_TEMP_DATA_OFFSET: u8 = 16;
    /// ## Bit offset for the `EXTERNAL_TEMP_ADDR` field.
    pub const EXTERNAL_TEMP_ADDR_OFFSET: u8 = 8;
    /// ## Bit offset for the `EXTERNAL_TEMP_DATA` field.
    pub const EXTERNAL_TEMP_DATA_OFFSET: u8 = 0;

    /// ## Bit mask for the `LOCAL_TEMP_ADDR` field.
    pub const LOCAL_TEMP_ADDR_MASK: u32 = 0xff << Self::LOCAL_TEMP_ADDR_OFFSET;
    /// ## Bit mask for the `LOCAL_TEMP_DATA` field.
    pub const LOCAL_TEMP_DATA_MASK: u32 = 0xff << Self::LOCAL_TEMP_DATA_OFFSET;
    /// ## Bit mask for the `EXTERNAL_TEMP_ADDR` field.
    pub const EXTERNAL_TEMP_ADDR_MASK: u32 = 0xff << Self::EXTERNAL_TEMP_ADDR_OFFSET;
    /// ## Bit mask for the `EXTERNAL_TEMP_DATA` field.
    pub const EXTERNAL_TEMP_DATA_MASK: u32 = 0xff << Self::EXTERNAL_TEMP_DATA_OFFSET;
}

impl ::core::fmt::Display for ExternalTemperatureSensorRead {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ExternalTemperatureSensorRead").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ExternalTemperatureSensorRead {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ExternalTemperatureSensorRead {{  }}",);
    }
}

/// # Error Flag register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ErrorFlag(u32);
impl_boilerplate_for!(ErrorFlag);

impl ErrorFlag {
    /// ## Error Flag register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{ErrorFlag, Register};
    ///
    /// assert_eq!(ErrorFlag::ADDR, ErrorFlag::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x48;

    /// ## Error Flag register reset value.
    pub const RESET: u32 = 0xff00_0000;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::ErrorFlag;
    ///
    /// assert_eq!(ErrorFlag::DEFAULT, ErrorFlag::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `CMD_ERR_CNT` field.
    pub const CMD_ERR_CNT_OFFSET: u8 = 24;
    /// ## Bit offset for the `WORK_ERR_CNT` field.
    pub const WORK_ERR_CNT_OFFSET: u8 = 16;
    /// ## Bit offset for the `CORE_RESP_ERR` field.
    pub const CORE_RESP_ERR_OFFSET: u8 = 0;

    /// ## Bit mask for the `CMD_ERR_CNT` field.
    pub const CMD_ERR_CNT_MASK: u32 = 0xff << Self::CMD_ERR_CNT_OFFSET;
    /// ## Bit mask for the `WORK_ERR_CNT` field.
    pub const WORK_ERR_CNT_MASK: u32 = 0xff << Self::WORK_ERR_CNT_OFFSET;
    /// ## Bit mask for the `CORE_RESP_ERR` field.
    pub const CORE_RESP_ERR_MASK: u32 = 0xff << Self::CORE_RESP_ERR_OFFSET;
}

impl ::core::fmt::Display for ErrorFlag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ErrorFlag").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ErrorFlag {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ErrorFlag {{  }}",);
    }
}

/// # Nonce Error Counter register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct NonceErrorCounter(u32);
impl_boilerplate_for!(NonceErrorCounter);

impl NonceErrorCounter {
    /// ## Nonce Error Counter register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{NonceErrorCounter, Register};
    ///
    /// assert_eq!(NonceErrorCounter::ADDR, NonceErrorCounter::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x4C;

    /// ## Nonce Error Counter register reset value.
    pub const RESET: u32 = 0x0000_0000;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::NonceErrorCounter;
    ///
    /// assert_eq!(NonceErrorCounter::DEFAULT, NonceErrorCounter::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `ERR_CNT` field.
    pub const ERR_CNT_OFFSET: u8 = 0;

    /// ## Bit mask for the `ERR_CNT` field.
    pub const ERR_CNT_MASK: u32 = 0xffff_ffff << Self::ERR_CNT_OFFSET;
}

impl ::core::fmt::Display for NonceErrorCounter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NonceErrorCounter").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for NonceErrorCounter {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "NonceErrorCounter {{  }}",);
    }
}

/// # Nonce Overflow Counter register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct NonceOverflowCounter(u32);
impl_boilerplate_for!(NonceOverflowCounter);

impl NonceOverflowCounter {
    /// ## Nonce Overflow Counter register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{NonceOverflowCounter, Register};
    ///
    /// assert_eq!(NonceOverflowCounter::ADDR, NonceOverflowCounter::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x50;

    /// ## Nonce Overflow Counter register reset value.
    pub const RESET: u32 = 0x0000_0000;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::NonceOverflowCounter;
    ///
    /// assert_eq!(NonceOverflowCounter::DEFAULT, NonceOverflowCounter::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `OVRF_CNT` field.
    pub const OVRF_CNT_OFFSET: u8 = 0;

    /// ## Bit mask for the `OVRF_CNT` field.
    pub const OVRF_CNT_MASK: u32 = 0xffff_ffff << Self::OVRF_CNT_OFFSET;
}

impl ::core::fmt::Display for NonceOverflowCounter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NonceOverflowCounter").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for NonceOverflowCounter {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "NonceOverflowCounter {{  }}",);
    }
}

/// # Analog Mux Control register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnalogMuxControl(u32);
impl_boilerplate_for!(AnalogMuxControl);

impl AnalogMuxControl {
    /// ## Analog Mux Control register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{AnalogMuxControl, Register};
    ///
    /// assert_eq!(AnalogMuxControl::ADDR, AnalogMuxControl::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x54;

    /// ## Analog Mux Control register reset value.
    pub const RESET: u32 = 0x0000_0000;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::AnalogMuxControl;
    ///
    /// assert_eq!(AnalogMuxControl::DEFAULT, AnalogMuxControl::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `DIODE_VDD_MUX_SEL` field.
    pub const DIODE_VDD_MUX_SEL_OFFSET: u8 = 0;

    /// ## Bit mask for the `DIODE_VDD_MUX_SEL` field.
    pub const DIODE_VDD_MUX_SEL_MASK: u32 = 0b111 << Self::DIODE_VDD_MUX_SEL_OFFSET;
}

impl ::core::fmt::Display for AnalogMuxControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AnalogMuxControl").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for AnalogMuxControl {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "AnalogMuxControl {{  }}",);
    }
}

/// # Io Driver Strenght Configuration register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct IoDriverStrenghtConfiguration(u32);
impl_boilerplate_for!(IoDriverStrenghtConfiguration);

impl IoDriverStrenghtConfiguration {
    /// ## Io Driver Strenght Configuration register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{IoDriverStrenghtConfiguration, Register};
    ///
    /// assert_eq!(IoDriverStrenghtConfiguration::ADDR, IoDriverStrenghtConfiguration::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x58;

    /// ## Io Driver Strenght Configuration register reset value.
    pub const RESET: u32 = 0x0211_2111;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::IoDriverStrenghtConfiguration;
    ///
    /// assert_eq!(IoDriverStrenghtConfiguration::DEFAULT, IoDriverStrenghtConfiguration::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `RF_DS` field.
    pub const RF_DS_OFFSET: u8 = 24;
    /// ## Bit offset for the `D3RS_EN` field.
    pub const D3RS_EN_OFFSET: u8 = 23;
    /// ## Bit offset for the `D2RS_EN` field.
    pub const D2RS_EN_OFFSET: u8 = 22;
    /// ## Bit offset for the `D1RS_EN` field.
    pub const D1RS_EN_OFFSET: u8 = 21;
    /// ## Bit offset for the `D0RS_EN` field.
    pub const D0RS_EN_OFFSET: u8 = 20;
    /// ## Bit offset for the `RO_DS` field.
    pub const RO_DS_OFFSET: u8 = 16;
    /// ## Bit offset for the `CLKO_DS` field.
    pub const CLKO_DS_OFFSET: u8 = 12;
    /// ## Bit offset for the `NRSTO_DS` field.
    pub const NRSTO_DS_OFFSET: u8 = 8;
    /// ## Bit offset for the `BO_DS` field.
    pub const BO_DS_OFFSET: u8 = 4;
    /// ## Bit offset for the `CO_DS` field.
    pub const CO_DS_OFFSET: u8 = 0;

    /// ## Bit mask for the `RF_DS` field.
    pub const RF_DS_MASK: u32 = 0b1111 << Self::RF_DS_OFFSET;
    /// ## Bit mask for the `D3RS_EN` field.
    pub const D3RS_EN_MASK: u32 = 0b1 << Self::D3RS_EN_OFFSET;
    /// ## Bit mask for the `D2RS_EN` field.
    pub const D2RS_EN_MASK: u32 = 0b1 << Self::D2RS_EN_OFFSET;
    /// ## Bit mask for the `D1RS_EN` field.
    pub const D1RS_EN_MASK: u32 = 0b1 << Self::D1RS_EN_OFFSET;
    /// ## Bit mask for the `D0RS_EN` field.
    pub const D0RS_EN_MASK: u32 = 0b1 << Self::D0RS_EN_OFFSET;
    /// ## Bit mask for the `RO_DS` field.
    pub const RO_DS_MASK: u32 = 0b1111 << Self::RO_DS_OFFSET;
    /// ## Bit mask for the `CLKO_DS` field.
    pub const CLKO_DS_MASK: u32 = 0b1111 << Self::CLKO_DS_OFFSET;
    /// ## Bit mask for the `NRSTO_DS` field.
    pub const NRSTO_DS_MASK: u32 = 0b1111 << Self::NRSTO_DS_OFFSET;
    /// ## Bit mask for the `BO_DS` field.
    pub const BO_DS_MASK: u32 = 0b1111 << Self::BO_DS_OFFSET;
    /// ## Bit mask for the `CO_DS` field.
    pub const CO_DS_MASK: u32 = 0b1111 << Self::CO_DS_OFFSET;
}

impl ::core::fmt::Display for IoDriverStrenghtConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IoDriverStrenghtConfiguration").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for IoDriverStrenghtConfiguration {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "IoDriverStrenghtConfiguration {{  }}",);
    }
}

/// # Time Out register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TimeOut(u32);
impl_boilerplate_for!(TimeOut);

impl TimeOut {
    /// ## Time Out register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{TimeOut, Register};
    ///
    /// assert_eq!(TimeOut::ADDR, TimeOut::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x5C;

    /// ## Time Out register reset value.
    pub const RESET: u32 = 0x0000_ffff;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::TimeOut;
    ///
    /// assert_eq!(TimeOut::DEFAULT, TimeOut::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `TMOUT` field.
    pub const TMOUT_OFFSET: u8 = 0;

    /// ## Bit mask for the `TMOUT` field.
    pub const TMOUT_MASK: u32 = 0xffff << Self::TMOUT_OFFSET;
}

impl ::core::fmt::Display for TimeOut {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimeOut").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for TimeOut {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "TimeOut {{  }}",);
    }
}

/// # PLL1 Parameter register
///
/// Used to set PLL1 frequency.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PLL1Parameter(u32);
impl_boilerplate_for!(PLL1Parameter);

impl PLL1Parameter {
    /// ## PLL1 Parameter register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{PLL1Parameter, Register};
    ///
    /// assert_eq!(PLL1Parameter::ADDR, PLL1Parameter::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x60;

    /// ## PLL1 Parameter register reset value.
    pub const RESET: u32 = 0x0064_0111;

    /// ### Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL1Parameter;
    ///
    /// assert_eq!(PLL1Parameter::DEFAULT, PLL1Parameter::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `LOCKED` field.
    pub const LOCKED_OFFSET: u8 = 31;
    /// ## Bit offset for the `PLLEN` field.
    pub const PLLEN_OFFSET: u8 = 30;
    /// ## Bit offset for the `FBDIV` field.
    pub const FBDIV_OFFSET: u8 = 16;
    /// ## Bit offset for the `REFDIV` field.
    pub const REFDIV_OFFSET: u8 = 8;
    /// ## Bit offset for the `POSTDIV1` field.
    pub const POSTDIV1_OFFSET: u8 = 4;
    /// ## Bit offset for the `POSTDIV2` field.
    pub const POSTDIV2_OFFSET: u8 = 0;

    /// ## Bit mask for the `LOCKED` field.
    pub const LOCKED_MASK: u32 = 0x1 << Self::LOCKED_OFFSET;
    /// ## Bit mask for the `PLLEN` field.
    pub const PLLEN_MASK: u32 = 0x1 << Self::PLLEN_OFFSET;
    /// ## Bit mask for the `FBDIV` field.
    pub const FBDIV_MASK: u32 = 0xfff << Self::FBDIV_OFFSET;
    /// ## Bit mask for the `REFDIV` field.
    pub const REFDIV_MASK: u32 = 0x3f << Self::REFDIV_OFFSET;
    /// ## Bit mask for the `POSTDIV1` field.
    pub const POSTDIV1_MASK: u32 = 0x7 << Self::POSTDIV1_OFFSET;
    /// ## Bit mask for the `POSTDIV2` field.
    pub const POSTDIV2_MASK: u32 = 0x7 << Self::POSTDIV2_OFFSET;

    /// ## Get the PLL1 locked state.
    ///
    /// This returns an `bool` with the locked state.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL1Parameter;
    ///
    /// let pll1: PLL1Parameter = PLL1Parameter::DEFAULT;
    /// assert!(!pll1.locked());
    /// let pll1: PLL1Parameter = pll1.lock();
    /// assert!(pll1.locked());
    /// let pll1: PLL1Parameter = pll1.unlock();
    /// assert!(!pll1.locked());
    /// ```
    pub const fn locked(&self) -> bool {
        self.0 & Self::LOCKED_MASK == Self::LOCKED_MASK
    }
    /// ## Lock the PLL1.
    #[must_use = "lock returns a modified PLL1Parameter"]
    pub const fn lock(mut self) -> Self {
        self.0 |= Self::LOCKED_MASK;
        self
    }
    /// ## Disable the PLL1.
    #[must_use = "unlock returns a modified PLL1Parameter"]
    pub const fn unlock(mut self) -> Self {
        self.0 &= !Self::LOCKED_MASK;
        self
    }

    /// ## Get the PLL1 enabled state.
    ///
    /// This returns an `bool` with the PLL1 enabled state.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL1Parameter;
    ///
    /// let pll1: PLL1Parameter = PLL1Parameter::DEFAULT;
    /// assert!(!pll1.enabled());
    /// let pll1: PLL1Parameter = pll1.enable();
    /// assert!(pll1.enabled());
    /// let pll1: PLL1Parameter = pll1.disable();
    /// assert!(!pll1.enabled());
    /// ```
    pub const fn enabled(&self) -> bool {
        self.0 & Self::PLLEN_MASK == Self::PLLEN_MASK
    }
    /// ## Enable the PLL1.
    #[must_use = "enable returns a modified PLL1Parameter"]
    pub const fn enable(mut self) -> Self {
        self.0 |= Self::PLLEN_MASK;
        self
    }
    /// ## Disable the PLL1.
    #[must_use = "disable returns a modified PLL1Parameter"]
    pub const fn disable(mut self) -> Self {
        self.0 &= !Self::PLLEN_MASK;
        self
    }

    /// ## Get the PLL1 FB Divider.
    ///
    /// This returns an `u16` with the PLL1 FB Divider.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL1Parameter;
    ///
    /// let pll1: PLL1Parameter = PLL1Parameter::DEFAULT;
    /// assert_eq!(pll1.fbdiv(), 0x0064);
    /// let pll1: PLL1Parameter = pll1.set_fbdiv(0xAAA);
    /// assert_eq!(pll1.fbdiv(), 0x0AAA);
    /// let pll1: PLL1Parameter = pll1.set_fbdiv(0xF555);
    /// assert_eq!(pll1.fbdiv(), 0x0555);
    /// ```
    pub const fn fbdiv(&self) -> u16 {
        ((self.0 & Self::FBDIV_MASK) >> Self::FBDIV_OFFSET) as u16
    }
    /// ## Set the PLL1 FB Divider.
    #[must_use = "set_fbdiv returns a modified PLL1Parameter"]
    pub const fn set_fbdiv(mut self, fbdiv: u16) -> Self {
        self.0 &= !Self::FBDIV_MASK;
        self.0 |= ((fbdiv as u32) << Self::FBDIV_OFFSET) & Self::FBDIV_MASK;
        self
    }

    /// ## Get the PLL1 REF Divider.
    ///
    /// This returns an `u8` with the PLL1 REF Divider.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL1Parameter;
    ///
    /// let pll1: PLL1Parameter = PLL1Parameter::DEFAULT;
    /// assert_eq!(pll1.refdiv(), 0x01);
    /// let pll1: PLL1Parameter = pll1.set_refdiv(0xAA);
    /// assert_eq!(pll1.refdiv(), 0x2A);
    /// let pll1: PLL1Parameter = pll1.set_refdiv(0xF5);
    /// assert_eq!(pll1.refdiv(), 0x35);
    /// ```
    pub const fn refdiv(&self) -> u8 {
        ((self.0 & Self::REFDIV_MASK) >> Self::REFDIV_OFFSET) as u8
    }
    /// ## Set the PLL1 REF Divider.
    #[must_use = "set_refdiv returns a modified PLL1Parameter"]
    pub const fn set_refdiv(mut self, refdiv: u8) -> Self {
        self.0 &= !Self::REFDIV_MASK;
        self.0 |= ((refdiv as u32) << Self::REFDIV_OFFSET) & Self::REFDIV_MASK;
        self
    }

    /// ## Get the PLL1 POST Divider 1.
    ///
    /// This returns an `u8` with the PLL1 POST Divider 1.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL1Parameter;
    ///
    /// let pll1: PLL1Parameter = PLL1Parameter::DEFAULT;
    /// assert_eq!(pll1.postdiv1(), 0x01);
    /// let pll1: PLL1Parameter = pll1.set_postdiv1(0x07);
    /// assert_eq!(pll1.postdiv1(), 0x07);
    /// let pll1: PLL1Parameter = pll1.set_postdiv1(0xF5);
    /// assert_eq!(pll1.postdiv1(), 0x05);
    /// ```
    pub const fn postdiv1(&self) -> u8 {
        ((self.0 & Self::POSTDIV1_MASK) >> Self::POSTDIV1_OFFSET) as u8
    }
    /// ## Set the PLL1 POST Divider 1.
    #[must_use = "set_postdiv1 returns a modified PLL1Parameter"]
    pub const fn set_postdiv1(mut self, postdiv1: u8) -> Self {
        self.0 &= !Self::POSTDIV1_MASK;
        self.0 |= ((postdiv1 as u32) << Self::POSTDIV1_OFFSET) & Self::POSTDIV1_MASK;
        self
    }

    /// ## Get the PLL1 POST Divider 2.
    ///
    /// This returns an `u8` with the PLL1 POST Divider 2.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL1Parameter;
    ///
    /// let pll1: PLL1Parameter = PLL1Parameter::DEFAULT;
    /// assert_eq!(pll1.postdiv2(), 0x01);
    /// let pll1: PLL1Parameter = pll1.set_postdiv2(0x07);
    /// assert_eq!(pll1.postdiv2(), 0x07);
    /// let pll1: PLL1Parameter = pll1.set_postdiv2(0xF5);
    /// assert_eq!(pll1.postdiv2(), 0x05);
    /// ```
    pub const fn postdiv2(&self) -> u8 {
        ((self.0 & Self::POSTDIV2_MASK) >> Self::POSTDIV2_OFFSET) as u8
    }
    /// ## Set the PLL1 POST Divider 2.
    #[must_use = "set_postdiv2 returns a modified PLL1Parameter"]
    pub const fn set_postdiv2(mut self, postdiv2: u8) -> Self {
        self.0 &= !Self::POSTDIV2_MASK;
        self.0 |= ((postdiv2 as u32) << Self::POSTDIV2_OFFSET) & Self::POSTDIV2_MASK;
        self
    }

    /// ## Get the PLL1 Frequency.
    ///
    /// This returns an `HertzU32` with the PLL1 Frequency according to the clki_freq parameter.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL1Parameter;
    /// use fugit::HertzU32;
    ///
    /// let clki_freq = HertzU32::MHz(25);
    /// assert_eq!(PLL1Parameter::DEFAULT.frequency(clki_freq), HertzU32::MHz(2500u32));
    /// ```
    pub const fn frequency(&self, clki_freq: HertzU32) -> HertzU32 {
        HertzU32::from_raw(
            clki_freq.raw() * (self.fbdiv() as u32)
                / ((self.refdiv() as u32) * (self.postdiv1() as u32) * (self.postdiv2() as u32)),
        )
    }
}

impl ::core::fmt::Display for PLL1Parameter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PLL1Parameter")
            .field("locked", &self.locked())
            .field("enabled", &self.enabled())
            .field("fbdiv", &self.fbdiv())
            .field("refdiv", &self.refdiv())
            .field("postdiv1", &self.postdiv1())
            .field("postdiv2", &self.postdiv2())
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for PLL1Parameter {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "PLL1Parameter {{ locked: {}, enabled: {}, fbdiv: {}, refdiv: {}, postdiv1: {}, postdiv2: {} }}",
            self.locked(),
            self.enabled(),
            self.fbdiv(),
            self.refdiv(),
            self.postdiv1(),
            self.postdiv2(),
        );
    }
}

/// # PLL2 Parameter register
///
/// Used to set PLL2 frequency.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PLL2Parameter(u32);
impl_boilerplate_for!(PLL2Parameter);

impl PLL2Parameter {
    /// ## PLL2 Parameter register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{PLL2Parameter, Register};
    ///
    /// assert_eq!(PLL2Parameter::ADDR, PLL2Parameter::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x64;

    /// ## PLL2 Parameter register reset value.
    pub const RESET: u32 = 0x0068_0111;

    /// ### Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL2Parameter;
    ///
    /// assert_eq!(PLL2Parameter::DEFAULT, PLL2Parameter::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `LOCKED` field.
    pub const LOCKED_OFFSET: u8 = 31;
    /// ## Bit offset for the `PLLEN` field.
    pub const PLLEN_OFFSET: u8 = 30;
    /// ## Bit offset for the `FBDIV` field.
    pub const FBDIV_OFFSET: u8 = 16;
    /// ## Bit offset for the `REFDIV` field.
    pub const REFDIV_OFFSET: u8 = 8;
    /// ## Bit offset for the `POSTDIV1` field.
    pub const POSTDIV1_OFFSET: u8 = 4;
    /// ## Bit offset for the `POSTDIV2` field.
    pub const POSTDIV2_OFFSET: u8 = 0;

    /// ## Bit mask for the `LOCKED` field.
    pub const LOCKED_MASK: u32 = 0x1 << Self::LOCKED_OFFSET;
    /// ## Bit mask for the `PLLEN` field.
    pub const PLLEN_MASK: u32 = 0x1 << Self::PLLEN_OFFSET;
    /// ## Bit mask for the `FBDIV` field.
    pub const FBDIV_MASK: u32 = 0xfff << Self::FBDIV_OFFSET;
    /// ## Bit mask for the `REFDIV` field.
    pub const REFDIV_MASK: u32 = 0x3f << Self::REFDIV_OFFSET;
    /// ## Bit mask for the `POSTDIV1` field.
    pub const POSTDIV1_MASK: u32 = 0x7 << Self::POSTDIV1_OFFSET;
    /// ## Bit mask for the `POSTDIV2` field.
    pub const POSTDIV2_MASK: u32 = 0x7 << Self::POSTDIV2_OFFSET;

    /// ## Get the PLL2 locked state.
    ///
    /// This returns an `bool` with the locked state.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL2Parameter;
    ///
    /// let pll2: PLL2Parameter = PLL2Parameter::DEFAULT;
    /// assert!(!pll2.locked());
    /// let pll2: PLL2Parameter = pll2.lock();
    /// assert!(pll2.locked());
    /// let pll2: PLL2Parameter = pll2.unlock();
    /// assert!(!pll2.locked());
    /// ```
    pub const fn locked(&self) -> bool {
        self.0 & Self::LOCKED_MASK == Self::LOCKED_MASK
    }
    /// ## Lock the PLL2.
    #[must_use = "lock returns a modified PLL2Parameter"]
    pub const fn lock(mut self) -> Self {
        self.0 |= Self::LOCKED_MASK;
        self
    }
    /// ## Disable the PLL2.
    #[must_use = "unlock returns a modified PLL2Parameter"]
    pub const fn unlock(mut self) -> Self {
        self.0 &= !Self::LOCKED_MASK;
        self
    }

    /// ## Get the PLL2 enabled state.
    ///
    /// This returns an `bool` with the PLL2 enabled state.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL2Parameter;
    ///
    /// let pll2: PLL2Parameter = PLL2Parameter::DEFAULT;
    /// assert!(!pll2.enabled());
    /// let pll2: PLL2Parameter = pll2.enable();
    /// assert!(pll2.enabled());
    /// let pll2: PLL2Parameter = pll2.disable();
    /// assert!(!pll2.enabled());
    /// ```
    pub const fn enabled(&self) -> bool {
        self.0 & Self::PLLEN_MASK == Self::PLLEN_MASK
    }
    /// ## Enable the PLL2.
    #[must_use = "enable returns a modified PLL2Parameter"]
    pub const fn enable(mut self) -> Self {
        self.0 |= Self::PLLEN_MASK;
        self
    }
    /// ## Disable the PLL2.
    #[must_use = "disable returns a modified PLL2Parameter"]
    pub const fn disable(mut self) -> Self {
        self.0 &= !Self::PLLEN_MASK;
        self
    }

    /// ## Get the PLL2 FB Divider.
    ///
    /// This returns an `u16` with the PLL2 FB Divider.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL2Parameter;
    ///
    /// let pll2: PLL2Parameter = PLL2Parameter::DEFAULT;
    /// assert_eq!(pll2.fbdiv(), 0x0068);
    /// let pll2: PLL2Parameter = pll2.set_fbdiv(0xAAA);
    /// assert_eq!(pll2.fbdiv(), 0x0AAA);
    /// let pll2: PLL2Parameter = pll2.set_fbdiv(0xF555);
    /// assert_eq!(pll2.fbdiv(), 0x0555);
    /// ```
    pub const fn fbdiv(&self) -> u16 {
        ((self.0 & Self::FBDIV_MASK) >> Self::FBDIV_OFFSET) as u16
    }
    /// ## Set the PLL2 FB Divider.
    #[must_use = "set_fbdiv returns a modified PLL2Parameter"]
    pub const fn set_fbdiv(mut self, fbdiv: u16) -> Self {
        self.0 &= !Self::FBDIV_MASK;
        self.0 |= ((fbdiv as u32) << Self::FBDIV_OFFSET) & Self::FBDIV_MASK;
        self
    }

    /// ## Get the PLL2 REF Divider.
    ///
    /// This returns an `u8` with the PLL2 REF Divider.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL2Parameter;
    ///
    /// let pll2: PLL2Parameter = PLL2Parameter::DEFAULT;
    /// assert_eq!(pll2.refdiv(), 0x01);
    /// let pll2: PLL2Parameter = pll2.set_refdiv(0xAA);
    /// assert_eq!(pll2.refdiv(), 0x2A);
    /// let pll2: PLL2Parameter = pll2.set_refdiv(0xF5);
    /// assert_eq!(pll2.refdiv(), 0x35);
    /// ```
    pub const fn refdiv(&self) -> u8 {
        ((self.0 & Self::REFDIV_MASK) >> Self::REFDIV_OFFSET) as u8
    }
    /// ## Set the PLL2 REF Divider.
    #[must_use = "set_refdiv returns a modified PLL2Parameter"]
    pub const fn set_refdiv(mut self, refdiv: u8) -> Self {
        self.0 &= !Self::REFDIV_MASK;
        self.0 |= ((refdiv as u32) << Self::REFDIV_OFFSET) & Self::REFDIV_MASK;
        self
    }

    /// ## Get the PLL2 POST Divider 1.
    ///
    /// This returns an `u8` with the PLL2 POST Divider 1.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL2Parameter;
    ///
    /// let pll2: PLL2Parameter = PLL2Parameter::DEFAULT;
    /// assert_eq!(pll2.postdiv1(), 0x01);
    /// let pll2: PLL2Parameter = pll2.set_postdiv1(0x07);
    /// assert_eq!(pll2.postdiv1(), 0x07);
    /// let pll2: PLL2Parameter = pll2.set_postdiv1(0xF5);
    /// assert_eq!(pll2.postdiv1(), 0x05);
    /// ```
    pub const fn postdiv1(&self) -> u8 {
        ((self.0 & Self::POSTDIV1_MASK) >> Self::POSTDIV1_OFFSET) as u8
    }
    /// ## Set the PLL2 POST Divider 1.
    #[must_use = "set_postdiv1 returns a modified PLL2Parameter"]
    pub const fn set_postdiv1(mut self, postdiv1: u8) -> Self {
        self.0 &= !Self::POSTDIV1_MASK;
        self.0 |= ((postdiv1 as u32) << Self::POSTDIV1_OFFSET) & Self::POSTDIV1_MASK;
        self
    }

    /// ## Get the PLL2 POST Divider 2.
    ///
    /// This returns an `u8` with the PLL2 POST Divider 2.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL2Parameter;
    ///
    /// let pll2: PLL2Parameter = PLL2Parameter::DEFAULT;
    /// assert_eq!(pll2.postdiv2(), 0x01);
    /// let pll2: PLL2Parameter = pll2.set_postdiv2(0x07);
    /// assert_eq!(pll2.postdiv2(), 0x07);
    /// let pll2: PLL2Parameter = pll2.set_postdiv2(0xF5);
    /// assert_eq!(pll2.postdiv2(), 0x05);
    /// ```
    pub const fn postdiv2(&self) -> u8 {
        ((self.0 & Self::POSTDIV2_MASK) >> Self::POSTDIV2_OFFSET) as u8
    }
    /// ## Set the PLL2 POST Divider 2.
    #[must_use = "set_postdiv2 returns a modified PLL2Parameter"]
    pub const fn set_postdiv2(mut self, postdiv2: u8) -> Self {
        self.0 &= !Self::POSTDIV2_MASK;
        self.0 |= ((postdiv2 as u32) << Self::POSTDIV2_OFFSET) & Self::POSTDIV2_MASK;
        self
    }

    /// ## Get the PLL2 Frequency.
    ///
    /// This returns an `HertzU32` with the PLL2 Frequency according to the clki_freq parameter.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL2Parameter;
    /// use fugit::HertzU32;
    ///
    /// let clki_freq = HertzU32::MHz(25);
    /// assert_eq!(PLL2Parameter::DEFAULT.frequency(clki_freq), HertzU32::MHz(2600u32));
    /// ```
    pub const fn frequency(&self, clki_freq: HertzU32) -> HertzU32 {
        HertzU32::from_raw(
            clki_freq.raw() * (self.fbdiv() as u32)
                / ((self.refdiv() as u32) * (self.postdiv1() as u32) * (self.postdiv2() as u32)),
        )
    }
}

impl ::core::fmt::Display for PLL2Parameter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PLL2Parameter")
            .field("locked", &self.locked())
            .field("enabled", &self.enabled())
            .field("fbdiv", &self.fbdiv())
            .field("refdiv", &self.refdiv())
            .field("postdiv1", &self.postdiv1())
            .field("postdiv2", &self.postdiv2())
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for PLL2Parameter {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "PLL2Parameter {{ locked: {}, enabled: {}, fbdiv: {}, refdiv: {}, postdiv1: {}, postdiv2: {} }}",
            self.locked(),
            self.enabled(),
            self.fbdiv(),
            self.refdiv(),
            self.postdiv1(),
            self.postdiv2(),
        );
    }
}

/// # PLL3 Parameter register
///
/// Used to set PLL3 frequency.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PLL3Parameter(u32);
impl_boilerplate_for!(PLL3Parameter);

impl PLL3Parameter {
    /// ## PLL3 Parameter register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{PLL3Parameter, Register};
    ///
    /// assert_eq!(PLL3Parameter::ADDR, PLL3Parameter::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x68;

    /// ## PLL3 Parameter register reset value.
    pub const RESET: u32 = 0x0070_0111;

    /// ### Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL3Parameter;
    ///
    /// assert_eq!(PLL3Parameter::DEFAULT, PLL3Parameter::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `LOCKED` field.
    pub const LOCKED_OFFSET: u8 = 31;
    /// ## Bit offset for the `PLLEN` field.
    pub const PLLEN_OFFSET: u8 = 30;
    /// ## Bit offset for the `FBDIV` field.
    pub const FBDIV_OFFSET: u8 = 16;
    /// ## Bit offset for the `REFDIV` field.
    pub const REFDIV_OFFSET: u8 = 8;
    /// ## Bit offset for the `POSTDIV1` field.
    pub const POSTDIV1_OFFSET: u8 = 4;
    /// ## Bit offset for the `POSTDIV2` field.
    pub const POSTDIV2_OFFSET: u8 = 0;

    /// ## Bit mask for the `LOCKED` field.
    pub const LOCKED_MASK: u32 = 0x1 << Self::LOCKED_OFFSET;
    /// ## Bit mask for the `PLLEN` field.
    pub const PLLEN_MASK: u32 = 0x1 << Self::PLLEN_OFFSET;
    /// ## Bit mask for the `FBDIV` field.
    pub const FBDIV_MASK: u32 = 0xfff << Self::FBDIV_OFFSET;
    /// ## Bit mask for the `REFDIV` field.
    pub const REFDIV_MASK: u32 = 0x3f << Self::REFDIV_OFFSET;
    /// ## Bit mask for the `POSTDIV1` field.
    pub const POSTDIV1_MASK: u32 = 0x7 << Self::POSTDIV1_OFFSET;
    /// ## Bit mask for the `POSTDIV2` field.
    pub const POSTDIV2_MASK: u32 = 0x7 << Self::POSTDIV2_OFFSET;

    /// ## Get the PLL3 locked state.
    ///
    /// This returns an `bool` with the locked state.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL3Parameter;
    ///
    /// let pll3: PLL3Parameter = PLL3Parameter::DEFAULT;
    /// assert!(!pll3.locked());
    /// let pll3: PLL3Parameter = pll3.lock();
    /// assert!(pll3.locked());
    /// let pll3: PLL3Parameter = pll3.unlock();
    /// assert!(!pll3.locked());
    /// ```
    pub const fn locked(&self) -> bool {
        self.0 & Self::LOCKED_MASK == Self::LOCKED_MASK
    }
    /// ## Lock the PLL3.
    #[must_use = "lock returns a modified PLL3Parameter"]
    pub const fn lock(mut self) -> Self {
        self.0 |= Self::LOCKED_MASK;
        self
    }
    /// ## Disable the PLL3.
    #[must_use = "unlock returns a modified PLL3Parameter"]
    pub const fn unlock(mut self) -> Self {
        self.0 &= !Self::LOCKED_MASK;
        self
    }

    /// ## Get the PLL3 enabled state.
    ///
    /// This returns an `bool` with the PLL3 enabled state.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL3Parameter;
    ///
    /// let pll3: PLL3Parameter = PLL3Parameter::DEFAULT;
    /// assert!(!pll3.enabled());
    /// let pll3: PLL3Parameter = pll3.enable();
    /// assert!(pll3.enabled());
    /// let pll3: PLL3Parameter = pll3.disable();
    /// assert!(!pll3.enabled());
    /// ```
    pub const fn enabled(&self) -> bool {
        self.0 & Self::PLLEN_MASK == Self::PLLEN_MASK
    }
    /// ## Enable the PLL3.
    #[must_use = "enable returns a modified PLL3Parameter"]
    pub const fn enable(mut self) -> Self {
        self.0 |= Self::PLLEN_MASK;
        self
    }
    /// ## Disable the PLL3.
    #[must_use = "disable returns a modified PLL3Parameter"]
    pub const fn disable(mut self) -> Self {
        self.0 &= !Self::PLLEN_MASK;
        self
    }

    /// ## Get the PLL3 FB Divider.
    ///
    /// This returns an `u16` with the PLL3 FB Divider.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL3Parameter;
    ///
    /// let pll3: PLL3Parameter = PLL3Parameter::DEFAULT;
    /// assert_eq!(pll3.fbdiv(), 0x0070);
    /// let pll3: PLL3Parameter = pll3.set_fbdiv(0xAAA);
    /// assert_eq!(pll3.fbdiv(), 0x0AAA);
    /// let pll3: PLL3Parameter = pll3.set_fbdiv(0xF555);
    /// assert_eq!(pll3.fbdiv(), 0x0555);
    /// ```
    pub const fn fbdiv(&self) -> u16 {
        ((self.0 & Self::FBDIV_MASK) >> Self::FBDIV_OFFSET) as u16
    }
    /// ## Set the PLL3 FB Divider.
    #[must_use = "set_fbdiv returns a modified PLL3Parameter"]
    pub const fn set_fbdiv(mut self, fbdiv: u16) -> Self {
        self.0 &= !Self::FBDIV_MASK;
        self.0 |= ((fbdiv as u32) << Self::FBDIV_OFFSET) & Self::FBDIV_MASK;
        self
    }

    /// ## Get the PLL3 REF Divider.
    ///
    /// This returns an `u8` with the PLL3 REF Divider.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL3Parameter;
    ///
    /// let pll3: PLL3Parameter = PLL3Parameter::DEFAULT;
    /// assert_eq!(pll3.refdiv(), 0x01);
    /// let pll3: PLL3Parameter = pll3.set_refdiv(0xAA);
    /// assert_eq!(pll3.refdiv(), 0x2A);
    /// let pll3: PLL3Parameter = pll3.set_refdiv(0xF5);
    /// assert_eq!(pll3.refdiv(), 0x35);
    /// ```
    pub const fn refdiv(&self) -> u8 {
        ((self.0 & Self::REFDIV_MASK) >> Self::REFDIV_OFFSET) as u8
    }
    /// ## Set the PLL3 REF Divider.
    #[must_use = "set_refdiv returns a modified PLL3Parameter"]
    pub const fn set_refdiv(mut self, refdiv: u8) -> Self {
        self.0 &= !Self::REFDIV_MASK;
        self.0 |= ((refdiv as u32) << Self::REFDIV_OFFSET) & Self::REFDIV_MASK;
        self
    }

    /// ## Get the PLL3 POST Divider 1.
    ///
    /// This returns an `u8` with the PLL3 POST Divider 1.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL3Parameter;
    ///
    /// let pll3: PLL3Parameter = PLL3Parameter::DEFAULT;
    /// assert_eq!(pll3.postdiv1(), 0x01);
    /// let pll3: PLL3Parameter = pll3.set_postdiv1(0x07);
    /// assert_eq!(pll3.postdiv1(), 0x07);
    /// let pll3: PLL3Parameter = pll3.set_postdiv1(0xF5);
    /// assert_eq!(pll3.postdiv1(), 0x05);
    /// ```
    pub const fn postdiv1(&self) -> u8 {
        ((self.0 & Self::POSTDIV1_MASK) >> Self::POSTDIV1_OFFSET) as u8
    }
    /// ## Set the PLL3 POST Divider 1.
    #[must_use = "set_postdiv1 returns a modified PLL3Parameter"]
    pub const fn set_postdiv1(mut self, postdiv1: u8) -> Self {
        self.0 &= !Self::POSTDIV1_MASK;
        self.0 |= ((postdiv1 as u32) << Self::POSTDIV1_OFFSET) & Self::POSTDIV1_MASK;
        self
    }

    /// ## Get the PLL3 POST Divider 2.
    ///
    /// This returns an `u8` with the PLL3 POST Divider 2.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL3Parameter;
    ///
    /// let pll3: PLL3Parameter = PLL3Parameter::DEFAULT;
    /// assert_eq!(pll3.postdiv2(), 0x01);
    /// let pll3: PLL3Parameter = pll3.set_postdiv2(0x07);
    /// assert_eq!(pll3.postdiv2(), 0x07);
    /// let pll3: PLL3Parameter = pll3.set_postdiv2(0xF5);
    /// assert_eq!(pll3.postdiv2(), 0x05);
    /// ```
    pub const fn postdiv2(&self) -> u8 {
        ((self.0 & Self::POSTDIV2_MASK) >> Self::POSTDIV2_OFFSET) as u8
    }
    /// ## Set the PLL3 POST Divider 2.
    #[must_use = "set_postdiv2 returns a modified PLL3Parameter"]
    pub const fn set_postdiv2(mut self, postdiv2: u8) -> Self {
        self.0 &= !Self::POSTDIV2_MASK;
        self.0 |= ((postdiv2 as u32) << Self::POSTDIV2_OFFSET) & Self::POSTDIV2_MASK;
        self
    }

    /// ## Get the PLL3 Frequency.
    ///
    /// This returns an `HertzU32` with the PLL3 Frequency according to the clki_freq parameter.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL3Parameter;
    /// use fugit::HertzU32;
    ///
    /// let clki_freq = HertzU32::MHz(25);
    /// assert_eq!(PLL3Parameter::DEFAULT.frequency(clki_freq), HertzU32::MHz(2800u32));
    /// ```
    pub const fn frequency(&self, clki_freq: HertzU32) -> HertzU32 {
        HertzU32::from_raw(
            clki_freq.raw() * (self.fbdiv() as u32)
                / ((self.refdiv() as u32) * (self.postdiv1() as u32) * (self.postdiv2() as u32)),
        )
    }
}

impl ::core::fmt::Display for PLL3Parameter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PLL3Parameter")
            .field("locked", &self.locked())
            .field("enabled", &self.enabled())
            .field("fbdiv", &self.fbdiv())
            .field("refdiv", &self.refdiv())
            .field("postdiv1", &self.postdiv1())
            .field("postdiv2", &self.postdiv2())
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for PLL3Parameter {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "PLL3Parameter {{ locked: {}, enabled: {}, fbdiv: {}, refdiv: {}, postdiv1: {}, postdiv2: {} }}",
            self.locked(),
            self.enabled(),
            self.fbdiv(),
            self.refdiv(),
            self.postdiv1(),
            self.postdiv2(),
        );
    }
}

/// # Ordered Clock Monitor register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct OrderedClockMonitor(u32);
impl_boilerplate_for!(OrderedClockMonitor);

impl OrderedClockMonitor {
    /// ## Ordered Clock Monitor register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{OrderedClockMonitor, Register};
    ///
    /// assert_eq!(OrderedClockMonitor::ADDR, OrderedClockMonitor::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x6C;

    /// ## Ordered Clock Monitor register reset value.
    pub const RESET: u32 = 0x0000_0000;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::OrderedClockMonitor;
    ///
    /// assert_eq!(OrderedClockMonitor::DEFAULT, OrderedClockMonitor::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `START` field.
    pub const START_OFFSET: u8 = 31;
    /// ## Bit offset for the `CLK_SEL` field.
    pub const CLK_SEL_OFFSET: u8 = 24;
    /// ## Bit offset for the `CLK_COUNT` field.
    pub const CLK_COUNT_OFFSET: u8 = 0;

    /// ## Bit mask for the `START` field.
    pub const START_MASK: u32 = 0b1 << Self::START_OFFSET;
    /// ## Bit mask for the `CLK_SEL` field.
    pub const CLK_SEL_MASK: u32 = 0b1111 << Self::CLK_SEL_OFFSET;
    /// ## Bit mask for the `CLK_COUNT` field.
    pub const CLK_COUNT_MASK: u32 = 0xffff << Self::CLK_COUNT_OFFSET;
}

impl ::core::fmt::Display for OrderedClockMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OrderedClockMonitor").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for OrderedClockMonitor {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "OrderedClockMonitor {{  }}",);
    }
}

/// # PLL0 Divider register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PLL0Divider(u32);
impl_boilerplate_for!(PLL0Divider);

impl PLL0Divider {
    /// ## PLL0 Divider register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{PLL0Divider, Register};
    ///
    /// assert_eq!(PLL0Divider::ADDR, PLL0Divider::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x70;

    /// ## PLL0 Divider register reset value.
    pub const RESET: u32 = 0x0304_0607;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL0Divider;
    ///
    /// assert_eq!(PLL0Divider::DEFAULT, PLL0Divider::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `PLLDIV3` field.
    pub const PLLDIV3_OFFSET: u8 = 24;
    /// ## Bit offset for the `PLLDIV2` field.
    pub const PLLDIV2_OFFSET: u8 = 16;
    /// ## Bit offset for the `PLLDIV1` field.
    pub const PLLDIV1_OFFSET: u8 = 8;
    /// ## Bit offset for the `PLLDIV0` field.
    pub const PLLDIV0_OFFSET: u8 = 0;

    /// ## Bit mask for the `PLLDIV3` field.
    pub const PLLDIV3_MASK: u32 = 0b1111 << Self::PLLDIV3_OFFSET;
    /// ## Bit mask for the `PLLDIV2` field.
    pub const PLLDIV2_MASK: u32 = 0b1111 << Self::PLLDIV2_OFFSET;
    /// ## Bit mask for the `PLLDIV1` field.
    pub const PLLDIV1_MASK: u32 = 0b1111 << Self::PLLDIV1_OFFSET;
    /// ## Bit mask for the `PLLDIV0` field.
    pub const PLLDIV0_MASK: u32 = 0b1111 << Self::PLLDIV0_OFFSET;
}

impl ::core::fmt::Display for PLL0Divider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PLL0Divider").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for PLL0Divider {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "PLL0Divider {{  }}",);
    }
}

/// # PLL1 Divider register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PLL1Divider(u32);
impl_boilerplate_for!(PLL1Divider);

impl PLL1Divider {
    /// ## PLL1 Divider register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{PLL1Divider, Register};
    ///
    /// assert_eq!(PLL1Divider::ADDR, PLL1Divider::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x74;

    /// ## PLL1 Divider register reset value.
    pub const RESET: u32 = 0x0304_0506;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL1Divider;
    ///
    /// assert_eq!(PLL1Divider::DEFAULT, PLL1Divider::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `PLLDIV3` field.
    pub const PLLDIV3_OFFSET: u8 = 24;
    /// ## Bit offset for the `PLLDIV2` field.
    pub const PLLDIV2_OFFSET: u8 = 16;
    /// ## Bit offset for the `PLLDIV1` field.
    pub const PLLDIV1_OFFSET: u8 = 8;
    /// ## Bit offset for the `PLLDIV0` field.
    pub const PLLDIV0_OFFSET: u8 = 0;

    /// ## Bit mask for the `PLLDIV3` field.
    pub const PLLDIV3_MASK: u32 = 0b1111 << Self::PLLDIV3_OFFSET;
    /// ## Bit mask for the `PLLDIV2` field.
    pub const PLLDIV2_MASK: u32 = 0b1111 << Self::PLLDIV2_OFFSET;
    /// ## Bit mask for the `PLLDIV1` field.
    pub const PLLDIV1_MASK: u32 = 0b1111 << Self::PLLDIV1_OFFSET;
    /// ## Bit mask for the `PLLDIV0` field.
    pub const PLLDIV0_MASK: u32 = 0b1111 << Self::PLLDIV0_OFFSET;
}

impl ::core::fmt::Display for PLL1Divider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PLL1Divider").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for PLL1Divider {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "PLL1Divider {{  }}",);
    }
}

/// # PLL2 Divider register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PLL2Divider(u32);
impl_boilerplate_for!(PLL2Divider);

impl PLL2Divider {
    /// ## PLL2 Divider register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{PLL2Divider, Register};
    ///
    /// assert_eq!(PLL2Divider::ADDR, PLL2Divider::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x78;

    /// ## PLL2 Divider register reset value.
    pub const RESET: u32 = 0x0304_0506;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL2Divider;
    ///
    /// assert_eq!(PLL2Divider::DEFAULT, PLL2Divider::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `PLLDIV3` field.
    pub const PLLDIV3_OFFSET: u8 = 24;
    /// ## Bit offset for the `PLLDIV2` field.
    pub const PLLDIV2_OFFSET: u8 = 16;
    /// ## Bit offset for the `PLLDIV1` field.
    pub const PLLDIV1_OFFSET: u8 = 8;
    /// ## Bit offset for the `PLLDIV0` field.
    pub const PLLDIV0_OFFSET: u8 = 0;

    /// ## Bit mask for the `PLLDIV3` field.
    pub const PLLDIV3_MASK: u32 = 0b1111 << Self::PLLDIV3_OFFSET;
    /// ## Bit mask for the `PLLDIV2` field.
    pub const PLLDIV2_MASK: u32 = 0b1111 << Self::PLLDIV2_OFFSET;
    /// ## Bit mask for the `PLLDIV1` field.
    pub const PLLDIV1_MASK: u32 = 0b1111 << Self::PLLDIV1_OFFSET;
    /// ## Bit mask for the `PLLDIV0` field.
    pub const PLLDIV0_MASK: u32 = 0b1111 << Self::PLLDIV0_OFFSET;
}

impl ::core::fmt::Display for PLL2Divider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PLL2Divider").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for PLL2Divider {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "PLL2Divider {{  }}",);
    }
}

/// # PLL3 Divider register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PLL3Divider(u32);
impl_boilerplate_for!(PLL3Divider);

impl PLL3Divider {
    /// ## PLL3 Divider register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{PLL3Divider, Register};
    ///
    /// assert_eq!(PLL3Divider::ADDR, PLL3Divider::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x7C;

    /// ## PLL3 Divider register reset value.
    pub const RESET: u32 = 0x0304_0506;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::PLL3Divider;
    ///
    /// assert_eq!(PLL3Divider::DEFAULT, PLL3Divider::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `PLLDIV3` field.
    pub const PLLDIV3_OFFSET: u8 = 24;
    /// ## Bit offset for the `PLLDIV2` field.
    pub const PLLDIV2_OFFSET: u8 = 16;
    /// ## Bit offset for the `PLLDIV1` field.
    pub const PLLDIV1_OFFSET: u8 = 8;
    /// ## Bit offset for the `PLLDIV0` field.
    pub const PLLDIV0_OFFSET: u8 = 0;

    /// ## Bit mask for the `PLLDIV3` field.
    pub const PLLDIV3_MASK: u32 = 0b1111 << Self::PLLDIV3_OFFSET;
    /// ## Bit mask for the `PLLDIV2` field.
    pub const PLLDIV2_MASK: u32 = 0b1111 << Self::PLLDIV2_OFFSET;
    /// ## Bit mask for the `PLLDIV1` field.
    pub const PLLDIV1_MASK: u32 = 0b1111 << Self::PLLDIV1_OFFSET;
    /// ## Bit mask for the `PLLDIV0` field.
    pub const PLLDIV0_MASK: u32 = 0b1111 << Self::PLLDIV0_OFFSET;
}

impl ::core::fmt::Display for PLL3Divider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PLL3Divider").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for PLL3Divider {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "PLL3Divider {{  }}",);
    }
}

/// # Clock Order Control 0 register
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ClockOrderControl0(u32);
impl_boilerplate_for!(ClockOrderControl0);

impl ClockOrderControl0 {
    /// ## Clock Order Control 0 register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{ClockOrderControl0, Register};
    ///
    /// assert_eq!(ClockOrderControl0::ADDR, ClockOrderControl0::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x80;

    /// ## Reset value of the socket mode register.
    pub const RESET: u32 = 0xD95C_8410;

    /// ### Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::ClockOrderControl0;
    ///
    /// assert_eq!(ClockOrderControl0::DEFAULT, ClockOrderControl0::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit length for a `CLKN_SEL` field.
    pub const CLKN_SEL_LENGTH: u8 = 4;

    /// ## Bit mask for a `CLKN_SEL` field.
    pub const CLKN_SEL_MASK: u32 = 0xF;

    /// ## Get the clock select.
    ///
    /// This returns an `Err(u8)` with the clock select bits if the clock select bits
    /// do not match a valid clock select.
    ///
    /// ### Example
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
    /// ## Set the clock select.
    ///
    /// ### Example
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

/// # Clock Order Control 1 register
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ClockOrderControl1(u32);
impl_boilerplate_for!(ClockOrderControl1);

impl ClockOrderControl1 {
    /// ## Clock Order Control 1 register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{ClockOrderControl1, Register};
    ///
    /// assert_eq!(ClockOrderControl1::ADDR, ClockOrderControl1::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x84;

    /// ## Reset value of the socket mode register.
    pub const RESET: u32 = 0xFB73_EA62;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::ClockOrderControl1;
    ///
    /// assert_eq!(ClockOrderControl1::DEFAULT, ClockOrderControl1::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit length for a `CLKN_SEL` field.
    pub const CLKN_SEL_LENGTH: u8 = 4;

    /// ## Bit mask for a `CLKN_SEL` field.
    pub const CLKN_SEL_MASK: u32 = 0xF;

    /// ## Get the clock select.
    ///
    /// This returns an `Err(u8)` with the clock select bits if the clock select bits
    /// do not match a valid clock select.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::{specifier::ClockSelect, register::ClockOrderControl1};
    ///
    /// let clk_ord_ctrl: ClockOrderControl1 = ClockOrderControl1::DEFAULT;
    /// assert_eq!(clk_ord_ctrl.clock_select(0), ClockSelect::from_raw(0x2));
    /// ```
    pub const fn clock_select(&self, clock: u8) -> Result<ClockSelect, u8> {
        if clock > 7 {
            return Err(clock);
        }
        ClockSelect::from_raw(
            ((self.0 >> (clock * Self::CLKN_SEL_LENGTH)) & Self::CLKN_SEL_MASK) as u8,
        )
    }

    /// ## Set the clock select.
    ///
    /// ### Example
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

/// # Clock Order Status register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ClockOrderStatus(u32);
impl_boilerplate_for!(ClockOrderStatus);

impl ClockOrderStatus {
    /// ## Clock Order Status register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{ClockOrderStatus, Register};
    ///
    /// assert_eq!(ClockOrderStatus::ADDR, ClockOrderStatus::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x8C;

    /// ## Clock Order Status register reset value.
    pub const RESET: u32 = 0x0000_0000;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::ClockOrderStatus;
    ///
    /// assert_eq!(ClockOrderStatus::DEFAULT, ClockOrderStatus::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `CLOK_ORDER_STATUS` field.
    pub const CLOK_ORDER_STATUS_OFFSET: u8 = 0;

    /// ## Bit mask for the `CLOK_ORDER_STATUS` field.
    pub const CLOK_ORDER_STATUS_MASK: u32 = 0xffff_ffff << Self::CLOK_ORDER_STATUS_OFFSET;
}

impl ::core::fmt::Display for ClockOrderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ClockOrderStatus").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ClockOrderStatus {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ClockOrderStatus {{  }}",);
    }
}

/// # Frequency Sweep Control 1 register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FrequencySweepControl1(u32);
impl_boilerplate_for!(FrequencySweepControl1);

impl FrequencySweepControl1 {
    /// ## Frequency Sweep Control 1 register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{FrequencySweepControl1, Register};
    ///
    /// assert_eq!(FrequencySweepControl1::ADDR, FrequencySweepControl1::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x90;

    /// ## Frequency Sweep Control 1 register reset value.
    pub const RESET: u32 = 0x0000_0070;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::FrequencySweepControl1;
    ///
    /// assert_eq!(FrequencySweepControl1::DEFAULT, FrequencySweepControl1::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `SWEEP_STATE` field.
    pub const SWEEP_STATE_OFFSET: u8 = 24;

    /// ## Bit mask for the `SWEEP_STATE` field.
    pub const SWEEP_STATE_MASK: u32 = 0b111 << Self::SWEEP_STATE_OFFSET;
}

impl ::core::fmt::Display for FrequencySweepControl1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FrequencySweepControl1").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for FrequencySweepControl1 {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "FrequencySweepControl1 {{  }}",);
    }
}

/// # Golden Nonce For Sweep Return register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GoldenNonceForSweepReturn(u32);
impl_boilerplate_for!(GoldenNonceForSweepReturn);

impl GoldenNonceForSweepReturn {
    /// ## Golden Nonce For Sweep Return register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{GoldenNonceForSweepReturn, Register};
    ///
    /// assert_eq!(GoldenNonceForSweepReturn::ADDR, GoldenNonceForSweepReturn::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x94;

    /// ## Golden Nonce For Sweep Return register reset value.
    pub const RESET: u32 = 0x0037_6400;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::GoldenNonceForSweepReturn;
    ///
    /// assert_eq!(GoldenNonceForSweepReturn::DEFAULT, GoldenNonceForSweepReturn::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `GNOSWR` field.
    pub const GNOSWR_OFFSET: u8 = 0;

    /// ## Bit mask for the `GNOSWR` field.
    pub const GNOSWR_MASK: u32 = 0xffff_ffff << Self::GNOSWR_OFFSET;
}

impl ::core::fmt::Display for GoldenNonceForSweepReturn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GoldenNonceForSweepReturn").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for GoldenNonceForSweepReturn {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "GoldenNonceForSweepReturn {{  }}",);
    }
}

/// # Returned Group Pattern Status register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ReturnedGroupPatternStatus(u32);
impl_boilerplate_for!(ReturnedGroupPatternStatus);

impl ReturnedGroupPatternStatus {
    /// ## Returned Group Pattern Status register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{ReturnedGroupPatternStatus, Register};
    ///
    /// assert_eq!(ReturnedGroupPatternStatus::ADDR, ReturnedGroupPatternStatus::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x98;

    /// ## Returned Group Pattern Status register reset value.
    pub const RESET: u32 = 0x3030_3030;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::ReturnedGroupPatternStatus;
    ///
    /// assert_eq!(ReturnedGroupPatternStatus::DEFAULT, ReturnedGroupPatternStatus::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `RGPS3` field.
    pub const RGPS3_OFFSET: u8 = 24;
    /// ## Bit offset for the `RGPS2` field.
    pub const RGPS2_OFFSET: u8 = 16;
    /// ## Bit offset for the `RGPS1` field.
    pub const RGPS1_OFFSET: u8 = 8;
    /// ## Bit offset for the `RGPS0` field.
    pub const RGPS0_OFFSET: u8 = 0;

    /// ## Bit mask for the `RGPS3` field.
    pub const RGPS3_MASK: u32 = 0b1111 << Self::RGPS3_OFFSET;
    /// ## Bit mask for the `RGPS2` field.
    pub const RGPS2_MASK: u32 = 0b1111 << Self::RGPS2_OFFSET;
    /// ## Bit mask for the `RGPS1` field.
    pub const RGPS1_MASK: u32 = 0b1111 << Self::RGPS1_OFFSET;
    /// ## Bit mask for the `RGPS0` field.
    pub const RGPS0_MASK: u32 = 0b1111 << Self::RGPS0_OFFSET;
}

impl ::core::fmt::Display for ReturnedGroupPatternStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ReturnedGroupPatternStatus").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ReturnedGroupPatternStatus {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ReturnedGroupPatternStatus {{  }}",);
    }
}

/// # Nonce Returned Timeout register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct NonceReturnedTimeout(u32);
impl_boilerplate_for!(NonceReturnedTimeout);

impl NonceReturnedTimeout {
    /// ## Nonce Returned Timeout register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{NonceReturnedTimeout, Register};
    ///
    /// assert_eq!(NonceReturnedTimeout::ADDR, NonceReturnedTimeout::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x9C;

    /// ## Nonce Returned Timeout register reset value.
    pub const RESET: u32 = 0x0000_ffff;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::NonceReturnedTimeout;
    ///
    /// assert_eq!(NonceReturnedTimeout::DEFAULT, NonceReturnedTimeout::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `SWEEP_TIMEOUT` field.
    pub const SWEEP_TIMEOUT_OFFSET: u8 = 0;

    /// ## Bit mask for the `SWEEP_TIMEOUT` field.
    pub const SWEEP_TIMEOUT_MASK: u32 = 0xffff << Self::SWEEP_TIMEOUT_OFFSET;
}

impl ::core::fmt::Display for NonceReturnedTimeout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NonceReturnedTimeout").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for NonceReturnedTimeout {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "NonceReturnedTimeout {{  }}",);
    }
}

/// # Returned Single Pattern Status register
///
/// Used to identify chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ReturnedSinglePatternStatus(u32);
impl_boilerplate_for!(ReturnedSinglePatternStatus);

impl ReturnedSinglePatternStatus {
    /// ## Returned Single Pattern Status register address.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::{ReturnedSinglePatternStatus, Register};
    ///
    /// assert_eq!(ReturnedSinglePatternStatus::ADDR, ReturnedSinglePatternStatus::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0xA0;

    /// ## Returned Single Pattern Status register reset value.
    pub const RESET: u32 = 0x0000_0000;

    /// ## Default value.
    ///
    /// This is the same as `default`, but as a `const` value.
    ///
    /// ### Example
    ///
    /// ```
    /// use bm1397_protocol::register::ReturnedSinglePatternStatus;
    ///
    /// assert_eq!(ReturnedSinglePatternStatus::DEFAULT, ReturnedSinglePatternStatus::default());
    /// ```
    pub const DEFAULT: Self = Self(Self::RESET);

    /// ## Bit offset for the `RSPS` field.
    pub const RSPS_OFFSET: u8 = 0;

    /// ## Bit mask for the `RSPS` field.
    pub const RSPS_MASK: u32 = 0xffff_ffff << Self::RSPS_OFFSET;
}

impl ::core::fmt::Display for ReturnedSinglePatternStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ReturnedSinglePatternStatus").finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ReturnedSinglePatternStatus {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ReturnedSinglePatternStatus {{  }}",);
    }
}

#[derive(Debug, PartialEq)]
pub enum Registers {
    ChipAddress(ChipAddress),
    HashRate(HashRate),
    PLL0Parameter(PLL0Parameter),
    ChipNonceOffset(ChipNonceOffset),
    HashCountingNumber(HashCountingNumber),
    TicketMask(TicketMask),
    MiscControl(MiscControl),
    I2CControl(I2CControl),
    OrderedClockEnable(OrderedClockEnable),
    FastUARTConfiguration(FastUARTConfiguration),
    UARTRelay(UARTRelay),
    TicketMask2(TicketMask2),
    CoreRegisterControl(CoreRegisterControl),
    CoreRegisterValue(CoreRegisterValue),
    ExternalTemperatureSensorRead(ExternalTemperatureSensorRead),
    ErrorFlag(ErrorFlag),
    NonceErrorCounter(NonceErrorCounter),
    NonceOverflowCounter(NonceOverflowCounter),
    AnalogMuxControl(AnalogMuxControl),
    IoDriverStrenghtConfiguration(IoDriverStrenghtConfiguration),
    TimeOut(TimeOut),
    PLL1Parameter(PLL1Parameter),
    PLL2Parameter(PLL2Parameter),
    PLL3Parameter(PLL3Parameter),
    OrderedClockMonitor(OrderedClockMonitor),
    PLL0Divider(PLL0Divider),
    PLL1Divider(PLL1Divider),
    PLL2Divider(PLL2Divider),
    PLL3Divider(PLL3Divider),
    ClockOrderControl0(ClockOrderControl0),
    ClockOrderControl1(ClockOrderControl1),
    ClockOrderStatus(ClockOrderStatus),
    FrequencySweepControl1(FrequencySweepControl1),
    GoldenNonceForSweepReturn(GoldenNonceForSweepReturn),
    ReturnedGroupPatternStatus(ReturnedGroupPatternStatus),
    NonceReturnedTimeout(NonceReturnedTimeout),
    ReturnedSinglePatternStatus(ReturnedSinglePatternStatus),
}
