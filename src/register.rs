//! BM1397 Registers structures.

use crate::specifier::ClockSelect;

use fugit::HertzU32;
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
    /// use bm1397_protocol::register::{PLL0Parameter, RegAddress};
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
            "PLL0Parameter {{ locked: {}, enabled: {}, fbdiv: {}, refdiv: {}, postdiv1: {}, postdiv2: {}, frequency: {} }}",
            self.locked(),
            self.enabled(),
            self.fbdiv(),
            self.refdiv(),
            self.postdiv1(),
            self.postdiv2(),
        );
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
    /// use bm1397_protocol::register::{PLL1Parameter, RegAddress};
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
            "PLL1Parameter {{ locked: {}, enabled: {}, fbdiv: {}, refdiv: {}, postdiv1: {}, postdiv2: {}, frequency: {} }}",
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
    /// use bm1397_protocol::register::{PLL2Parameter, RegAddress};
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
            "PLL2Parameter {{ locked: {}, enabled: {}, fbdiv: {}, refdiv: {}, postdiv1: {}, postdiv2: {}, frequency: {} }}",
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
    /// use bm1397_protocol::register::{PLL3Parameter, RegAddress};
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
            "PLL3Parameter {{ locked: {}, enabled: {}, fbdiv: {}, refdiv: {}, postdiv1: {}, postdiv2: {}, frequency: {} }}",
            self.locked(),
            self.enabled(),
            self.fbdiv(),
            self.refdiv(),
            self.postdiv1(),
            self.postdiv2(),
        );
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
    /// use bm1397_protocol::register::{ClockOrderControl0, RegAddress};
    ///
    /// assert_eq!(ClockOrderControl0::ADDR, ClockOrderControl0::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x80;

    /// ## Reset value of the socket mode register.
    pub const RESET: u32 = 0x0000_0000;

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
    /// use bm1397_protocol::register::{ClockOrderControl1, RegAddress};
    ///
    /// assert_eq!(ClockOrderControl1::ADDR, ClockOrderControl1::DEFAULT.addr());
    /// ```
    pub const ADDR: u8 = 0x84;

    /// ## Reset value of the socket mode register.
    pub const RESET: u32 = 0x0000_0000;

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
    PLL0Parameter(PLL0Parameter),
    PLL1Parameter(PLL1Parameter),
    PLL2Parameter(PLL2Parameter),
    PLL3Parameter(PLL3Parameter),
    ClockOrderControl0(ClockOrderControl0),
    ClockOrderControl1(ClockOrderControl1),
}
