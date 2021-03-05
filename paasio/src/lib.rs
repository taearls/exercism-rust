use std::io::{Read, Result, Write};

pub struct ReadStats<R>{
    data: R,
    bytes_thru: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        // for byte in wrapped.bytes() {
        //     data_bytes += 1;
        // }
        ReadStats {
            data: wrapped,
            bytes_thru: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_thru
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        self.data.read(buf).map(|bytes| {
            self.bytes_thru += bytes;
            bytes
        })   
    }
}

pub struct WriteStats<W>{
    data: W,
    bytes_thru: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            data: wrapped,
            bytes_thru: 0,
            writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_thru
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        self.data.write(buf).map(|bytes| {
            self.bytes_thru = bytes;
            bytes
        })
    }

    fn flush(&mut self) -> Result<()> {
        unimplemented!()
    }
}
