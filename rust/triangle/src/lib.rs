pub struct Triangle {
    x: u64,
    y: u64,
    z: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        match (sides[0], sides[1], sides[2]) {
            (x, y, z) if x == 0 || y == 0 || z == 0 => None,
            (x, y, z) if x + y <= z || y + z <= x || z + x <= y => None,
            (x, y, z) => Some(Triangle { x: x, y: y, z: z }),
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.x == self.y &&
        self.y == self.z &&
        self.z == self.x
    }

    pub fn is_scalene(&self) -> bool {
        !(self.is_equilateral() || self.is_isosceles())
    }

    pub fn is_isosceles(&self) -> bool {
        self.x == self.y ||
        self.y == self.z ||
        self.z == self.x
    }
}
