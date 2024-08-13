use std::{fmt::Debug, ops::Add};

// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
pub struct SaturatingU16 {
    value: u16,
}

impl Clone for SaturatingU16 {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for SaturatingU16 {}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        Self::new(value as u16)
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        Self::new(*value)
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        Self::new(*value as u16)
    }
}

impl Add for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value.saturating_add(rhs.value))
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output {
        Self::new(self.value.saturating_add(rhs))
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        Self::new(self.value.saturating_add(rhs.value))
    }
}

impl PartialEq for SaturatingU16 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

impl Debug for SaturatingU16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SaturatingU16")
            .field("value", &self.value)
            .finish()
    }
}

impl SaturatingU16 {
    fn new(value: u16) -> Self {
        Self { value }
    }
}
