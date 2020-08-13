use std::fmt::Debug;
use crate::cyclic;

const TRIM: bool = true;

pub struct Tape {
    pub iterations: usize,
    pub left: isize,
    pub right: isize,
    pub tiling: Vec<bool>,
    pub repeat: usize,
    pub rule: u8,
    pub cells: Vec<bool>,
}

impl Tape {
    pub fn cells(string: &str) -> Vec<bool> {
        let mut cells = vec![];

        for char in string.chars() {
            let cell = match char {
                '0' => false,
                '1' => true,
                _   => panic!("Expected string of only ones ('1') and zeroes ('0')"),
            };
            cells.push(cell);
        }

        return cells;
    }

    pub fn new(cells: Vec<bool>) -> Tape {
        Tape {
            iterations: 0,
            left: -1,
            right: 0,
            tiling: Tape::cells("00010011011111"),
            repeat: 7,
            rule: 110,
            cells
        }
    }

    pub fn tile_lookup(&self, index: isize) -> bool {
        assert_eq!(self.iterations % 7, 0);
        let modulated = index.rem_euclid(self.tiling.len() as isize);
        return self.tiling[modulated as usize];
    }

    pub fn tile_left(&mut self) {
        let cell = self.tile_lookup(self.left as isize);
        self.cells.insert(0, cell);
        self.left -= 1;
    }

    pub fn tile_right(&mut self) {
        let cell = self.tile_lookup(self.right as isize);
        self.cells.push(cell);
        self.right += 1;
    }

    // not perfect, just prevents some unnecessary growth
    pub fn needs_tiling(&mut self) -> (bool, bool) {
        let size = self.tiling.len();

        if self.cells.len() <= size || !TRIM { return (true, true) }

        let left_slice  = &self.cells[0..size];
        let right_slice = &self.cells[self.cells.len()-size..self.cells.len()];

        return (
            left_slice  != self.tiling.as_slice(),
            right_slice != self.tiling.as_slice(),
        );
    }

    pub fn tile(&mut self) {
        // only insert background tiling when applicable
        if self.iterations % self.repeat != 0 {
            return;
        }

        let (left, right) = self.needs_tiling();
        if TRIM { println!("tiling | left: {} right: {}", left, right); }

        // a triangle of height n will have a width of 2n, hence:
        for _ in 0..(2 * self.repeat) {
            if left  { self.tile_left();  }
            if right { self.tile_right(); }
        }
    }

    pub fn apply_rule(&self, step: &[bool]) -> bool {
        let index: u8 = if step[0] { 4 } else { 0 }
                      + if step[1] { 2 } else { 0 }
                      + if step[2] { 1 } else { 0 };
        return (self.rule & (1 << index)) != 0;
    }

    pub fn step(&mut self) {
        self.tile();
        let mut new = vec![];

        for index in 0..(self.cells.len()-2) {
            new.push(self.apply_rule(&self.cells[index..index+3]));
        }

        self.left += 1;
        self.right -= 1;
        self.iterations += 1;
        self.cells = new;
    }
}

impl Debug for Tape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#4} {:#4} {}\\",
            self.left,
            self.right,
            " ".repeat((self.iterations - 1) % self.repeat)
        )?;

        for cell in self.cells.iter() {
            match cell {
                false => write!(f, " ")?,
                true  => write!(f, "█")?,
            }
        }
        write!(f, "/")?;
        Ok(())
    }
}

impl cyclic::Compile for Tape {
    /// converts a cyclic tag system into a rule 110 celluar automata
    fn compile(i: cyclic::Cyclic) -> Tape {
        todo!()
    }
}
