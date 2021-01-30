#[derive(Debug, PartialEq)]
pub struct Dna {
    str: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    str: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (index, i) in dna.char_indices() {
            match i {
                'A' | 'C' | 'G' | 'T' => (),
                _ => return Err(index),
            }
        }
        let dna = Dna {
            str: dna.to_string()
        };
        Ok(dna)
    }

    pub fn into_rna(self) -> Rna {
        let mut str = String::new();

        for i in self.str.chars() {
            match i {
                'G' => str.push('C'),
                'C' => str.push('G'),
                'T' => str.push('A'),
                'A' => str.push('U'),
                _ => (),
            }
        }
        Rna {
            str: str
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (index, i) in rna.char_indices() {
            match i {
                'A' | 'C' | 'G' | 'U' => (),
                _ => return Err(index),
            }
        }
        let rna = Rna {
            str: rna.to_string()
        };
        Ok(rna)    
    }
}
