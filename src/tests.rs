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
