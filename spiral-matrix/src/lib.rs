pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let square_width = size as usize;
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; square_width]; square_width];

    let mut num: u32 = 1;
    let mut current_position: (usize, usize) = (0, 0);
    let mut direction: Direction = Direction::Right;
    let mut empty_elements = square_width * square_width;

    loop {
        if empty_elements == 0 {
            break;
        }
        let (row, col) = current_position;

        matrix[row][col] = num;

        num += 1;
        empty_elements -= 1;
        if row == square_width - 1 || col == square_width - 1 {
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
        }
        current_position = match direction {
            Direction::Up => (row - 1, col),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
        };
    }
    matrix
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction = match *self {
            Direction::Right => "Right",
            Direction::Left => "Left",
            Direction::Up => "Up",
            Direction::Down => "Down",
        };
        write!(f, "{}", direction)
    }
}
