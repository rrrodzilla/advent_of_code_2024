// memory/number.rs
use super::digit::MemoryDigit;
use std::cmp::Ordering;
use std::ops::{AddAssign, Mul};

// Represents a number found in corrupted memory
#[derive(Copy, Clone)]
pub struct MemoryNumber(u16);

// Implement multiplication for corrupted memory values
impl Mul<&mut MemoryNumber> for &mut MemoryNumber {
    type Output = u32;

    fn mul(self, rhs: &mut MemoryNumber) -> u32 {
        // Convert to u32 to prevent memory overflow during multiplication
        self.0 as u32 * rhs.0 as u32
    }
}

// Convert single digits to memory numbers
impl From<MemoryDigit> for MemoryNumber {
    fn from(digit: MemoryDigit) -> MemoryNumber {
        MemoryNumber(digit.into())
    }
}

// Memory number comparison operations
impl PartialEq<u16> for MemoryNumber {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u16> for MemoryNumber {
    fn partial_cmp(&self, other: &u16) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

// Memory digit accumulation operation
impl AddAssign<MemoryDigit> for MemoryNumber {
    fn add_assign(&mut self, digit: MemoryDigit) {
        // Append digit to current memory value
        let new_value = self.0 * 10 + u16::from(digit);
        self.0 = new_value;
    }
}

// Memory checksum accumulation
impl AddAssign<MemoryNumber> for u32 {
    fn add_assign(&mut self, rhs: MemoryNumber) {
        *self += rhs.0 as u32;
    }
}
