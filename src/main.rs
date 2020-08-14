use std::{thread, time};

pub mod automata;
pub mod gliders;
pub mod cyclic;

use automata::Tape;
use gliders::*;
use cyclic::*;

const ITER: usize = 200;

fn main() {
    // build up a tape
    let mut starting = stacker();
    starting.append(&mut filler());
    starting.append(&mut stacker());
    starting.append(&mut filler());
    starting.append(&mut wiggler());

    let mut tape = Tape::new(starting);

    // run it
    for i in 0..ITER {
        tape.step();
        // print!("{}", "       ".repeat((ITER/tape.repeat) - i / tape.repeat));
        println!("{:?}", tape);
        thread::sleep(time::Duration::from_millis(16));
    }
}

// // what I'm aiming for:
// fn main() {
//     let initial = 7;
//
//     // build a cyclic tag system
//     let collatz_cyclic = Cyclic::new(
//         Tape::cells(&"100".repeat(initial)),
//         vec![
//             Tape::cells("010001"),
//             Tape::cells("100"),
//             Tape::cells("100100100"),
//             Tape::cells(""),
//             Tape::cells(""),
//             Tape::cells(""),
//         ]
//     );
//
//     // compile it into a rule 110 automata
//     let mut collatz_tape = Tape::compile(collatz_cyclic);
//
//     // run it!
//     for i in 0..ITER {
//         collatz_tape.step();
//         print!("{}", "       ".repeat((ITER/collatz_tape.repeat) - i / collatz_tape.repeat));
//         println!("{:?}", collatz_tape);
//         thread::sleep(time::Duration::from_millis(16));
//     }
// }
