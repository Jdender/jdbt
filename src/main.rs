use decode::decode;
use encode::encode;
use types::Tag;

mod types;
mod decode;
mod encode;

#[cfg(test)]
mod tests;

fn main() {

    let tag = Tag::String("foo bar".to_owned());

    let data = encode(tag).unwrap();

    let result = decode(data).unwrap();

    println!("{:?}", result);
}

