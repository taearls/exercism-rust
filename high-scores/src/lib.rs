#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.iter().copied().collect()
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.len() == 0 {
            return None
        }
        let scores_len = self.scores.len();
        Some(self.scores[scores_len - 1])
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.len() == 0 {
            return None
        }
        let mut best_score: u32 = 0;
        for i in &self.scores {
            if i > &best_score {
                best_score = *i;
            }
        }
        Some(best_score)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let top_three: Vec<u32> = match self.scores.len() {
            0 => Vec::new(),
            1 => vec![self.scores[0]],
            2 => {
                let mut vec = vec![self.scores[0], self.scores[1]];
                if vec[1] > vec[0] {
                    vec = vec![self.scores[1], self.scores[0]];
                }
                vec
            }
            _ => {
                // highest to lowest
                let mut top_three = vec![0, 0, 0];
                for i in &self.scores {
                    if i > &top_three[0] {
                        // move other two to the right before changing highest score
                        top_three[2] = top_three[1];
                        top_three[1] = top_three[0];
                        top_three[0] = *i;
                    } else if i > &top_three[1] {
                        // move 2nd highest to 3rd slot before changing 2nd highest score
                        top_three[2] = top_three[1];
                        top_three[1] = *i;
                    } else if i > &top_three[2] {
                        top_three[2] = *i;
                    }
                }
                top_three
            }
        };
        top_three
    }
}
