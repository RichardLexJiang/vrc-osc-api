use crate::osc::{message::OscMessage, value::OSCValue};

/// - `msg` Input text into the chatbox.
/// - If `bypass_keyboard` is True, send the text in `msg` immediately, bypassing the keyboard. If `bypass_keyboard` is False, open the keyboard and populate it with the provided text.  
/// - `sfx_trigger` is an additional bool parameter that when set to False will not trigger the notification SFX (defaults to True if not specified).
pub struct ChatboxInput {
    msg: String,
    bypass_keyboard: bool,
    sfx_trigger: bool,
}

impl ChatboxInput {
    pub fn new(s: &str) -> Self {
        Self {
            msg: s.to_string(),
            ..Self::default()
        }
    }
}

impl From<ChatboxInput> for OscMessage {
    fn from(value: ChatboxInput) -> Self {
        OscMessage::new("/chatbox/input")
            .arg(OSCValue::OSCString(value.msg))
            .arg(OSCValue::Bool(value.bypass_keyboard))
            .arg(OSCValue::Bool(value.sfx_trigger))
    }
}

impl Default for ChatboxInput {
    fn default() -> Self {
        Self {
            msg: Default::default(),
            bypass_keyboard: true,
            sfx_trigger: true,
        }
    }
}
