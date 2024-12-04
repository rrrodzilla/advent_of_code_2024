// Memory digit found in corrupted data
#[derive(Copy, Clone)]
pub struct MemoryDigit(u8);

// Convert raw memory values to digits
impl TryFrom<char> for MemoryDigit {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        if c.is_ascii_digit() {
            Ok(MemoryDigit(c as u8 - b'0'))
        } else {
            Err(()) // Not a valid memory digit
        }
    }
}

impl From<MemoryDigit> for u16 {
    fn from(digit: MemoryDigit) -> u16 {
        digit.0 as u16
    }
}
