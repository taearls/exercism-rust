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
        let mut vec: Vec<String> = (0..self.rails)
            .map(|_| String::with_capacity(text.len()))
            .collect();

        // build up rail cypher
        let graphemes: Vec<&str> = text.graphemes(true).collect();
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

    pub fn decode(&self, cipher: &str) -> String {
        unimplemented!("Decode this ciphertext: {}", cipher)
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
