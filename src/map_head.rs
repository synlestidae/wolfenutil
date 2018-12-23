use std::io::Read;

pub struct MapHead {
    pub rlew_tag: u16,
    pub header_offsets: [u32; 100],
    pub tile_info: Vec<u8>
}

impl MapHead {
    pub fn read_from(s: &mut Read) -> Option<MapHead> {
        let mut buffer = Vec::new();
        let mut index: usize = 0;

        let mut map_head = MapHead { 
            rlew_tag: 0,
            header_offsets: [0; 100],
            tile_info: Vec::new()
        };

        s.read_to_end(&mut buffer).unwrap();

        println!("{}", buffer.len());

        map_head.rlew_tag = ((buffer[1] as u16) << 8) + buffer[0] as u16;

        index = 2;

        while index < 400 {
            map_head.header_offsets[index / 4] = 
                (buffer[index] as u32) +
                ((buffer[index + 1] as u32) << 8) +
                ((buffer[index + 2] as u32) << 16) +
                ((buffer[index + 3] as u32) << 24);

            index += 4;
        }

        println!("{} {}", index, buffer.len());
        map_head.tile_info = buffer[index..buffer.len()].to_vec();

        Some(map_head)
    }
}

