#[derive(Copy, Clone, Debug)]
enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}

pub fn spiral_matrix(size: usize) -> Vec<Vec<usize>> {
    if size == 0 {
        return vec![];
    };

    let mut matrix = vec![vec![0; size]; size];
    let mut x = 0;
    let mut y = 0;
    let mut direction = Direction::RIGHT;
    let w = size - 1;
    let h = size - 1;
    for i in 1..size * size + 1 {
        matrix[y][x] = i;

        direction = match direction {
            Direction::RIGHT if x >= w => Direction::DOWN,
            Direction::DOWN if y >= h => Direction::LEFT,
            Direction::LEFT if x == 0 => Direction::UP,
            Direction::UP if y == 0 => Direction::RIGHT,
            Direction::RIGHT if matrix[y][x + 1] != 0 => Direction::DOWN,
            Direction::DOWN if matrix[y + 1][x] != 0 => Direction::LEFT,
            Direction::LEFT if matrix[y][x - 1] != 0 => Direction::UP,
            Direction::UP if matrix[y - 1][x] != 0 => Direction::RIGHT,
            Direction::RIGHT => Direction::RIGHT,
            Direction::DOWN => Direction::DOWN,
            Direction::LEFT => Direction::LEFT,
            Direction::UP => Direction::UP,
        };
        match direction {
            Direction::RIGHT => x += 1,
            Direction::DOWN => y += 1,
            Direction::LEFT => x -= 1,
            Direction::UP => y -= 1,
        }
    }
    matrix
}
