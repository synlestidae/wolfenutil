use std::io::{Read, Result};
use std::cmp::min;

pub struct RlewReader<'a> {
    read: &'a mut Read,
    index: usize,
    uncompressed_index: usize,
    last_was_fe: bool,
    compressed_buf: Vec<u8>,
    uncompressed_buf: Vec<u8>
}

impl<'a> RlewReader<'a> {
    pub fn new(r: &'a mut Read) -> Self {
        let mut buf = Vec::new();

        r.read_to_end(&mut buf);

        Self {
            read: r,
            uncompressed_index: 0,
            index: 0,
            last_was_fe: false,
            compressed_buf: buf,
            uncompressed_buf: Vec::new()
        }
    }


    fn read_word(&self, segment: &[u8], i: &mut usize) -> u16 {
        let (b1, b2) = self.read_word_bytes(segment, i);

        ((b1 as u16) << 8) + (b2 as u16)
    }

    fn read_word_bytes(&self, segment: &[u8], i: &mut usize) -> (u8, u8) {
        let t = (segment[*i], segment[*i + 1]);
        *i += 2;
        t
    }
}

impl<'a> Read for RlewReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let start = self.index;

        let end = min(self.index + buf.len(), self.compressed_buf.len());

        let segment = &self.compressed_buf[start..end];
        let mut i = 0usize;

        while i < segment.len() {
            let word = self.read_word(&segment, &mut i);

            if word == 0xFEFE {
                let len = self.read_word(&segment, &mut i);
                let (b1, b2) = self.read_word_bytes(&segment, &mut i);

                for j in 0..len {
                    self.uncompressed_buf.push(b1);
                    self.uncompressed_buf.push(b2);
                }
            } else {
                self.uncompressed_buf.push((word >> 8) as u8);
                self.uncompressed_buf.push(word as u8);
            }
        }

        self.index += segment.len();

        // now get the good stuff
       
        let u_start = self.uncompressed_index;  
        let u_end = min(self.uncompressed_index + buf.len(), self.uncompressed_buf.len());

        for i in u_start..u_end {
            buf[i] = self.uncompressed_buf[i]
        }

        self.uncompressed_index = u_end;
        
        Ok(u_end - u_start)
    }
}
