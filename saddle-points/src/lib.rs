use std::collections::HashMap;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let row_count = input.len();
    println!("input: {:?}", input);
    println!("row_count: {}", row_count);
    let mut cols: HashMap<usize, Vec<u64>> = HashMap::new();
    if row_count > 0 {
        for row in 0..row_count {
            let current_row = input.get(row).unwrap().to_vec();
            let max_row_val = match current_row.iter().max() {
                Some(val) => val,
                None => continue,
            };
            println!("max_row_val: {}", max_row_val);

            // iterate through items in row
            for current_row_value in &current_row {
                if current_row_value == max_row_val {
                    // only check cols of row element that matches max value
                    // calculate once using hashmap entry api
                    let current_col_index = || {
                        current_row
                            .iter()
                            .position(|x| x == current_row_value)
                            .unwrap()
                    };
                    let cols: &mut Vec<u64> = cols.entry(row).or_insert_with(|| {
                        let mut col: Vec<u64> = Vec::new();
                        for current_row_val in 0..row_count {
                            let new_col_val: &u64 = input
                                .get(current_row_val)
                                .unwrap()
                                .get(current_col_index())
                                .unwrap();
                            col.push(*new_col_val);
                        }
                        col
                    });

                    //
                    let min_col_val = cols.iter().min().unwrap();
                    if current_row_value == min_col_val {
                        result.push((row, current_col_index()));
                    }
                }
            }
        }
    }
    result
}
