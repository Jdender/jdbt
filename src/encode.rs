use std::collections::HashMap;
use byteorder::{BigEndian, WriteBytesExt};
use super::types::Tag;

pub fn encode(tag: Tag) -> Result<Vec<u8>, &'static str> {

    let mut data = vec![];

    tag_and_type_into_vec(tag, &mut data)?;

    Ok(data)
}

fn tag_and_type_into_vec(tag: Tag, data: &mut Vec<u8>) -> Result<(), &'static str> {

    data.write_u8(tag.to_binary()).map_err(|_| "Unable to write type.")?;

    tag_into_vec(tag, data)
}

fn tag_into_vec(tag: Tag, data: &mut Vec<u8>) -> Result<(), &'static str> {

    match tag {
        Tag::Byte(val)  => 
            data.write_u8(val as u8)
                .map_err(|_| "Unable to write Byte.")?,
        Tag::Short(val) => 
            data.write_u16::<BigEndian>(val as u16)
                .map_err(|_| "Unable to write Short.")?,
        Tag::Int(val)   => 
            data.write_u32::<BigEndian>(val as u32)
                .map_err(|_| "Unable to write Int.")?,
        Tag::Long(val)  => 
            data.write_u64::<BigEndian>(val as u64)
                .map_err(|_| "Unable to write Long.")?,
        
        Tag::Float(val)  => 
            data.write_f32::<BigEndian>(val)
                .map_err(|_| "Unable to write Float.")?,
        Tag::Double(val) => 
            data.write_f64::<BigEndian>(val)
                .map_err(|_| "Unable to write Double.")?,

        Tag::String(val) => string_into_vec(val, data)?,
        Tag::Array(val)  => array_into_vec(val, data)?,
        Tag::Map(val)    => map_into_vec(val, data)?,
    };

    Ok(())
}

fn string_into_vec(tag: String, data: &mut Vec<u8>) -> Result<(), &'static str> {

    data.write_u16::<BigEndian>(tag.len() as u16)
        .map_err(|_| "Unable to write String length.")?;

    for byte in tag.bytes() {
        data.write_u8(byte)
            .map_err(|_| "Unable to write String.")?;
    }

    Ok(())
}

fn array_into_vec(_tag: Vec<Tag>, _data: &mut Vec<u8>) -> Result<(), &'static str> {
    Ok(())
}

fn map_into_vec(_tag: HashMap<String, Tag>, _data: &mut Vec<u8>) -> Result<(), &'static str> {
    Ok(())
}
