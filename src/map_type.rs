use std::io::Read;
use crate::rlew_reader::RlewReader;

pub struct MapType {
   pub plan_start: [u32; 3], 
   pub plane_length: [u16; 3], 
   pub width: u16,
   pub height: u16,
   pub name: String
}

impl MapType {
    fn read_from(r: &mut Read) -> Option<Self> {
        let mut reader = RlewReader::new(r);

        let mut buf = Vec::new();

        buf.resize(26, 0u8);

        if let Ok(26) = reader.read(&mut buf) {
            let mut map_type = MapType {
                plan_start: [reader.read_u32().unwrap(), reader.read_u32().unwrap(), reader.read_u32().unwrap()],
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
