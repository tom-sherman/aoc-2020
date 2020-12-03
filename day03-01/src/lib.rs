#[derive(Copy, Clone, Debug)]
pub enum Square {
    Empty,
    Tree,
}

impl Square {
    pub fn is_tree(&self) -> bool {
        match &self {
            Square::Tree => true,
            Square::Empty => false,
        }
    }
}

pub struct Topology<'t> {
    pub squares: &'t Vec<Vec<Square>>,
    pub height: usize,
    pub width: usize,
}

pub struct Slope {
    pub vertical: usize,
    pub horizontal: usize,
}

impl<'t> Topology<'t> {
    /// Get the squares on the path of some slope
    pub fn iter(&'t self, slope: &'t Slope) -> TopologyIterator<'t> {
        TopologyIterator {
            topology: &self,
            slope: &slope,
            curr_vertical: 0,
            curr_horizontal: 0,
        }
    }
}

pub struct TopologyIterator<'t> {
    topology: &'t Topology<'t>,
    slope: &'t Slope,
    curr_vertical: usize,
    curr_horizontal: usize,
}

impl<'t> Iterator for TopologyIterator<'t> {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        if self.curr_vertical >= self.topology.height {
            return None;
        }

        let horizontal = self.curr_horizontal;
        let vertical = self.curr_vertical;

        self.curr_horizontal = self.curr_horizontal + self.slope.horizontal;
        self.curr_vertical = self.curr_vertical + self.slope.vertical;

        Some(self.topology.squares[vertical][horizontal % self.topology.width])
    }
}
