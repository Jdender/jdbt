use std::collections::HashMap;

#[derive(Debug)]
pub enum Tag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Array(Vec<Tag>),
    Map(HashMap<String, Tag>),
}

impl Tag {

    pub fn to_type(&self) -> TagType {
        
        match self {
            Tag::Byte(_)   => TagType::Byte,
            Tag::Short(_)  => TagType::Short,
            Tag::Int(_)    => TagType::Int,
            Tag::Long(_)   => TagType::Long,
            Tag::Float(_)  => TagType::Float,
            Tag::Double(_) => TagType::Double,
            Tag::String(_) => TagType::String,
            Tag::Array(_)  => TagType::Array,
            Tag::Map(_)    => TagType::Map,
        }
    }

    pub fn to_binary(&self) -> u8 {

        self.to_type().to_binary()
    }
}

pub enum TagType {
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

impl TagType {

    pub fn from_binary(tag: u8) -> Option<TagType> {

        match tag {
            1 => Some(TagType::Byte),
            2 => Some(TagType::Short),
            3 => Some(TagType::Int),
            4 => Some(TagType::Long),
            5 => Some(TagType::Float),
            6 => Some(TagType::Double),
            7 => Some(TagType::String),
            8 => Some(TagType::Array),
            9 => Some(TagType::Map),
            _ => None,
        }
    }

    pub fn to_binary(&self) -> u8 {

        match self {
            TagType::Byte   => 1,
            TagType::Short  => 2,
            TagType::Int    => 3,
            TagType::Long   => 4,
            TagType::Float  => 5,
            TagType::Double => 6,
            TagType::String => 7,
            TagType::Array  => 8,
            TagType::Map    => 9,
        }
    }
}
