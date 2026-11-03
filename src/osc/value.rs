use crate::tools::byte_tools::realign_bytes;

pub enum OSCValue {
    Int32(i32),
    Float32(f32),
    OSCString(String),
    OSCBlob(Vec<u8>),
    Int64(i64),
    OSCTimetag(u64),
    Float64(f64),
    AltString,
    ASCIICharacter(u32),
    RGBAColor(u32),
    MIDIMessage(u32),
    Bool(bool),
    Nil,
    Infinitum,
    LeftSquareBracket,
    RightSquareBracket,
}

impl OSCValue {
    pub fn get_tag(&self) -> u8 {
        (match &self {
            OSCValue::Int32(_) => 'i',
            OSCValue::Float32(_) => 'f',
            OSCValue::OSCString(_) => 's',
            OSCValue::OSCBlob(_) => 'b',
            OSCValue::Int64(_) => 'h',
            OSCValue::OSCTimetag(_) => 't',
            OSCValue::Float64(_) => 'd',
            OSCValue::AltString => 'S',
            OSCValue::ASCIICharacter(_) => 'c',
            OSCValue::RGBAColor(_) => 'r',
            OSCValue::MIDIMessage(_) => 'm',
            OSCValue::Bool(b) => {
                if *b {
                    'T'
                } else {
                    'F'
                }
            }
            OSCValue::Nil => 'N',
            OSCValue::Infinitum => 'I',
            OSCValue::LeftSquareBracket => '[',
            OSCValue::RightSquareBracket => ']',
        }) as u8
    }

    pub fn write_to_buffer(&self, buffer: &mut Vec<u8>) {
        match self {
            OSCValue::Int32(i) => buffer.append(&mut i.to_be_bytes().to_vec()),
            OSCValue::Int64(i) => buffer.append(&mut i.to_be_bytes().to_vec()),
            OSCValue::Float32(f) => buffer.append(&mut f.to_be_bytes().to_vec()),
            OSCValue::Float64(f) => buffer.append(&mut f.to_be_bytes().to_vec()),
            OSCValue::OSCString(s) => {
                buffer.append(&mut s.as_bytes().to_vec());
                realign_bytes(buffer);
            }
            OSCValue::ASCIICharacter(c) => buffer.append(&mut c.to_be_bytes().to_vec()),
            OSCValue::Bool(_) => {}
            OSCValue::Nil => {}
            OSCValue::Infinitum => {}
            OSCValue::LeftSquareBracket => {}
            OSCValue::RightSquareBracket => {}
            OSCValue::OSCBlob(_) => todo!(),
            OSCValue::OSCTimetag(_) => todo!(),
            OSCValue::AltString => todo!(),
            OSCValue::RGBAColor(_) => todo!(),
            OSCValue::MIDIMessage(_) => todo!(),
        }
    }
}

/// impl From<xx> for OSCValue
macro_rules! oscvalue_mapping {
    ($type:ty=>$ov:ident) => {
        impl From<$type> for OSCValue {
            fn from(v: $type) -> Self {
                OSCValue::$ov(v)
            }
        }
    };
}

oscvalue_mapping!(i32=> Int32);
oscvalue_mapping!(i64=>Int64);
oscvalue_mapping!(bool=> Bool);
oscvalue_mapping!(String=> OSCString);
oscvalue_mapping!(f32=> Float32);
oscvalue_mapping!(f64=> Float64);
