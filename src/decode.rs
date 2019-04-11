use std::collections::HashMap;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use super::types::{Tags, TagsTypes};

pub fn vec_to_tags(raw: Vec<u8>) -> Result<Tags, &'static str> {

    let mut data = Cursor::new(raw);

    cursor_to_tags(&mut data)
}

fn cursor_to_tags(data: &mut Cursor<Vec<u8>>) -> Result<Tags, &'static str> {

    let tag_type = data.read_u8().map_err(|_| "Unable to read tag type.")?;

    let tag_type = TagsTypes::from_binary(tag_type).ok_or("Invalid tag type.")?;

    Ok(match tag_type {

        TagsTypes::End => Tags::End,

        TagsTypes::Byte => Tags::Byte(
            data.read_u8()
                .map_err(|_| "Unable to read Byte.")? as i8
        ),
        TagsTypes::Short => Tags::Short(
            data.read_u16::<BigEndian>()
                .map_err(|_| "Unable to read Short.")? as i16
        ),
        TagsTypes::Int => Tags::Int(
            data.read_u32::<BigEndian>()
                .map_err(|_| "Unable to read tag Int.")? as i32
        ),
        TagsTypes::Long => Tags::Long(
            data.read_u64::<BigEndian>()
                .map_err(|_| "Unable to read tag Long.")? as i64
        ),
        
        TagsTypes::Float => Tags::Float(
            data.read_f32::<BigEndian>()
                .map_err(|_| "Unable to read tag Float.")? as f32
        ),
        TagsTypes::Double => Tags::Double(
            data.read_f64::<BigEndian>()
                .map_err(|_| "Unable to read tag Double.")? as f64
        ),

        TagsTypes::String => Tags::String(cursor_to_string(data)?),
        TagsTypes::Array => Tags::Array(cursor_to_array(data)?),
        TagsTypes::Map => Tags::Map(cursor_to_map(data)?),
    })
}

fn cursor_to_string(_data: &mut Cursor<Vec<u8>>) -> Result<String, &'static str> {
    Ok("Not implemented".to_owned())
}

fn cursor_to_array(_data: &mut Cursor<Vec<u8>>) -> Result<Vec<Tags>, &'static str> {
    Ok(Vec::new())
}

fn cursor_to_map(_data: &mut Cursor<Vec<u8>>) -> Result<HashMap<String, Tags>, &'static str> {
    Ok(HashMap::new())
}
