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
pub fn uncompresses_data() {
    let data = vec![0xFE, 0xFE, 0x0, 0x1, 0x0, 0x2];

    let mut c = Cursor::new(data);
    let mut f = RlewReader::new(&mut c);
    let mut buf = Vec::new();

    f.read_to_end(&mut buf);

    assert_eq!(vec![0x0, 0x2], buf);
}
