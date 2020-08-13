pub mod automata;
pub mod gliders;

use automata::Tape;

fn main() {
    let mut tape = Tape::new(gliders::wiggler());
    for i in 0..100 {
        println!("{:?}", tape);
        tape.step();
    }
}
