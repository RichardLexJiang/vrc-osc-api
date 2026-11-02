use crate::osc::{message::OscMessage, value::OSCValue};

///
/// - address: /avatar/eyeheight  
///
/// [Offcial Doc](https://docs.vrchat.com/docs/osc-avatar-scaling)
#[derive(Debug)]
pub struct AvatarEyeheight {
    meter: f32,
}

impl AvatarEyeheight {
    pub fn new(meter: f32) -> Self {
        Self { meter }
    }
}

impl From<AvatarEyeheight> for OscMessage {
    fn from(value: AvatarEyeheight) -> Self {
        OscMessage::new("/avatar/eyeheight").arg(OSCValue::Float32(value.meter))
    }
}

impl Default for AvatarEyeheight {
    fn default() -> Self {
        Self { meter: 1.0 }
    }
}
