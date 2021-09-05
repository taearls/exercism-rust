pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence { rails: rails as usize }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut vec: Vec<String> = Vec::with_capacity(self.rails);

        // initialize vec with strings
        for _ in 0..self.rails {
            vec.push(String::with_capacity(text.len()));
        }

        // build up rail cypher
        for pos in 0..text.len() {
            let row_with_char = pos % self.rails;
            for row in 0..self.rails {
                if row == row_with_char {
                    vec[row].push(text.chars().nth(pos).unwrap());
                } else {
                    vec[row].push('.');
                }
            }
        }

        // get encoded str in rail cypher
        for rail in 0..self.rails {
            for c in vec[rail].chars() {
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
