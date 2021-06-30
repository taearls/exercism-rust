#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}
const MASK: u32 = 0b1111111;
const IS_REMAINED: u8 = 0b10000000;
const IS_LAST: u8 = 0b00000000;
/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for value in values.iter().rev() {
        let mut val = *value; 
        result.push((val as u8) & 0b0111_1111);
        val >>= 7;

        while val > 0 {
            result.push((val as u8) | 0b1000_0000);
            val >>= 7;
        }
    }
    result.reverse();
    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result: Vec<u32> = Vec::new(); 
    let mut is_complete = false;
    let mut sum = 0u32;

    for byte in bytes.iter() {
        let val = (*byte as u32) & 0b0111_1111; // drop 1 in 8th bit if it exists
        is_complete = (*byte & 0b1000_0000) == 0; // check if 8th bit is 0. if it is, then we know the encoded number has ended.

        // since we're reading 7 bits at a time, have to multiply by 2^7 before adding.
        sum = sum.checked_mul(128).ok_or(Error::Overflow)?;
        sum = sum.checked_add(val).ok_or(Error::Overflow)?;

        if is_complete {
            result.push(sum);
            sum = 0;
        }
    }

    if is_complete {
        Ok(result)
    } else {
        // if the last encoded 8 bit string doesn't lead with a 0, it's incomplete
        Err(Error::IncompleteNumber)
    }
}
