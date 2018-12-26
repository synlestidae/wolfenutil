use std::io::{Read, Result};
use std::cmp::min;
use std::io::Cursor;

pub struct RlewReader<'a> {
    read: &'a mut Read,
    index: usize,
    uncompressed_buf: Vec<u8>,
    compressed_buf: Vec<u8>
}

impl<'a> RlewReader<'a> {
    pub fn new(r: &'a mut Read) -> Self {
        let mut buf = Vec::new();

        r.read_to_end(&mut buf);

        let r = Self {
            read: r,
            index: 0,
            uncompressed_buf: RlewReader::decompress(&buf),
            compressed_buf: buf
        };

        r
    }

    fn decompress(data: &[u8]) -> Vec<u8> {
        let mut i = 0usize;
        let mut decompressed: Vec<u8> = Vec::new();

        while i < data.len() {
            if i + 2 >= data.len()  {
                decompressed.push(data[i]);
                i += 1;
            } else {
                match read_word_bytes(&data, &mut i) {
                    ((0xFE, 0xFE)) => {
                        let len = read_word(&data, &mut i);
                        let (b1, b2) = read_word_bytes(&data, &mut i);

                        for j in 0..len {
                            decompressed.push(b1);
                            decompressed.push(b2);
                        }
                    },
                    ((b1, b2)) => {
                        decompressed.push(b1);
                        decompressed.push(b2);
                    }
                }
            }
        }

        decompressed
    }


    pub fn read_u32(&mut self) -> Option<u32> {
        let mut buf = vec![0, 0, 0, 0];
        if let Ok(4) = self.read(&mut buf) {
            let val = ((buf[3] as u32) << 24) +
                   ((buf[2] as u32) << 16) +
                   ((buf[1] as u32) << 8) +
                   ((buf[0] as u32));
            return Some(val);
        }
        None
    }

    pub fn read_offset(&mut self, idx: usize, offset: usize) -> Vec<u8> {
        let data = RlewReader::decompress(&self.compressed_buf);

        let mut c = Cursor::new(&self.compressed_buf[idx..(idx + offset)]);

        let mut reader = RlewReader::new(&mut c);

        let mut buf = Vec::new();

        buf.resize(offset, 0u8);

        reader.read(&mut buf);

        buf
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        let mut buf = vec![0, 0];
        if let Ok(2) = self.read(&mut buf) {
            return Some(((buf[1] as u16) << 8) + (buf[0] as u16));
        }
        None
    }
}

impl<'a> Read for RlewReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.index > self.uncompressed_buf.len() {
            Ok(0)
        } else {
            let start = self.index;
            let end = min(self.index + buf.len(), self.uncompressed_buf.len());
            let segment = &self.uncompressed_buf[start..end];

            for (i, b) in segment.iter().enumerate() {
                buf[i] = *b;//.copy_from_slice(segment);
            }

            self.index = end;

            Ok(end - start)
        }
    }
}

pub fn read_word(segment: &[u8], i: &mut usize) -> u16 {
    let (b1, b2) = read_word_bytes(segment, i);

    ((b2 as u16) << 8) + (b1 as u16)
}

pub fn read_word_bytes(segment: &[u8], i: &mut usize) -> (u8, u8) {
    let t = (segment[*i], segment[*i + 1]);
    *i += 2;
    t
}

