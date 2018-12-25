use std::io::Read;
use crate::rlew_reader::RlewReader;

pub struct MapHead {
    pub rlew_tag: u16,
    pub header_offsets: [u32; 100],
    pub tile_info: Vec<u8>
}

impl MapHead {
    pub fn parse(s: &mut Read) -> Option<MapHead> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut index: usize = 0;

        let mut map_head = MapHead { 
            rlew_tag: 0,
            header_offsets: [0; 100],
            tile_info: Vec::new()
        };


        let mut r = RlewReader::new(s);

        map_head.rlew_tag = r.read_u16().unwrap();

        index = 2;

        for i in 0..100 {
            map_head.header_offsets[i] = r.read_u32().unwrap();
        }

        let mut tile_info = Vec::new();

        r.read_to_end(&mut tile_info);

        map_head.tile_info = tile_info;

        Some(map_head)
    }
}

