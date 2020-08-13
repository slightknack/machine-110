pub mod automata;

fn main() {
    let mut tape = automata::Tape::new(vec![true]);
    for i in 0..20 {
        tape.pad();
        println!("{}{:?}", " ".repeat(40-i), tape);
        tape.step();
    }
}
