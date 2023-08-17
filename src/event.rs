#[derive(Copy, Clone, Debug)]
pub enum Event {
    MousePress {
        button: ggez::input::mouse::MouseButton,
        position: crate::Point,
    },
    MouseRelease {
        button: ggez::input::mouse::MouseButton,
        position: crate::Point,
    },
    MouseWheel {
        pos: crate::Point, // not sure of what to call it
    },
    KeyDown {
        input: ggez::input::keyboard::KeyInput,
        repeated: bool,
    },
    KeyUp {
        input: ggez::input::keyboard::KeyInput,
    },
    TextInput {
        character: char,
    },
}
