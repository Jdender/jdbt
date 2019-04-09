use std::fs;
use std::collections::HashMap;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

fn main() {

    let contents = fs::read("hmm.txt")
        .expect("Something went wrong reading the file");

    println!("{:x?}", contents);
}

fn vec_to_tags(raw: Vec<u8>) -> Tags {

    let mut data = Cursor::new(raw);

    data.read_u16::<BigEndian>();
}

fn _vec_to_tags(data: impl ReadBytesExt) -> Tags {

    data.read_u16::<BigEndian>();
}

enum Tags {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Array(Vec<Tags>),
    Map(HashMap<String, Tags>),
}
