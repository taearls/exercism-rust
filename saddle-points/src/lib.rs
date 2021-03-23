use std::collections::HashMap;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let row_count = input.len();
    // map from col index to col
    let mut cols: HashMap<usize, Vec<u64>> = HashMap::new();

    if row_count > 0 {
        for row in 0..row_count {
            let current_row = input.get(row).unwrap();
            let max_row_val = match current_row.iter().max() {
                Some(val) => val,
                // returns None if current row is empty
                None => continue,
            };

            // iterate through items in row
            for (current_col_index, current_row_value) in current_row.iter().enumerate() {
                if current_row_value == max_row_val {
                    // only check cols of row element that matches max value
                    // calculate once using hashmap entry api
                    let cols: &mut Vec<u64> = cols.entry(current_col_index).or_insert_with(|| {
                        let mut col: Vec<u64> = Vec::new();
                        for current_row_val in 0..row_count {
                            let new_col_val: &u64 = input
                                .get(current_row_val)
                                .unwrap()
                                .get(current_col_index)
                                .unwrap();
                            col.push(*new_col_val);
                        }
                        col
                    });

                    let min_col_val = cols.iter().min().unwrap();
                    if current_row_value == min_col_val {
                        result.push((row, current_col_index));
                    }
                }
            }
        }
    }
    result.dedup();
    result
}
