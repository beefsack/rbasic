use std::io::{Read,Write,Result};
use std::iter;

const DEFAULT_BUF_SIZE: usize = 64 * 1024;

pub struct PeekReader<R> {
    inner: R,
    buf: Vec<u8>,
}

pub trait PeekRead: Read {
    fn peek(&mut self, buf: &mut [u8]) -> Result<usize>;
}

impl<R: Read> PeekReader<R> {
    pub fn new(inner: R) -> PeekReader<R> {
        PeekReader::with_capacity(DEFAULT_BUF_SIZE, inner)
    }

    pub fn with_capacity(cap: usize, inner: R) -> PeekReader<R> {
        PeekReader {
            inner: inner,
            buf: Vec::with_capacity(cap),
        }
    }

    pub fn consume(&mut self, amt: usize) {
        for _ in 0..amt {
            self.buf.remove(0);
        }
    }
}

impl <R: Read> Read for PeekReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.buf.len() == 0 {
            return self.inner.read(buf);
        }
        let nread = try!((&self.buf[..]).read(buf));
        self.consume(nread);
        Ok(nread)
    }
}

impl <R: Read> PeekRead for PeekReader<R> {
    fn peek(&mut self, buf: &mut [u8]) -> Result<usize> {
        if buf.len() > self.buf.len() {
            let diff = buf.len() - self.buf.len();
            let mut rem_buf = Vec::with_capacity(diff);
            rem_buf.extend(iter::repeat(0).take(diff));
            let nread = try!(self.inner.read(&mut rem_buf));
            try!(Write::write(&mut self.buf, &rem_buf[..nread]));
        }
        (&self.buf[..]).read(buf)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::io::{Read,BufReader};
    use super::*;

    #[test]
    fn test_peek() {
        let input = "farts and cheese oh yes please";
        let mut reader = PeekReader::new(BufReader::new(input.as_bytes()));
        let mini: &mut[u8] = &mut[0; 5];
        let med: &mut[u8] = &mut[0; 10];

        assert!(reader.peek(mini).is_ok());
        assert_eq!("farts", String::from_utf8_lossy(mini));

        assert!(reader.peek(med).is_ok());
        assert_eq!("farts and ", String::from_utf8_lossy(med));

        assert!(reader.read(mini).is_ok());
        assert_eq!("farts", String::from_utf8_lossy(mini));

        assert!(reader.peek(med).is_ok());
        assert_eq!(" and chees", String::from_utf8_lossy(med));

        assert!(reader.read(med).is_ok());
        assert_eq!(" and chees", String::from_utf8_lossy(med));

        assert!(reader.read(med).is_ok());
        assert_eq!("e oh yes p", String::from_utf8_lossy(med));

        assert!(reader.peek(mini).is_ok());
        assert_eq!("lease", String::from_utf8_lossy(mini));

        assert!(reader.peek(med).is_ok());
        assert_eq!("lease", String::from_utf8_lossy(&med[..5]));

        assert!(reader.read(med).is_ok());
        assert_eq!("lease", String::from_utf8_lossy(&med[..5]));
    }
}
