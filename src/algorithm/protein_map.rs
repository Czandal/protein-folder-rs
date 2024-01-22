use std::cell::Cell;

use super::protein::Protein;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProteinMap {
    pub score: Cell<i32>,
    pub max_x: i32,
    pub min_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub points: Vec<Protein>,
}

impl ProteinMap {
    pub fn find(&self, x: i32, y: i32) -> Option<&Protein> {
        self.points.iter().find(|prot| x == prot.x && y == prot.y)
    }
    fn update_score(&self, new_tail: &Protein) {
        let old_tail = self.points.last();
        match old_tail {
            Some(old_tail) => {
                if !new_tail.hydrophobic {
                    return;
                }
                if old_tail.x != new_tail.x - 1 {
                    if let Some(left_prot) = self.find(new_tail.x - 1, new_tail.y) {
                        if left_prot.hydrophobic {
                            self.score.set(self.score.get() + 1);
                        }
                    }
                }
                if old_tail.x != new_tail.x + 1 {
                    if let Some(right_prot) = self.find(new_tail.x + 1, new_tail.y) {
                        if right_prot.hydrophobic {
                            self.score.set(self.score.get() + 1);
                        }
                    }
                }

                if old_tail.y != new_tail.y + 1 {
                    if let Some(up_prot) = self.find(new_tail.x, new_tail.y + 1) {
                        if up_prot.hydrophobic {
                            self.score.set(self.score.get() + 1);
                        }
                    }
                }
                if old_tail.y != new_tail.y - 1 {
                    if let Some(down_prot) = self.find(new_tail.x, new_tail.y - 1) {
                        if down_prot.hydrophobic {
                            self.score.set(self.score.get() + 1);
                        }
                    }
                }
            }
            None => {
                // do nothing since the should be set to 0
            }
        };
    }

    pub fn push(&mut self, protein: Protein) -> Option<()> {
        if self.points.is_empty() {
            self.max_x = protein.x;
            self.min_x = protein.x;
            self.max_y = protein.y;
            self.min_y = protein.y;
            self.points.push(protein);
            return Some(());
        }
        let tail = self.points[self.points.len() - 1];
        if !tail.neighbours(&protein) { panic!("Tried to push protein which does not neighbour tail") }
        let protein_at_appended_pos = self.find(protein.x, protein.y);
        match protein_at_appended_pos {
            Some(_) => None,
            None => {
                self.update_score(&protein);
                self.points.push(protein);
                self.max_x = self.max_x.max(protein.x);
                self.max_y = self.max_y.max(protein.y);
                self.min_y = self.min_y.min(protein.y);
                self.min_x = self.min_x.min(protein.x);
                Some(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_updates_score_and_coords() {
        let mut map = ProteinMap {
            score: Cell::from(0),
            max_x: 1,
            min_x: 0,
            min_y: 0,
            max_y: 1,
            points: vec![Protein{x:0,y:0, hydrophobic: true}, Protein{x:0,y:1, hydrophobic: true}, Protein{x:1,y:1, hydrophobic: true}],
        };

        map.push(Protein {x: 1, y:0, hydrophobic: true});

        assert_eq!(map.score.get(), 1);

        assert_eq!(map.min_y, 0);
        assert_eq!(map.max_y, 1);
        assert_eq!(map.min_x, 0);
        assert_eq!(map.max_x, 1);
        assert_eq!(map.push(Protein {x: 1, y:-1, hydrophobic: true}), Some(()));
        assert_eq!(map.max_y, 1);
        assert_eq!(map.min_x, 0);
        assert_eq!(map.max_x, 1);
        assert_eq!(map.min_y, -1);
        assert_eq!(map.score.get(), 1);
        assert_eq!(map.push(Protein {x: 0, y:-1, hydrophobic: true}), Some(()));
        assert_eq!(map.score.get(), 2);
        assert_eq!(map.max_y, 1);
        assert_eq!(map.min_x, 0);
        assert_eq!(map.max_x, 1);
        assert_eq!(map.min_y, -1);
        assert_eq!(map.push(Protein {x: -1, y:-1, hydrophobic: true}), Some(()));
        assert_eq!(map.score.get(), 2);
        assert_eq!(map.max_y, 1);
        assert_eq!(map.min_x, -1);
        assert_eq!(map.max_x, 1);
        assert_eq!(map.min_y, -1);
    }

    #[test]
    fn push_does_not_allow_push_on_existing_pos() {
        let mut map = ProteinMap {
            score: Cell::from(0),
            max_x: 1,
            min_x: 0,
            min_y: 0,
            max_y: 1,
            points: vec![Protein{x:0,y:0, hydrophobic: true}, Protein{x:0,y:1, hydrophobic: true}, Protein{x:1,y:1, hydrophobic: true}],
        };

        assert_eq!(map.push(Protein {x: 0, y:1, hydrophobic: true}), None);
    }


    #[test]
    #[should_panic]
    fn push_panics_on_pushing_position_which_is_too_far_from_tail() {
        let mut map = ProteinMap {
            score: Cell::from(0),
            max_x: 1,
            min_x: 0,
            min_y: 0,
            max_y: 1,
            points: vec![Protein{x:0,y:0, hydrophobic: true}, Protein{x:0,y:1, hydrophobic: true}, Protein{x:1,y:1, hydrophobic: true}],
        };
        map.push(Protein {x: 10, y:1, hydrophobic: true});
    }

    #[test]
    fn find_should_return_none_on_nonexistent_place() {
        let mut map = ProteinMap {
            score: Cell::from(0),
            max_x: 1,
            min_x: 0,
            min_y: 0,
            max_y: 1,
            points: vec![Protein{x:0,y:0, hydrophobic: true}, Protein{x:0,y:1, hydrophobic: true}, Protein{x:1,y:1, hydrophobic: true}],
        };

        let result = map.find(1, 0);
        assert_eq!(result, None);

        let existing_result = map.find(0,1);
        assert_eq!(existing_result, Some(&map.points[1]));
        map.push(Protein {x: 1, y:0, hydrophobic: true});
        let result_after_push = map.find(1, 0);
        assert_eq!(result_after_push, Some(&map.points[3]));
    }
    
    #[test]
    fn push_allows_to_build_valid_map() {
        let mut map = ProteinMap {
            score: Cell::from(0),
            max_x: 0,
            min_x: 0,
            min_y: 0,
            max_y: 0,
            points: vec![Protein{x:0,y:0, hydrophobic: true}],
        };
        assert_eq!(map.push(Protein {x: 0, y:1, hydrophobic: true}), Some(()));
        assert_eq!(map.push(Protein {x: 1, y:1, hydrophobic: true}), Some(()));
        assert_eq!(map.push(Protein {x: 1, y:0, hydrophobic: true}), Some(()));
        assert_eq!(map.score.get(), 1);
        assert_eq!(map.min_y, 0);
        assert_eq!(map.max_y, 1);
        assert_eq!(map.min_x, 0);
        assert_eq!(map.max_x, 1);
    }
}
