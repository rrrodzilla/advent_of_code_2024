// memory/mod.rs
mod digit;
mod number;
mod validation;

pub use digit::MemoryDigit;
pub use number::MemoryNumber;
pub use validation::ValidMemoryValue;

// Re-export the MAX_MEMORY_VALUE constant
pub const MAX_MEMORY_VALUE: u16 = 999;
