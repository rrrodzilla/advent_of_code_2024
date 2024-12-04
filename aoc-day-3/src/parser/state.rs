// parser/state.rs
use crate::memory::MemoryNumber;

// Memory corruption parser states
pub enum ParserState {
    Initial,
    M,
    MU,
    Mul,
    MulLParen,
    ParsingX(MemoryNumber),
    AfterComma(MemoryNumber),
    ParsingY(MemoryNumber, MemoryNumber),
    D,
    DO,
    Don,
    DontQuote,
    Dont,
    DontLparen,
    DoLparen,
}
