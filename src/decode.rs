use std::collections::HashMap;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use super::types::{Tag, TagType};

pub fn decode(raw: Vec<u8>) -> Result<Tag, &'static str> {

    let mut data = Cursor::new(raw);

    cursor_to_tag(&mut data, None)
}

fn cursor_to_tag(data: &mut Cursor<Vec<u8>>, tag_type: Option<TagType>) -> Result<Tag, &'static str> {

    let tag_type = match tag_type {
        Some(tag_type) => tag_type,
        None => cursor_to_tag_type(data)?,
    };

    Ok(match tag_type {

        TagType::Byte => Tag::Byte(
            data.read_u8()
                .map_err(|_| "Unable to read Byte.")? as i8
        ),
        TagType::Short => Tag::Short(
            data.read_u16::<BigEndian>()
                .map_err(|_| "Unable to read Short.")? as i16
        ),
        TagType::Int => Tag::Int(
            data.read_u32::<BigEndian>()
                .map_err(|_| "Unable to read tag Int.")? as i32
        ),
        TagType::Long => Tag::Long(
            data.read_u64::<BigEndian>()
                .map_err(|_| "Unable to read tag Long.")? as i64
        ),
        
        TagType::Float => Tag::Float(
            data.read_f32::<BigEndian>()
                .map_err(|_| "Unable to read tag Float.")? as f32
        ),
        TagType::Double => Tag::Double(
            data.read_f64::<BigEndian>()
                .map_err(|_| "Unable to read tag Double.")? as f64
        ),

        TagType::String => Tag::String(cursor_to_string(data)?),
        TagType::Array => Tag::Array(cursor_to_array(data)?),
        TagType::Map => Tag::Map(cursor_to_map(data)?),
    })
}

fn cursor_to_tag_type(data: &mut Cursor<Vec<u8>>) -> Result<TagType, &'static str> {

    let tag_type = data.read_u8().map_err(|_| "Unable to read tag type.")?;

    TagType::from_binary(tag_type).ok_or("Invalid tag type.")
}

fn cursor_to_string(data: &mut Cursor<Vec<u8>>) -> Result<String, &'static str> {

    let length = data.read_u16::<BigEndian>().map_err(|_| "Unable to read String length.")?;

    let mut string: Vec<u8> = vec![];

    for _ in 0..length { 
        string.push(
            data.read_u8().map_err(|_| "Unable to read String.")?
        );
    }

    let string = String::from_utf8(string).map_err(|_| "Unable to read String as valid utf8.")?;

    Ok(string)
}

fn cursor_to_array(_data: &mut Cursor<Vec<u8>>) -> Result<Vec<Tag>, &'static str> {
    Ok(Vec::new())
}

fn cursor_to_map(_data: &mut Cursor<Vec<u8>>) -> Result<HashMap<String, Tag>, &'static str> {
    Ok(HashMap::new())
}
