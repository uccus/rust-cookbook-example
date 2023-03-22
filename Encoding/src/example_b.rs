use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Error;

#[derive(Default, Debug)]
struct Payload{
    kind: u8,
    value: u16,
}

pub fn test(){
    let mut data = Payload::default();
    data.kind = 12;
    data.value = 4000;
    println!("{:?}", data);
    let mut arr = vec![];
    arr.write_u8(data.kind).unwrap();
    arr.write_u16::<LittleEndian>(data.value).unwrap();
    println!("{:?}", arr);
}