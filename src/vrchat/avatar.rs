use crate::to_osc_message;

to_osc_message!(AvatarEyeheight, "/avatar/eyeheight", meter);
to_osc_message!(AvatarEyeheightMin, "/avatar/eyeheightmin", min);
to_osc_message!(AvatarEyeheightMax, "/avatar/eyeheightmax", max);
to_osc_message!(
    AvatarEyeheightScalingAllowed,
    "/avatar/eyeheightscalingallowed",
    trigger
);

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

impl Default for AvatarEyeheight {
    fn default() -> Self {
        Self { meter: 1.0 }
    }
}

/// The minimum user-selectable eye height configured by Udon, defaults to 0.2. Does not affect settable range via OSC
#[derive(Debug)]
pub struct AvatarEyeheightMin {
    min: f32,
}

impl AvatarEyeheightMin {
    pub fn new(min: f32) -> Self {
        Self { min }
    }
}

/// The maximum user-selectable eye height configured by Udon, defaults to 5.0. Does not affect settable range via OSC.
#[derive(Debug)]
pub struct AvatarEyeheightMax {
    max: f32,
}

impl AvatarEyeheightMax {
    pub fn new(max: f32) -> Self {
        Self { max }
    }
}

///False, if disabled by Udon or world tag (website). When false, writes to eyeheight will be ignored.
#[derive(Debug)]
pub struct AvatarEyeheightScalingAllowed {
    trigger: bool,
}

impl AvatarEyeheightScalingAllowed {
    pub fn new(trigger: bool) -> Self {
        Self { trigger }
    }
}
