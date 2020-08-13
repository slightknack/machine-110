use std::fmt::Debug;

pub struct Cyclic {
    state: Vec<bool>,
    rules: Vec<Vec<bool>>,
}

impl Cyclic {
    pub fn new(state: Vec<bool>, rules: Vec<Vec<bool>>) -> Cyclic {
        Cyclic { state, rules }
    }

    pub fn step(&mut self) {
        let rule = self.rules.remove(0);
        if !self.state.is_empty() && self.state.remove(0) {
            self.state.append(&mut rule.clone())
        }
        self.rules.push(rule);
    }
}

impl Debug for Cyclic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rule in self.rules.iter() {
            write!(f, "|")?;
            for bit in rule {
                match bit {
                    false => write!(f, " ")?,
                    true  => write!(f, "█")?,
                }
            }
        }
        write!(f, "| -> |")?;
        for bit in self.state.iter() {
            match bit {
                false => write!(f, " ")?,
                true  => write!(f, "█")?,
            }
        }
        write!(f, "|")?;
        Ok(())
    }
}

pub trait Compile {
    fn compile(i: Cyclic) -> Self;
}
