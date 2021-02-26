pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let square_width = size as usize;
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; square_width]; square_width];


    let mut num: u32 = 1;
    let mut current_position: (usize, usize) = (0, 0);
    let mut direction: Direction = Direction::Right;
    let mut empty_elements = square_width * square_width;
    let mut traverse_amount_before_turn = square_width;

    loop {
        if empty_elements == 0 {
            break;
        }
        let (row, col) = current_position;

        matrix[row][col] = num;

        num += 1;
        traverse_amount_before_turn -= 1;
        empty_elements -= 1;
        if traverse_amount_before_turn == 0 {
            traverse_amount_before_turn = 1;
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
        }
        current_position = match direction {
            Direction::Up => {
                (row - 1, col)
            },
            Direction::Right => {
                (row, col + 1)
            },
            Direction::Down => {
                (row + 1, col)
            },
            Direction::Left => {
                (row, col - 1)
            },
        };
    }

    // for row in 0..square_width {
    //     for col in 0..square_width {
    //         fill_matrix(&mut matrix, row, col, &direction, num);
    //         num += 1;
    //     }
    // }
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

// fn fill_matrix(matrix: &mut Vec<Vec<u32>>, row: usize, col: usize, direction: &Direction, num: u32) {
//     matrix[row][col] = num;
// }

// 1 2
// 4 3

// 1 2 3
// 8 9 4
// 7 6 5

// 1  2  3  4
// 12 13 14 5
// 11 16 15 6
// 10 9  8  7

// #[test]
// #[ignore]
// fn size_one_spiral() {
//     let mut expected: Vec<Vec<u32>> = Vec::new();
//     expected.push(vec![1]);
//     assert_eq!(spiral_matrix(1), expected);
// }