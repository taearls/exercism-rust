#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for value in values.iter().rev() {
        let mut temp = *value; 
        result.push((temp as u8) & 0x7F);
        temp >>= 7;

        while temp > 0 {
            result.push((temp as u8) | 0x80);
            temp >>= 7;
        }
    }
    result.reverse();
    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    unimplemented!("Convert the list of bytes {:?} to a list of numbers", bytes)
}
