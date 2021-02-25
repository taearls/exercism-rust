pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let square_width = size as usize;
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; square_width]; square_width];
    let mut num = 1;
    for row in 0..square_width {
        for col in 0..square_width {
            matrix[row][col] = num;
            num += 1;
        }
    }
    matrix
}

// 1 2 3
// 8 9 4
// 7 6 5

// #[test]
// #[ignore]
// fn size_one_spiral() {
//     let mut expected: Vec<Vec<u32>> = Vec::new();
//     expected.push(vec![1]);
//     assert_eq!(spiral_matrix(1), expected);
// }