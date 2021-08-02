pub struct CodonsInfo<'a> {
    // This field is here to make the template compile and not to
    // complain about unused type lifetime parameter "'a". Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
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
            if let Some(name) = self.name_for(&chunk) {
                result.push(&name);
            } else {
                return None;
            }
        }
        Some(result)

        // unimplemented!("Return a list of protein names that correspond to the '{}' RNA string or None if the RNA string is invalid", rna);
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo { pairs }
    // unimplemented!(
    //     "Construct a new CodonsInfo struct from given pairs: {:?}",
    //     pairs
    // );
}
