use std::fs;
use std::collections::HashMap;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

fn main() {

    let data = vec![1, 5, 1];

    let result = vec_to_tags(data).unwrap();

    println!("{:?}", result);
}

fn vec_to_tags(raw: Vec<u8>) -> Result<Tags, &'static str> {

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

#[derive(Debug)]
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

impl Tags {

    pub fn to_type(&self) -> TagsTypes {
        
        match self {
            Tags::End       => TagsTypes::End,
            Tags::Byte(_)   => TagsTypes::Byte,
            Tags::Short(_)  => TagsTypes::Short,
            Tags::Int(_)    => TagsTypes::Int,
            Tags::Long(_)   => TagsTypes::Long,
            Tags::Float(_)  => TagsTypes::Float,
            Tags::Double(_) => TagsTypes::Double,
            Tags::String(_) => TagsTypes::String,
            Tags::Array(_)  => TagsTypes::Array,
            Tags::Map(_)    => TagsTypes::Map,
        }
    }

    pub fn to_binary(&self) -> u8 {

        self.to_type().to_binary()
    }
}

enum TagsTypes {
    End,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    String,
    Array,
    Map,
}

impl TagsTypes {

    pub fn from_binary(tag: u8) -> Option<TagsTypes> {

        match tag {
            0 => Some(TagsTypes::End),
            1 => Some(TagsTypes::Byte),
            2 => Some(TagsTypes::Short),
            3 => Some(TagsTypes::Int),
            4 => Some(TagsTypes::Long),
            5 => Some(TagsTypes::Float),
            6 => Some(TagsTypes::Double),
            7 => Some(TagsTypes::String),
            8 => Some(TagsTypes::Array),
            9 => Some(TagsTypes::Map),
            _ => None,
        }
    }

    pub fn to_binary(&self) -> u8 {

        match self {
            TagsTypes::End    => 0,
            TagsTypes::Byte   => 1,
            TagsTypes::Short  => 2,
            TagsTypes::Int    => 3,
            TagsTypes::Long   => 4,
            TagsTypes::Float  => 5,
            TagsTypes::Double => 6,
            TagsTypes::String => 7,
            TagsTypes::Array  => 8,
            TagsTypes::Map    => 9,
        }
    }
}
