// parser/scanner.rs
use super::state::ParserState;
use crate::memory::{MemoryDigit, ValidMemoryValue};

// Parser for scanning corrupted memory
pub struct MemoryParser {
    checksum: u32,      // Running total of valid multiplication results
    state: ParserState, // Current parser state for memory validation
    mul_enabled: bool,  // Tracks if multiplication is enabled by do()/don't()
}

impl Default for MemoryParser {
    fn default() -> Self {
        MemoryParser {
            checksum: 0,
            state: ParserState::Initial,
            mul_enabled: true, // Multiplications start enabled per specs
        }
    }
}

impl MemoryParser {
    pub fn checksum(&self) -> &u32 {
        &self.checksum
    }
    pub fn process_char(&mut self, c: char) {
        use ParserState::*;
        match &mut self.state {
            Initial => {
                if c == 'm' {
                    self.state = M;
                } else if c == 'd' {
                    self.state = D;
                }
            }
            M => {
                if c == 'u' {
                    self.state = MU;
                } else {
                    self.state = Initial;
                }
            }
            MU => {
                if c == 'l' {
                    self.state = Mul;
                } else {
                    self.state = Initial;
                }
            }
            Mul => {
                if c == '(' {
                    self.state = MulLParen;
                } else {
                    self.state = Initial;
                }
            }
            MulLParen => {
                if let Ok(digit) = MemoryDigit::try_from(c) {
                    self.state = ParsingX(digit.into());
                } else {
                    self.state = Initial;
                }
            }
            ParsingX(number) => {
                if let Ok(digit) = MemoryDigit::try_from(c) {
                    *number += digit;

                    // Validate memory value constraints
                    if ValidMemoryValue::try_from(*number).is_err() {
                        self.state = Initial;
                    }
                } else if c == ',' {
                    self.state = AfterComma(*number);
                } else {
                    self.state = Initial;
                }
            }
            AfterComma(x) => {
                if let Ok(digit) = MemoryDigit::try_from(c) {
                    self.state = ParsingY(*x, digit.into());
                } else {
                    self.state = Initial;
                }
            }
            ParsingY(x, y) => {
                if let Ok(digit) = MemoryDigit::try_from(c) {
                    *y += digit;

                    // Validate memory value constraints
                    if ValidMemoryValue::try_from(*y).is_err() {
                        self.state = Initial;
                    }
                } else if c == ')' {
                    if self.mul_enabled {
                        let result = x * y;
                        self.checksum += result;
                    }
                    self.state = Initial;
                } else {
                    self.state = Initial;
                }
            }
            D => {
                if c == 'o' {
                    self.state = DO;
                } else {
                    self.state = Initial;
                }
            }
            DO => {
                if c == '(' {
                    self.state = DoLparen;
                } else if c == 'n' {
                    self.state = Don;
                } else {
                    self.state = Initial;
                }
            }
            DoLparen => {
                if c == ')' {
                    self.mul_enabled = true; // Enable multiplication operations
                    self.state = Initial;
                } else {
                    self.state = Initial;
                }
            }
            Don => {
                if c == '\'' {
                    self.state = DontQuote;
                } else {
                    self.state = Initial;
                }
            }
            DontQuote => {
                if c == 't' {
                    self.state = Dont;
                } else {
                    self.state = Initial;
                }
            }
            Dont => {
                if c == '(' {
                    self.state = DontLparen;
                } else {
                    self.state = Initial;
                }
            }
            DontLparen => {
                if c == ')' {
                    self.mul_enabled = false; // Disable multiplication operations
                    self.state = Initial;
                } else {
                    self.state = Initial;
                }
            }
        }
    }
}
