use std::io::Read;
use std::io::Cursor;
use crate::rlew_reader::RlewReader;

#[derive(Debug)]
pub struct MapType {
   pub plane_start: [u32; 3], 
   pub plane_length: [u16; 3], 
   pub width: u16,
   pub height: u16,
   pub name: String
}

impl MapType {
    pub fn read_map(&self, data: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
        let p1 = self.plane_start[0] as usize;
        let p1_len = self.plane_length[0] as usize;

        let p2 = self.plane_start[1] as usize;
        let p2_len = self.plane_length[1] as usize;

        let v1 = data[p1..p1_len].to_vec();
        let v2 = data[p2..p2_len].to_vec();

        Some((v1, v2))
    }

    pub fn parse(data: &[u8]) -> Option<Self> {
        let mut cursor = Cursor::new(data);

        let mut reader = RlewReader::new(&mut cursor);

        Some(MapType {
            plane_start: [reader.read_u32().unwrap(), reader.read_u32().unwrap(), reader.read_u32().unwrap()],
            plane_length: [reader.read_u16().unwrap(), reader.read_u16().unwrap(), reader.read_u16().unwrap()],
            width: reader.read_u16().unwrap(),
            height: reader.read_u16().unwrap(),
            name: read_c_string(&((0..16).into_iter().map(|_| reader.read_u8().unwrap()).collect::<Vec<u8>>()))
        })
    }
}

fn read_c_string(buf: &[u8]) -> String {
    let mut string = String::new();
    for &b in buf {
        if b == 0 {
            return string
        }
        if b < 127 {
            string.push(b as char); 
        } 
    }
    string
}
