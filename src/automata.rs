use std::fmt::Debug;

pub struct Tape(Vec<bool>);

impl Tape {
    pub fn new(state: Vec<bool>) -> Tape {
        Tape(state)
    }

    pub fn rule(step: &[bool]) -> bool {
        let index: u8 = if step[0] { 4 } else { 0 }
                      + if step[1] { 2 } else { 0 }
                      + if step[2] { 1 } else { 0 };
        // print!("{:?} {:?}", step, index);
        let rule: u8 = 110; // binary: 0110_1110
        // for i in 0..8 {
        //     println!("{}: {:?}", i, (rule & (1 << 7 - i)) != 0);
        // }
        // panic!();
        let result = (rule & (1 << index)) != 0;
        // println!(" -> {:?}", result);
        return result;
    }

    pub fn pad(&mut self) {
        // inneficient, shifts everything
        while self.0.len() <= 3 || self.0[0..3].contains(&true) {
            self.0.insert(0, false);
        }
        while self.0[self.0.len()-3..self.0.len()].contains(&true) {
            self.0.push(false)
        }
    }

    pub fn step(&mut self) {
        self.pad();
        let mut new = vec![];

        for index in 0..(self.0.len()-2) {
            new.push(Tape::rule(&self.0[index..index+3]));
        }

        self.0 = new;
    }
}

impl Debug for Tape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cell in self.0.iter() {
            match cell {
                false => write!(f, " ")?,
                true  => write!(f, "█")?,
            }
        }
        Ok(())
    }
}
