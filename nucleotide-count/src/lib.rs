use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let dna_charset = "ACGT";
    if !dna_charset.contains(nucleotide) {
        return Err(nucleotide);
    }     
    // A is valid, so if it's overwritten it's invalid
    let mut dna_invalid_match: char = 'A';
    for char in dna.chars() {
        if !dna_charset.contains(char) {
            dna_invalid_match = char;
            break;
        }
    }

    if dna_invalid_match != 'A' {
        return Err(dna_invalid_match);
    } else {
        let mut result: usize = 0;
        for char in dna.chars() {
             if nucleotide == char {
                 result += 1;
             }
        }
        Ok(result)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result: HashMap<char, usize> = HashMap::new();
    let dna_charset = "ACGT";
    let mut a_result: usize = 0;
    let mut c_result: usize = 0;
    let mut g_result: usize = 0;
    let mut t_result: usize = 0;
    for char in dna.chars() {
        match char {
            'A' => a_result += 1,
            'C' => c_result += 1,
            'G' => g_result += 1,
            'T' => t_result += 1,
            _ => return Err(char)
        }
    }
    result.insert('A', a_result);
    result.insert('C', c_result);
    result.insert('G', g_result);
    result.insert('T', t_result);
    
    Ok(result)
}
