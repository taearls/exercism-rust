use std::io::{Read, Result, Write};

pub struct ReadStats<R>{
    data: R
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            data: wrapped
        }
    }

    pub fn get_ref(&self) -> &R {
        unimplemented!()
    }

    pub fn bytes_through(&self) -> usize {
        unimplemented!()
    }

    pub fn reads(&self) -> usize {
        unimplemented!()
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        unimplemented!("Collect statistics about this call reading {:?}", buf)
    }
}

pub struct WriteStats<W>{
    data: W,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            data: wrapped
        }
    }

    pub fn get_ref(&self) -> &W {
        unimplemented!()
    }

    pub fn bytes_through(&self) -> usize {
        unimplemented!()
    }

    pub fn writes(&self) -> usize {
        unimplemented!()
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        unimplemented!("Collect statistics about this call writing {:?}", buf)
    }

    fn flush(&mut self) -> Result<()> {
        unimplemented!()
    }
}
