use crate::map_head::MapHead;
use crate::map_type::MapType;
use crate::map::Map;
use crate::rlew_reader::RlewReader;
use std::io::Read;
use std::io::Cursor;
use crate::plane::Plane;

pub struct MapBuilder {
    map_head: MapHead,
    //map_type: MapType
    data: Vec<u8>
}

impl MapBuilder {
    pub fn new(map_head: MapHead, data: Vec<u8>) -> Self {
        Self {
            map_head: map_head,
            data: data
        }
    }

    pub fn build(&self, map_idx: u16) -> Option<Map> {
        // idx in the data
        // parse the MapType
        // get the indexed data
        
        let offset = self.map_head.header_offsets[map_idx as usize] as usize;

        let m_end = self.data.len();

        let map_type = MapType::parse(&self.data[offset..m_end])?;

        println!("{:?}", map_type);

        let mut plane1_data: Vec<u8> = Vec::new();
        let p1_start = map_type.plane_start[0] as usize;
        let p1_length = map_type.plane_length[0] as usize;

        let mut plane2_data: Vec<u8> = Vec::new();
        let p2_start = map_type.plane_start[1] as usize;
        let p2_length = map_type.plane_length[1] as usize;

        let mut c = Cursor::new(&self.data);
        let mut reader = RlewReader::new(&mut c);

        println!("Read 1 {} {}", p1_start, p1_length);
        println!("Read 2 {} {}", p2_start, p2_length);

        let mut plane1_data = reader.read_offset(p1_start, p1_length, 64 * 64);
        let mut plane2_data = reader.read_offset(p2_start, p2_length, 64 * 64);

        Some(Map {
            width: map_type.width, 
            height: map_type.height, 
            name: map_type.name,
            plane1: Plane {
                data: plane1_data
            },
            plane2: Plane {
                data: plane2_data
            }
        })
    }


}
