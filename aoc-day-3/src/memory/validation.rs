// memory/validation.rs
use super::{MemoryNumber, MAX_MEMORY_VALUE};

// Represents a validated 3-digit number from corrupted memory
#[derive(Copy, Clone, Debug)]
pub struct ValidMemoryValue;

// Memory validation implementation
impl TryFrom<MemoryNumber> for ValidMemoryValue {
    type Error = ();

    fn try_from(number: MemoryNumber) -> Result<Self, Self::Error> {
        if number <= MAX_MEMORY_VALUE {
            Ok(ValidMemoryValue)
        } else {
            Err(()) // Memory value exceeds valid range
        }
    }
}
