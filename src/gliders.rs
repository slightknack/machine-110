use crate::automata::Tape;

// type A4 glider
pub fn lazer() -> Vec<bool> {
    Tape::cells("0001110111")
}

// type E glider
pub fn wiggler() -> Vec<bool> {
    Tape::cells("1001111")
}

// type C glider
pub fn stacker() -> Vec<bool> {
    Tape::cells("111")
}

// not a glider - just the background pattern
pub fn filler() -> Vec<bool> {
    Tape::cells("00010011011111")
}

// spacers

// pub fn over(n: usize) -> Vec<bool> {
//     let cells =
// }
//
// pub fn up(n: usize) -> Vec<bool> {
//     todo!()
// }
