/// impl From<xx> for OscMessage
#[macro_export]
macro_rules! to_osc_message {
    // main capture
    ($from:ident, $address:literal, $($args:tt)*) => {
        impl From<$from> for crate::osc::message::OscMessage {
            fn from(v: $from) -> Self {
                $crate::to_osc_message!(
                    @recursive v;crate::osc::message::OscMessage::new($address); $($args)*
                )
            }
        }
    };
    // capture OSCValue
    (@recursive $v:ident;$message:expr; OSCValue::$variant:ident($field:ident) $(, $($rest:tt)*)?) => {
        $crate::to_osc_message!(
            @recursive $v;$message.arg(crate::osc::value::OSCValue::$variant($v.$field));
            $($($rest)*)?
        )
    };
    // capture normal field
    (@recursive $v:ident;$message:expr; $field:ident $(, $($rest:tt)*)?) => {
        $crate::to_osc_message!(
            @recursive $v;$message.arg($v.$field); $($($rest)*)?
        )
    };
    // no args
    (@recursive $v:ident;$message:expr;) => {
        $message
    };
}
