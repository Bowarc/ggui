struct MainState {
    ui: ggui::Ui,
}

fn add_every_anchor(ui: &mut ggui::Ui) {
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(ggui::element::Anchor::CenterCenter, None),
        (100., 100.),
    ));
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(ggui::element::Anchor::Topleft, None),
        (
            (
                ggui::value::Value::from(ggui::value::MagicValue::ScreenSizeW),
                ggui::value::ValueOperation::Mul,
                ggui::value::Value::from(0.1),
            ),
            (
                ggui::value::Value::from(ggui::value::MagicValue::ScreenSizeH),
                ggui::value::ValueOperation::Mul,
                ggui::value::Value::from(0.1),
            ),
        ),
    ));
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(ggui::element::Anchor::TopCenter, None),
        (
            (
                ggui::value::Value::from(ggui::value::MagicValue::ScreenSizeW),
                ggui::value::ValueOperation::Mul,
                ggui::value::Value::from(0.1),
            ),
            (
                ggui::value::Value::from(ggui::value::MagicValue::ScreenSizeH),
                ggui::value::ValueOperation::Mul,
                ggui::value::Value::from(0.1),
            ),
        ),
    ));
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(ggui::element::Anchor::TopRight, None),
        (
            (
                ggui::value::Value::from(ggui::value::MagicValue::ScreenSizeW),
                ggui::value::ValueOperation::Mul,
                ggui::value::Value::from(0.1),
            ),
            (
                ggui::value::Value::from(ggui::value::MagicValue::ScreenSizeH),
                ggui::value::ValueOperation::Mul,
                ggui::value::Value::from(0.1),
            ),
        ),
    ));
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(ggui::element::Anchor::RightCenter, None),
        (
            (
                ggui::value::Value::from(ggui::value::MagicValue::ScreenSizeW),
                ggui::value::ValueOperation::Mul,
                ggui::value::Value::from(0.1),
            ),
            (
                ggui::value::Value::from(ggui::value::MagicValue::ScreenSizeH),
                ggui::value::ValueOperation::Mul,
                ggui::value::Value::from(0.1),
            ),
        ),
    ));
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(ggui::element::Anchor::BotRight, None),
        (
            (
                ggui::value::Value::from(ggui::value::MagicValue::ScreenSizeW),
                ggui::value::ValueOperation::Mul,
                ggui::value::Value::from(0.1),
            ),
            (
                ggui::value::Value::from(ggui::value::MagicValue::ScreenSizeH),
                ggui::value::ValueOperation::Mul,
                ggui::value::Value::from(0.1),
            ),
        ),
    ));
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(ggui::element::Anchor::BotCenter, None),
        (
            (
                ggui::value::Value::from(ggui::value::MagicValue::ScreenSizeW),
                ggui::value::ValueOperation::Mul,
                ggui::value::Value::from(0.1),
            ),
            (
                ggui::value::Value::from(ggui::value::MagicValue::ScreenSizeH),
                ggui::value::ValueOperation::Mul,
                ggui::value::Value::from(0.1),
            ),
        ),
    ));
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(ggui::element::Anchor::BotLeft, None),
        (
            (
                ggui::value::Value::from(ggui::value::MagicValue::ScreenSizeW),
                ggui::value::ValueOperation::Mul,
                ggui::value::Value::from(0.1),
            ),
            (
                ggui::value::Value::from(ggui::value::MagicValue::ScreenSizeH),
                ggui::value::ValueOperation::Mul,
                ggui::value::Value::from(0.1),
            ),
        ),
    ));
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(ggui::element::Anchor::LeftCenter, None),
        (
            (
                ggui::value::Value::from(ggui::value::MagicValue::ScreenSizeW),
                ggui::value::ValueOperation::Mul,
                ggui::value::Value::from(0.1),
            ),
            (
                ggui::value::Value::from(ggui::value::MagicValue::ScreenSizeH),
                ggui::value::ValueOperation::Mul,
                ggui::value::Value::from(0.1),
            ),
        ),
    ));
}

impl MainState {
    pub fn new(_ctx: &mut ggez::Context) -> ggez::GameResult<Self> {
        let mut ui = ggui::Ui::new();

        add_every_anchor(&mut ui);
        Ok(MainState { ui })
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::context::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::context::Context) -> ggez::GameResult {
        ggez::graphics::Canvas::from_frame(ctx, ggez::graphics::Color::BLACK).finish(ctx)?;

        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, None);

        self.ui.draw(ctx, &mut canvas)?;

        canvas.finish(ctx)?;

        Ok(())
    }
    // Provided methods
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut ggez::context::Context,
        _button: ggez::input::mouse::MouseButton,
        _x: f32,
        _y: f32,
    ) -> ggez::GameResult {
        Ok(())
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut ggez::context::Context,
        _button: ggez::input::mouse::MouseButton,
        _x: f32,
        _y: f32,
    ) -> ggez::GameResult {
        Ok(())
    }
    fn mouse_motion_event(
        &mut self,
        _ctx: &mut ggez::context::Context,
        _x: f32,
        _y: f32,
        _dx: f32,
        _dy: f32,
    ) -> ggez::GameResult {
        Ok(())
    }
    fn mouse_enter_or_leave(
        &mut self,
        _ctx: &mut ggez::context::Context,
        _entered: bool,
    ) -> ggez::GameResult {
        Ok(())
    }
    fn mouse_wheel_event(
        &mut self,
        _ctx: &mut ggez::context::Context,
        _x: f32,
        _y: f32,
    ) -> ggez::GameResult {
        Ok(())
    }
    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::context::Context,
        _input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> ggez::GameResult {
        Ok(())
    }
    fn key_up_event(
        &mut self,
        _ctx: &mut ggez::context::Context,
        _input: ggez::input::keyboard::KeyInput,
    ) -> ggez::GameResult {
        Ok(())
    }
    fn text_input_event(
        &mut self,
        _ctx: &mut ggez::context::Context,
        _character: char,
    ) -> ggez::GameResult {
        Ok(())
    }
    fn touch_event(
        &mut self,
        _ctx: &mut ggez::context::Context,
        _phase: ggez::event::winit_event::TouchPhase,
        _x: f64,
        _y: f64,
    ) -> ggez::GameResult {
        Ok(())
    }
    fn gamepad_button_down_event(
        &mut self,
        _ctx: &mut ggez::context::Context,
        _btn: ggez::event::Button,
        _id: ggez::event::GamepadId,
    ) -> ggez::GameResult {
        Ok(())
    }
    fn gamepad_button_up_event(
        &mut self,
        _ctx: &mut ggez::context::Context,
        _btn: ggez::event::Button,
        _id: ggez::event::GamepadId,
    ) -> ggez::GameResult {
        Ok(())
    }
    fn gamepad_axis_event(
        &mut self,
        _ctx: &mut ggez::context::Context,
        _axis: ggez::event::Axis,
        _value: f32,
        _id: ggez::event::GamepadId,
    ) -> ggez::GameResult {
        Ok(())
    }
    fn focus_event(
        &mut self,
        _ctx: &mut ggez::context::Context,
        _gained: bool,
    ) -> ggez::GameResult {
        Ok(())
    }
    fn quit_event(&mut self, _ctx: &mut ggez::context::Context) -> ggez::GameResult<bool> {
        Ok(false)
    }
    fn resize_event(
        &mut self,
        _ctx: &mut ggez::context::Context,
        _width: f32,
        _height: f32,
    ) -> ggez::GameResult {
        Ok(())
    }
    fn on_error(
        &mut self,
        _ctx: &mut ggez::context::Context,
        origin: ggez::event::ErrorOrigin,
        e: ggez::GameError,
    ) -> bool {
        true
    }
}

fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("Name", "Author").window_mode(ggez::conf::WindowMode {
        width: 800.,
        height: 600.,
        maximized: false,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 1.0,
        max_width: 0.0,
        min_height: 1.0,
        max_height: 0.0,
        resizable: true,
        visible: true,
        transparent: false,
        resize_on_scale_factor_change: false,
        logical_size: None,
    });
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    ggez::event::run(ctx, event_loop, state)
}
