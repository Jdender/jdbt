use super::decode::decode;
use super::encode::encode;
use super::types::Tag;

fn encode_decode_test_helper(tag: Tag) {

    let binary = encode(tag.clone()).unwrap();
    
    let result = decode(binary).unwrap();

    assert_eq!(result, tag)
}

#[test]
fn byte() {
    encode_decode_test_helper(Tag::Byte(1))
}

#[test]
fn short() {
    encode_decode_test_helper(Tag::Short(1))
}

#[test]
fn int() {
    encode_decode_test_helper(Tag::Int(1))
}

#[test]
fn long() {
    encode_decode_test_helper(Tag::Long(1))
}

#[test]
fn float() {
    encode_decode_test_helper(Tag::Float(1.0))
}

#[test]
fn double() {
    encode_decode_test_helper(Tag::Double(1.0))
}

#[test]
fn string() {
    encode_decode_test_helper(Tag::String("wew".to_owned()))
}
