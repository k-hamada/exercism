#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: isize,
    y: isize,
    d: Direction,
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        let d = match self.direction() {
            &Direction::North => Direction::East,
            &Direction::East  => Direction::South,
            &Direction::South => Direction::West,
            &Direction::West  => Direction::North,
        };

        Robot { d, ..self }
    }

    pub fn turn_left(self) -> Self {
        let d = match self.direction() {
            &Direction::North => Direction::West,
            &Direction::East  => Direction::North,
            &Direction::South => Direction::East,
            &Direction::West  => Direction::South,
        };

        Robot { d, ..self }
    }

    pub fn advance(self) -> Self {
        let (move_x, move_y) = match self.direction() {
            &Direction::North => ( 0,  1),
            &Direction::East  => ( 1,  0),
            &Direction::South => ( 0, -1),
            &Direction::West  => (-1,  0),
        };

        Robot {
            x: self.x + move_x,
            y: self.y + move_y,
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for instruction in instructions.chars() {
            match instruction {
                'R' => robot = robot.turn_right(),
                'L' => robot = robot.turn_left(),
                'A' => robot = robot.advance(),
                _ => panic!("crash"),
            }
        }
        robot
    }

    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
