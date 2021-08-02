const STOP_CODON: &str = "stop codon";

pub struct CodonsInfo<'a> {
    pairs: Vec<(&'a str, &'a str)>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        if let Some(pair) = self.pairs.iter().find(|(c, _name)| *c == codon) {
            Some(pair.1)
        } else {
            None
        }
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut result: Vec<&'a str> = Vec::with_capacity(rna.len() / 3);
        let mut p = rna.chars().peekable();
        while p.peek().is_some() {
            let chunk: String = p.by_ref().take(3).collect();
            match self.name_for(&chunk) {
                Some(name) if name == STOP_CODON => break,
                Some(name) => result.push(name),
                _ => return None,
            }
        }
        Some(result)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo { pairs }
}
