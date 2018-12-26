use crate::map_head::MapHead;
use crate::rlew_reader::RlewReader;
use crate::map_builder::MapBuilder;
use std::fs::File;
use std::io::{Read, Cursor};

#[test]
pub fn loads_headers() {
    let mut f = File::open("MAPHEAD.WL6").unwrap();
    let head = MapHead::parse(&mut f).unwrap();
    assert_eq!(0, head.tile_info.len());
    assert_eq!(0xABCD, head.rlew_tag);
}

#[test]
pub fn decompresses_vec() {
    let data = vec![0xFE, 0xFE, 0x1, 0x0, 0x2, 0x0];

    let mut c = Cursor::new(data);
    let mut f = RlewReader::new(&mut c);
    let mut buf = Vec::new();

    f.read_to_end(&mut buf);

    assert_eq!(vec![0x2, 0x0], buf);
}

#[test]
pub fn decompresses_word() {
    let data = vec![0xFE, 0xFE, 0x1, 0x0, 0x2, 0x0];

    let mut c = Cursor::new(data);
    let mut f = RlewReader::new(&mut c);

    let v = f.read_u16().unwrap();

    assert_eq!(v, 0x2);
}

#[test]
pub fn reads_u16() {
    let data = vec![0x01, 0x00, 0x02, 0x00];

    let mut c = Cursor::new(data);
    let mut f = RlewReader::new(&mut c);

    assert_eq!(f.read_u16().unwrap(), 0x0001);
    assert_eq!(f.read_u16().unwrap(), 0x0002);
}

#[test]
pub fn reads_u32() {
    let data = vec![0x01, 0x00, 0x02, 0x00];

    let mut c = Cursor::new(data);
    let mut f = RlewReader::new(&mut c);

    assert_eq!(f.read_u32().unwrap(), 0x00020001);
}

#[test]
pub fn reads_u32_from_compressed() {
    let data = vec![0xFE, 0xFE, 0x04, 0x00, 0x10, 0x00];

    let mut c = Cursor::new(data);
    let mut f = RlewReader::new(&mut c);

    assert_eq!(f.read_u32().unwrap(), 0x00100010);
    assert_eq!(f.read_u32().unwrap(), 0x00100010);
}

#[test]
pub fn reads_headers() {
   let mut f = File::open("MAPHEAD.WL6").unwrap();
   let mut reader = RlewReader::new(&mut f);
   let header = MapHead::parse(&mut reader).unwrap(); 

   assert_eq!(header.header_offsets[99], 0);
   assert_eq!(header.tile_info.len(), 0);
}

#[test]
pub fn reads_map() {
   let head = MapHead::parse(&mut File::open("MAPHEAD.WL6").unwrap()).unwrap();

   let mut data = Vec::new(); 

   File::open("GAMEMAPS.WL6").unwrap().read_to_end(&mut data);

   let data = MapBuilder::new(head, data); 

   let map = data.build(0).unwrap();

   //assert_eq!(64 * 64, map.plane1.data.len());
   assert_eq!(64 * 64, map.plane2.data.len());
}


