use unicode_segmentation::UnicodeSegmentation;

pub struct RailFence {
    rails: usize,
    rail_len: usize
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
            rail_len: 0
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let graphemes: Vec<&str> = text.graphemes(true).collect();

        let mut vec: Vec<String> = (0..self.rails)
            .map(|_| String::with_capacity(graphemes.len()))
            .collect();

        // build up rail cypher
        graphemes.iter().enumerate().for_each(|(pos, grapheme)| {
            let row_with_char = get_row(pos, self.rails);
            for (row, item) in vec.iter_mut().enumerate().take(self.rails) {
                if row == row_with_char {
                    item.push_str(grapheme);
                }
            }
        });
        vec.iter().fold(String::new(), |acc, x| acc + x)
    }
    // "TEITELHDVLSNHDTISEIIEA" -> "THEDEVILISINTHEDETAILS"
    // T . . . E . . . I . . . T . . . E . . . L .
    // . H . D . V . L . S . N . H . D . T . I . S
    // . . E . . . I . . . I . . . E . . . A . . .

    pub fn decode(&self, cipher: &str) -> String {
        let graphemes: String = cipher.graphemes(true).map(|str| str.to_string()).collect();
        // get encoded rail lens
        let rail_lens: Vec<usize> = get_rail_lens(self.rails, &graphemes);

        let mut start = 0;

        let copy_str = graphemes.clone();
        let result = String::new();
        for (idx, grapheme) in cipher.graphemes(true).enumerate() {}

        // write str into vec
        // for len in rail_lens.iter() {
        //     let mut rail = String::with_capacity(*len);
        //     for i in 0..*len {
        //         rail.push_str(cipher.get(i..i + 1).unwrap());
        //     }
        //     rails.push(rail);
        // }

        // read vec in zig zag, fold into String

        println!("row_lens: {:?}", rail_lens);

        // println!("rails: {:?}", rails);

        String::new()
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

fn get_rail_lens(rail_count: usize, graphemes: &str) -> Vec<usize> {
    let mut is_increasing = true;

    let mut current_rail = 0;
    let mut rail_lens: Vec<usize> = (0..rail_count).map(|_| 0).collect();

    for _ in 0..graphemes.len() {
        rail_lens[current_rail] += 1;
        match current_rail {
            0 => {
                is_increasing = true;
                current_rail += 1;
            }
            _ if current_rail == rail_count - 1 => {
                is_increasing = false;
                current_rail -= 1;
            }
            _ => {
                if is_increasing {
                    current_rail += 1;
                } else {
                    current_rail -= 1;
                }
            }
        }
    }
    rail_lens
}

// fn build_row(total_rows: usize, current_row: usize, cipher: &str) -> String {
//     let period_start_offset = current_row % total_rows;
//     let period_filler_count = if current_row == 0 || current_row == total_rows - 1 {
//         total_rows
//     } else {
//         total_rows - current_row
//     };
//     format!(
//         "{}{}{}",
//         ".".repeat(period_start_offset),
//         cipher.to_string().drain(0..1).collect::<String>(),
//         ".".repeat(period_filler_count)
//     )
// }
