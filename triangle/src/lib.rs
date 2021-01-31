pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides[0] <= 0 || sides[1] <= 0 || sides[2] <= 0 {
            return None;
        } else if sides[1] + sides[2] < sides[0] {
            return None;
        } else if sides[0] + sides[2] < sides[1] {
            return None;
        } else if sides[0] + sides[1] < sides[2] {
            return None;
        }
        Some(Triangle {sides: sides})
    }

    pub fn is_equilateral(&self) -> bool {
        self.equal_side_count() == 3
    }

    pub fn is_scalene(&self) -> bool {
        self.equal_side_count() == 0
    }

    pub fn is_isosceles(&self) -> bool {
        self.equal_side_count() == 2
    }

    fn equal_side_count(&self) -> usize {
        let mut sides: usize = 0;
        if self.sides[0] == self.sides[1] && self.sides[0] == self.sides[2] {
            sides = 3;
        } else if self.sides[0] == self.sides[1] { 
            sides = 2;
        } else if self.sides[0] == self.sides[2] { 
            sides = 2;
        } else if self.sides[1] == self.sides[2] {
            sides = 2;
        }
        sides
    }
}
