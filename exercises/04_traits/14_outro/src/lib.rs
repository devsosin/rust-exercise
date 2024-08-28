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

use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16,
}

impl SaturatingU16 {
    fn new(value: u16) -> Self {
        Self { value }
    }
}

// 특정 자료형으로 바꿀 경우 From을 정의
impl Into<SaturatingU16> for &u8 {
    fn into(self) -> SaturatingU16 {
        SaturatingU16::new(*self as u16)
    }
}
impl Into<SaturatingU16> for u8 {
    fn into(self) -> SaturatingU16 {
        SaturatingU16::new(self as u16)
    }
}

impl Into<SaturatingU16> for &u16 {
    fn into(self) -> SaturatingU16 {
        SaturatingU16::new(*self)
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

// 어떤 값과 더해지냐에 따라 상대적으로 먼저 더해짐
impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.value // u16 더하는 impl
                         // Self::new(self.value.saturating_add(rhs.value))
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        self + *rhs // Self 더하는 impl
                    // Self::new(self.value.saturating_add(rhs.value))
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        Self {
            value: self.value.saturating_add(rhs),
        }
        // Self::new(self.value.saturating_add(rhs))
    }
}

// Add<&u16> ?
// self + *rhs

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
