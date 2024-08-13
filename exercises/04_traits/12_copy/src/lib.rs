// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

use std::fmt::Debug;

pub struct WrappingU32 {
    value: u32,
}

impl std::ops::Add for WrappingU32 {
    type Output = WrappingU32;

    fn add(self, rhs: Self) -> Self::Output {
        WrappingU32::new(self.value.wrapping_add(rhs.value))
    }
}

impl PartialEq for WrappingU32 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Copy for WrappingU32 {}

impl Clone for WrappingU32 {
    fn clone(&self) -> Self {
        Self { value: self.value }
    }
}

impl Debug for WrappingU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WrappingU32")
            .field("value", &self.value)
            .finish()
    }
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
