pub struct Triangle {
     sides: [u64; 3],
}

impl Triangle {
     pub fn build(sides: [u64; 3]) -> Option<Triangle> {
         if sides[0] + sides[1] <= sides[2] || sides[0] + sides[2] <= sides[1] || sides[1] + sides[2] <= sides[0] {
             None
         } else {
             Some(Triangle { sides })
         }
    }

     pub fn is_equilateral(&self) -> bool {
         self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

     pub fn is_scalene(&self) -> bool {
         self.sides[0] != self.sides[1] && self.sides[1] != self.sides[2] && self.sides[0] != self.sides[2]
    }

     pub fn is_isosceles(&self) -> bool {
         self.sides[0] == self.sides[1] || self.sides[1] == self.sides[2] || self.sides[0] == self.sides[2]
    }
}
