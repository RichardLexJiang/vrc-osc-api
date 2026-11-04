use crate::to_osc_message;

macro_rules! axis_input {
    ($(#[$meta:meta])* $name:ident, $address:literal) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $name {
            value: f32,
        }

        impl $name {
            pub fn new(value: f32) -> Self {
                Self { value }
            }
        }

        to_osc_message!($name, $address, value);
    };
}

macro_rules! button_input {
    ($doc:literal, $name:ident, $address:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name {
            value: i32,
        }

        impl $name {
            pub fn new(pressed: bool) -> Self {
                Self {
                    value: i32::from(pressed),
                }
            }
        }

        to_osc_message!($name, $address, OSCValue::Int32(value));
    };
}

axis_input!(
    /// Moves forward (1.0) or backward (-1.0); reset to 0.0 when not in use.
    InputVertical,
    "/input/Vertical"
);
axis_input!(
    /// Moves right (1.0) or left (-1.0); reset to 0.0 when not in use.
    InputHorizontal,
    "/input/Horizontal"
);
axis_input!(
    /// Controls horizontal looking; reset to 0.0 when not in use.
    InputLookHorizontal,
    "/input/LookHorizontal"
);
axis_input!(
    /// Controls use of the held right-hand item.
    InputUseAxisRight,
    "/input/UseAxisRight"
);
axis_input!(
    /// Controls grabbing with the right hand.
    InputGrabAxisRight,
    "/input/GrabAxisRight"
);
axis_input!(
    /// Moves a held object forward (1.0) or backward (-1.0).
    InputMoveHoldFB,
    "/input/MoveHoldFB"
);
axis_input!(
    /// Spins a held object clockwise or counter-clockwise.
    InputSpinHoldCwCcw,
    "/input/SpinHoldCwCcw"
);
axis_input!(
    /// Spins a held object up or down.
    InputSpinHoldUD,
    "/input/SpinHoldUD"
);
axis_input!(
    /// Spins a held object left or right.
    InputSpinHoldLR,
    "/input/SpinHoldLR"
);

button_input!(
    "Holds the forward movement input.",
    InputMoveForward,
    "/input/MoveForward"
);
button_input!(
    "Holds the backward movement input.",
    InputMoveBackward,
    "/input/MoveBackward"
);
button_input!(
    "Holds the left movement input.",
    InputMoveLeft,
    "/input/MoveLeft"
);
button_input!(
    "Holds the right movement input.",
    InputMoveRight,
    "/input/MoveRight"
);
button_input!(
    "Holds the look-left input.",
    InputLookLeft,
    "/input/LookLeft"
);
button_input!(
    "Holds the look-right input.",
    InputLookRight,
    "/input/LookRight"
);
button_input!("Jumps if supported by the world.", InputJump, "/input/Jump");
button_input!(
    "Walks faster if supported by the world.",
    InputRun,
    "/input/Run"
);
button_input!(
    "Snap-turns left in VR.",
    InputComfortLeft,
    "/input/ComfortLeft"
);
button_input!(
    "Snap-turns right in VR.",
    InputComfortRight,
    "/input/ComfortRight"
);
button_input!(
    "Drops the item held in the right hand.",
    InputDropRight,
    "/input/DropRight"
);
button_input!(
    "Uses the item highlighted by the right hand.",
    InputUseRight,
    "/input/UseRight"
);
button_input!(
    "Grabs the item highlighted by the right hand.",
    InputGrabRight,
    "/input/GrabRight"
);
button_input!(
    "Drops the item held in the left hand.",
    InputDropLeft,
    "/input/DropLeft"
);
button_input!(
    "Uses the item highlighted by the left hand.",
    InputUseLeft,
    "/input/UseLeft"
);
button_input!(
    "Grabs the item highlighted by the left hand.",
    InputGrabLeft,
    "/input/GrabLeft"
);
button_input!(
    "Turns on Safe Mode.",
    InputPanicButton,
    "/input/PanicButton"
);
button_input!(
    "Toggles the Quick Menu using the left-hand control.",
    InputQuickMenuToggleLeft,
    "/input/QuickMenuToggleLeft"
);
button_input!(
    "Toggles the Quick Menu using the right-hand control.",
    InputQuickMenuToggleRight,
    "/input/QuickMenuToggleRight"
);
button_input!(
    "Controls voice input; behavior depends on the VRChat voice setting.",
    InputVoice,
    "/input/Voice"
);

#[cfg(test)]
mod tests {
    use crate::{
        osc::{message::OscMessage, value::OSCValue},
        vrchat::input::{InputJump, InputVertical},
    };

    #[test]
    fn axis_uses_float_value() {
        assert_eq!(
            OscMessage::from(InputVertical::new(-1.0)).encode(),
            OscMessage::new("/input/Vertical")
                .arg(OSCValue::Float32(-1.0))
                .encode()
        );
    }

    #[test]
    fn button_uses_integer_press_and_release_values() {
        assert_eq!(
            OscMessage::from(InputJump::new(true)).encode(),
            OscMessage::new("/input/Jump")
                .arg(OSCValue::Int32(1))
                .encode()
        );
        assert_eq!(
            OscMessage::from(InputJump::new(false)).encode(),
            OscMessage::new("/input/Jump")
                .arg(OSCValue::Int32(0))
                .encode()
        );
    }
}
