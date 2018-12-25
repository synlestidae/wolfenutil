use std::io::Read;
use std::io::Cursor;
use crate::rlew_reader::RlewReader;

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

        let mut buf = Vec::new();

        buf.resize(26, 0u8);

        if let Ok(26) = reader.read(&mut buf) {
            let mut map_type = MapType {
                plane_start: [reader.read_u32().unwrap(), reader.read_u32().unwrap(), reader.read_u32().unwrap()],
                plane_length: [reader.read_u16().unwrap(), reader.read_u16().unwrap(), reader.read_u16().unwrap()],
                width: reader.read_u16().unwrap(),
                height: reader.read_u16().unwrap(),
                name: String::new()
            };

            Some(map_type)
        } else {
            None
        }
    }
}
