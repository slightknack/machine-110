use crate::automata::Tape;

// type A4 glider
pub fn lazer(phase: usize) -> Vec<bool> {
    match phase {
        _ => Tape::cells("0001110111")
    }
}

// type E glider
pub fn wiggler(phase: usize) -> Vec<bool> {
    match phase {
        _ => Tape::cells("1001111")
    }
}

// type C glider
pub fn stacker(phase: usize) -> Vec<bool> {
    match phase {
        _ => Tape::cells("111")
    }
}

// not a glider - just the background pattern
pub fn filler(phase: usize) -> Vec<bool> {
    Tape::cells("00010011011111")
}

pub fn half_filler(phase: usize) -> Vec<bool> {
    Tape::cells("0001001")
}

// spacers

// pub fn over(n: usize) -> Vec<bool> {
//     let cells =
// }
//
// pub fn up(n: usize) -> Vec<bool> {
//     todo!()
// }
