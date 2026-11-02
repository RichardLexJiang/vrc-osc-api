use crate::osc::message::OscMessage;

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
            .arg(value.msg)
            .arg(value.bypass_keyboard)
            .arg(value.sfx_trigger)
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

///  Toggle the typing indicator on or off.
#[derive(Debug)]
pub struct ChatboxTyping {
    toggle: bool,
}

impl From<ChatboxTyping> for OscMessage {
    fn from(value: ChatboxTyping) -> Self {
        OscMessage::new("/chatbox/typing").arg(value.toggle)
    }
}

impl ChatboxTyping {
    pub fn new(toggle: bool) -> Self {
        Self { toggle }
    }
}

#[cfg(test)]
mod test {
    use crate::{osc::message::OscMessage, vrchat::chatbox::ChatboxInput};

    #[test]
    fn test_chatbox_input() {
        assert_eq!(
            OscMessage::from(ChatboxInput::new("hi~")).encode(),
            vec![
                47, 99, 104, 97, 116, 98, 111, 120, 47, 105, 110, 112, 117, 116, 0, 0, 44, 115, 84,
                84, 0, 0, 0, 0, 104, 105, 126, 0
            ]
        );
    }
}
