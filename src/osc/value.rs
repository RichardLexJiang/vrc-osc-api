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
        match &self {
            OSCValue::Int32(_) => 'i' as u8,
            OSCValue::Float32(_) => 'f' as u8,
            OSCValue::OSCString(_) => 's' as u8,
            OSCValue::OSCBlob(_) => 'b' as u8,
            OSCValue::Int64(_) => 'h' as u8,
            OSCValue::OSCTimetag(_) => 't' as u8,
            OSCValue::Float64(_) => 'd' as u8,
            OSCValue::AltString => 'S' as u8,
            OSCValue::ASCIICharacter(_) => 'c' as u8,
            OSCValue::RGBAColor(_) => 'r' as u8,
            OSCValue::MIDIMessage(_) => 'm' as u8,
            OSCValue::Bool(b) => (if *b { 'T' } else { 'F' }) as u8,
            OSCValue::Nil => 'N' as u8,
            OSCValue::Infinitum => 'I' as u8,
            OSCValue::LeftSquareBracket => '[' as u8,
            OSCValue::RightSquareBracket => ']' as u8,
        }
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
macro_rules! to_osc_value {
    ($type:ty,$ov:ident) => {
        impl From<$type> for OSCValue {
            fn from(v: $type) -> Self {
                OSCValue::$ov(v)
            }
        }
    };
}

to_osc_value!(i32, Int32);
to_osc_value!(i64, Int64);
to_osc_value!(bool, Bool);
to_osc_value!(String, OSCString);
to_osc_value!(f32, Float32);
