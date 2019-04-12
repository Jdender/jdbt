use byteorder::{BigEndian, WriteBytesExt};
use super::types::{Tag, TagType};

pub fn encode(tag: Tag) -> Result<Vec<u8>, &'static str> {

    let mut data = vec![];

    tag_to_vec(tag, &mut data)?;

    Ok(data)
}

fn tag_to_vec(tag: Tag, data: &mut Vec<u8>) -> Result<(), &'static str> {

    data.write_u8(tag.to_binary()).map_err(|_| "Unable to write type.")?;

    match tag {

        Tag::End => data.write_u8(0).map_err(|_| "Unable to write End.")?,

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

        Tag::String(val) => data.write_u8(0).map_err(|_| "Unable to write Byte.")?,
        Tag::Array(val)  => data.write_u8(0).map_err(|_| "Unable to write Byte.")?,
        Tag::Map(val)    => data.write_u8(0).map_err(|_| "Unable to write Byte.")?,
    };

    Ok(())
}
