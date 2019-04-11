use std::collections::HashMap;

#[derive(Debug)]
pub enum Tags {
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

pub enum TagsTypes {
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
