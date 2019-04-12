use decode::decode;
use encode::encode;
use types::Tag;

mod types;
mod decode;
mod encode;

fn main() {

    let tag = Tag::Byte(1);

    let data = encode(tag).unwrap();

    let result = decode(data).unwrap();

    println!("{:?}", result);
}

