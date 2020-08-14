#[derive(Clone, Copy)]
pub enum Glider {
    Lazer,
    Wiggler,
    Stacker,
}


pub struct Gliders {
    phase: usize,
    gliders:  Vec<Glider>,
    /// Distance in T3 triangles in the ether
    distances: Vec<usize>,
}

impl Gliders {
    pub fn repeat(
        phase: usize,
        glider: Glider,
        distances: Vec<usize>
    ) -> Gliders {
        Gliders {
            phase,
            gliders: vec![glider].repeat(distances.len()),
            distances,
        }
    }

    // 1Ele_C2
    pub fn true_stackers() -> Gliders {
        Gliders::repeat(0, Glider::Stacker, vec![9, 9, 7])
    }

    // 0Ele_C2
    pub fn false_stackers() -> Gliders {
        Gliders::repeat(0, Glider::Stacker, vec![9, 5, 7])
    }

    // OBlo_E
    pub fn zero_wiggler() -> Gliders {
        Gliders::repeat(0, Glider::Wiggler, vec![10, 1, 2, 8, 8, 8, 10, 1, 2, 8, 8])
    }

    // 1BloP_E
    pub fn primary_wiggler() -> Gliders {
        Gliders::repeat(0, Glider::Wiggler, vec![4,  6, 2, 8, 8, 2, 10, 1, 2, 8, 8])
    }

    // 1BloS_E
    pub fn standard_wiggler() -> Gliders {
        Gliders::repeat(0, Glider::Wiggler, vec![10, 1, 2, 8, 8, 2, 10, 1, 2, 8, 8])
    }

    // SepInit_EE
    pub fn leading_wiggler(big: bool) -> Gliders {
        todo!()
    }

    // 1Add_E
    pub fn one_add_wiggler() -> Gliders {
        todo!()
    }

    // 0Add_E
    pub fn zero_add_wiggler() -> Gliders {
        todo!()
    }
}
