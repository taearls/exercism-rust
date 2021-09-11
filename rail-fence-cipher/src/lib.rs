use unicode_segmentation::UnicodeSegmentation;

pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let graphemes: Vec<&str> = text.graphemes(true).collect();

        let mut vec: Vec<String> = (0..self.rails)
            .map(|_| String::with_capacity(graphemes.len()))
            .collect();

        // build up rail cypher
        for pos in 0..graphemes.len() {
            let row_with_char = get_row(pos, self.rails);
            for (row, item) in vec.iter_mut().enumerate().take(self.rails) {
                if row == row_with_char {
                    item.push_str(graphemes.get(pos).unwrap());
                } else {
                    item.push('.');
                }
            }
        }

        vec.iter()
            .map(|row| row.chars().filter(|c| *c != '.').collect::<String>())
            .collect::<String>()
    }

    // T . . . E . . . I . . . T . . . E . . . L .
    // . H . D . V . L . S . N . H . D . T . I . S
    // . . E . . . I . . . I . . . E . . . A . . .

    pub fn decode(&self, cipher: &str) -> String {
        // let graphemes: Vec<&str> = cipher.graphemes(true).collect();
        // let vec: Vec<String> = (0..self.rails)
        //     .map(|rail| build_row(self.rails, rail, cipher))
        //     .collect();
        // println!("vec: {:?}", vec);

        // cipher
        //     .chars()
        //     .map(|c| {

        //     });

        // vec.iter()
        //     .map(|row| row.chars().filter(|c| *c != '.').collect::<String>())
        //     .collect::<String>()
        String::from(cipher)
    }
}

// WEAREDISCOVEREDFLEEATONCE
// W . . . E
// . E . R .
// . . A . .
// 0 1 2 1

// 4 rails
// 0 1 2 3 2 1
fn get_row(pos: usize, rail_count: usize) -> usize {
    let mut vec: Vec<usize> = (0..rail_count).collect();

    (0..*vec.last().unwrap()).rev().for_each(|count| {
        vec.push(count);
    });

    let idx = pos % (rail_count + rail_count - 2);
    vec[idx]
}

// fn build_row(total_rows: usize, current_row: usize, cipher: &str) -> String {
//     let period_offset = current_row % total_rows;
//     let period_filler_count = if current_row == 0 || current_row == total_rows - 1 {
//         total_rows
//     } else {
//         total_rows - current_row
//     };
//     format!(
//         "{}{}{}",
//         ".".repeat(period_offset),
//         cipher.to_string().drain(0..1).collect::<String>(),
//         ".".repeat(period_filler_count)
//     )
// }
