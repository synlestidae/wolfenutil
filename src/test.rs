use crate::map_head::MapHead;
use crate::rlew_reader::RlewReader;
use std::fs::File;
use std::io::{Read, Cursor};

#[test]
pub fn loads_headers() {
    let mut f = File::open("MAPHEAD.WL6").unwrap();
    let head = MapHead::read_from(&mut f).unwrap();
    assert_eq!(0, head.tile_info.len());
    assert_eq!(0xABCD, head.rlew_tag);
}

#[test]
pub fn decompresses_vec() {
    let data = vec![0xFE, 0xFE, 0x0, 0x1, 0x0, 0x2];

    let mut c = Cursor::new(data);
    let mut f = RlewReader::new(&mut c);
    let mut buf = Vec::new();

    f.read_to_end(&mut buf);

    assert_eq!(vec![0x0, 0x2], buf);
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
pub fn reads_headers() {
   let mut f = File::open("MAPHEAD.WL6").unwrap();
   let mut reader = RlewReader::new(&mut f);
   let header = MapHead::read_from(&mut reader).unwrap(); 

   assert_eq!(header.header_offsets[99], 0);
   assert_eq!(header.tile_info.len(), 0);
}
