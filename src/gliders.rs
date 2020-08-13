use crate::automata::Tape;

pub fn lazer() -> Vec<bool> {
    Tape::cells("0001110111")
}

pub fn wiggler() -> Vec<bool> {
    Tape::cells("1001111")
}

pub fn vertical() -> Vec<bool> {
    Tape::cells("111")
}

// not a glider - just the background pattern
pub fn filler() -> Vec<bool> {
    Tape::cells("00010011011111")
}
