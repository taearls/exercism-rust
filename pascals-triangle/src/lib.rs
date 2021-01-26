pub struct PascalsTriangle {
    row_count: u32,
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            row_count: row_count,
            rows: PascalsTriangle::set_rows(row_count)
        }
    }

    pub fn rows(self) -> Vec<Vec<u32>> {
        self.rows
    }

    fn set_rows(row_count: u32) -> Vec<Vec<u32>> {
        match row_count {
            0 => vec![],
            _ => {
                let mut vec: Vec<Vec<u32>> = Vec::new();
                for row in 1..=row_count {
                    let new_row = PascalsTriangle::calculate_row(row);
                    vec.push(new_row);
                }
                vec
            },
        }
    }
    fn calculate_row(row_number: u32) -> Vec<u32> {
        let mut vec: Vec<u32> = vec![1];
        match row_number {
            1 => (),
            2 => vec.push(1),
            _ => {
                // https://en.wikipedia.org/wiki/Pascal%27s_triangle#Calculating_a_row_or_diagonal_by_itself
                for i in 2..=row_number {
                    let temp_num: f32 = match i {
                        1 => 1.0,
                        _ => (row_number + 1 - i) as f32 / (i - 1) as f32,
                    };
                    let num: u32 = match i {
                        1 => 1,
                        _ => ((vec[(i - 2) as usize]) as f32 * temp_num) as u32,
                    };
                    vec.push(num as u32);
                }
            }
        }
        vec
    }
}
