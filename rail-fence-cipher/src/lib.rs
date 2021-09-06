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

        vec
            .iter()
            .map(|row| {
                row
                    .chars()
                    .filter(|c| *c != '.')
                    .collect::<String>()
            })
            .collect::<String>()
    }

    pub fn decode(&self, cipher: &str) -> String {
        unimplemented!("Decode this ciphertext: {}", cipher)
    }
}
