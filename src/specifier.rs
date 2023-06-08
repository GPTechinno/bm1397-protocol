/// Clock Select.
///
/// This is used by [`ClockOrderControl0::clock_select`], [`ClockOrderControl0::set_clock_select`],
/// [`ClockOrderControl1::clock_select`] and [`ClockOrderControl1::set_clock_select`] method
///
/// [`ClockOrderControl0::clock_select`]: crate::register::ClockOrderControl0::clock_select
/// [`ClockOrderControl0::set_clock_select`]: crate::register::ClockOrderControl0::set_clock_select
/// [`ClockOrderControl1::clock_select`]: crate::register::ClockOrderControl1::clock_select
/// [`ClockOrderControl1::set_clock_select`]: crate::register::ClockOrderControl1::set_clock_select
#[derive(Copy, Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum ClockSelect {
    /// Default.
    Default = 0b0000,
}
impl ClockSelect {
    /// Convert a raw `u8` to an `ClockSelect`.
    ///
    /// Bit values that do not correspond to a ClockSelect will be returned in the
    /// `Err` variant of the result.
    ///
    /// # Example
    ///
    /// ```
    /// use bm1397_protocol::specifier::ClockSelect;
    ///
    /// assert_eq!(ClockSelect::from_raw(0b0000), Ok(ClockSelect::Default));
    /// assert_eq!(ClockSelect::from_raw(0b0101), Err(0b0101));
    /// ```
    pub const fn from_raw(val: u8) -> Result<Self, u8> {
        match val {
            x if x == ClockSelect::Default as u8 => Ok(ClockSelect::Default),
            _ => Err(val),
        }
    }
}
impl From<ClockSelect> for u8 {
    fn from(val: ClockSelect) -> u8 {
        val as u8
    }
}
impl Default for ClockSelect {
    fn default() -> Self {
        Self::Default
    }
}
impl TryFrom<u8> for ClockSelect {
    type Error = u8;
    fn try_from(val: u8) -> Result<Self, u8> {
        Self::from_raw(val)
    }
}
