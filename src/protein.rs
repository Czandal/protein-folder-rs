#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Protein {
    // position on 2d plane
    pub x: i32,
    pub y: i32,
    // is hydrophobic
    pub hydrophobic: bool,
}

impl Protein {
    pub fn new_on_left(protein: &Protein, hydrophobic: bool) -> Protein {
        Protein {
            x: protein.x - 1,
            y: protein.y,
            hydrophobic,
        }
    }

    pub fn new_on_right(protein: &Protein, hydrophobic: bool) -> Protein {
        Protein {
            x: protein.x + 1,
            y: protein.y,
            hydrophobic,
        }
    }

    pub fn new_on_up(protein: &Protein, hydrophobic: bool) -> Protein {
        Protein {
            x: protein.x,
            y: protein.y + 1,
            hydrophobic,
        }
    }

    pub fn new_on_down(protein: &Protein, hydrophobic: bool) -> Protein {
        Protein {
            x: protein.x,
            y: protein.y - 1,
            hydrophobic,
        }
    }

    pub fn neighbours(&self, other_protein: &Protein) -> bool {
        if (self.x + 1 == other_protein.x || self.x - 1 == other_protein.x)
            && self.y == other_protein.y
        {
            return true;
        }
        if (self.y + 1 == other_protein.y || self.y - 1 == other_protein.y)
            && self.x == other_protein.x
        {
            return true;
        }
        return false;
    }
}
