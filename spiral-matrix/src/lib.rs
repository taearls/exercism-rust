pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let square_width = size as usize;
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; square_width]; square_width];
    if size == 0 {
        return matrix;
    }
    let mut current_position: (usize, usize) = (0, 0);
    let mut direction: Direction = Direction::Right;

    // top_left_bound needs to be initialized one row down so it doesn't match starting position
    let mut top_left_bound = (1, 0);
    let mut top_right_bound = (0, square_width - 1);
    let mut bottom_right_bound = (square_width - 1, square_width - 1);
    let mut bottom_left_bound = (square_width - 1, 0);

    for num in 1..=(size * size) {
        let (row, col) = current_position;
        matrix[row][col] = num;

        // check if we need to turn
        if current_position == top_left_bound
            || current_position == top_right_bound
            || current_position == bottom_right_bound
            || current_position == bottom_left_bound
        {
            // handle turn, update appropriate boundary
            direction = match direction {
                Direction::Up => {
                    top_left_bound = (top_left_bound.0 + 1, top_left_bound.1 + 1);
                    Direction::Right
                }
                Direction::Right => {
                    if top_right_bound.1 > 1 {
                        top_right_bound = (top_right_bound.0 + 1, top_right_bound.1 - 1);
                    }
                    Direction::Down
                }
                Direction::Down => {
                    if bottom_right_bound.0 > 1 && bottom_right_bound.1 > 1 {
                        bottom_right_bound = (bottom_right_bound.0 - 1, bottom_right_bound.1 - 1);
                    }
                    Direction::Left
                }
                Direction::Left => {
                    if bottom_left_bound.0 > 1 {
                        bottom_left_bound = (bottom_left_bound.0 - 1, bottom_left_bound.1 + 1);
                    }
                    Direction::Up
                }
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
