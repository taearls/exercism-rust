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
        let mut result = String::with_capacity(text.len());

        let mut vec: Vec<String> = (0..self.rails)
            .map(|_| String::with_capacity(text.len()))
            .collect();

        // build up rail cypher
        for pos in 0..text.len() {
            let row_with_char = pos % self.rails;
            for (row, item) in vec.iter_mut().enumerate().take(self.rails) {
                if row == row_with_char {
                    item.push(text.chars().nth(pos).unwrap());
                } else {
                    item.push('.');
                }
            }
        }

        // get encoded str in rail cypher
        for row in vec.iter().take(self.rails) {
            for c in row.chars() {
                if c != '.' {
                    result.push(c);
                }
            }
        }

        result
    }

    pub fn decode(&self, cipher: &str) -> String {
        unimplemented!("Decode this ciphertext: {}", cipher)
    }
}
