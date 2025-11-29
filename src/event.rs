#[derive(Copy, Clone, Debug)]
pub enum Event {
    MousePress {
        button: ggez::event::MouseButton,
        position: math::Point,
    },
    MouseRelease {
        button: ggez::event::MouseButton,
        position: math::Point,
    },
    MouseMotion {
        position: math::Point,
        delta: math::Vec2,
    },
    MouseWheel {
        delta: math::Point,
    },
    KeyDown {
        key: ggez::input::keyboard::KeyInput,
        repeated: bool,
    },
    KeyUp {
        key: ggez::input::keyboard::KeyInput,
    },
    TextInput {
        character: char,
    },
}
