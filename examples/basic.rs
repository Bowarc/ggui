struct MainState {
    ui: ggui::Ui,
}

fn add_every_anchor(ui: &mut ggui::Ui) {
    // Anchored at the centerX centerY, but the offset reset the centerX to 0 and sets the X to the MouseX magic value
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(
            ggui::element::Anchor::CenterCenter,
            // btw this shows that the order of operation is perfectly computed
            Some(ggez::mint::Point2::from([
                (0. - ggui::value::MagicValue::ScreenSizeW / 2.
                    + ggui::value::MagicValue::MousePosX),
                ggui::Value::from(0.),
            ])),
        ),
        (100., 100.),
    ));
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(ggui::element::Anchor::Topleft, None),
        (
            ggui::value::MagicValue::ScreenSizeW * 0.1,
            ggui::value::MagicValue::ScreenSizeH * 0.1,
        ),
    ));
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(ggui::element::Anchor::TopCenter, None),
        (
            ggui::value::MagicValue::ScreenSizeW * 0.1,
            ggui::value::MagicValue::ScreenSizeH * 0.1,
        ),
    ));
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(ggui::element::Anchor::TopRight, None),
        (
            ggui::value::MagicValue::ScreenSizeW * 0.1,
            ggui::value::MagicValue::ScreenSizeH * 0.1,
        ),
    ));
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(ggui::element::Anchor::RightCenter, None),
        (
            ggui::value::MagicValue::ScreenSizeW * 0.1,
            ggui::value::MagicValue::ScreenSizeH * 0.1,
        ),
    ));
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(ggui::element::Anchor::BotRight, None),
        (
            ggui::value::MagicValue::ScreenSizeW * 0.1,
            ggui::value::MagicValue::ScreenSizeH * 0.1,
        ),
    ));
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(ggui::element::Anchor::BotCenter, None),
        (
            ggui::value::MagicValue::ScreenSizeW * 0.1,
            ggui::value::MagicValue::ScreenSizeH * 0.1,
        ),
    ));
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(ggui::element::Anchor::BotLeft, None),
        (
            ggui::value::MagicValue::ScreenSizeW * 0.1,
            ggui::value::MagicValue::ScreenSizeH * 0.1,
        ),
    ));

    // This one is kept extremely verbose to show what abstraction i implemented
    ui.add_element(ggui::element::Element::new(
        ggui::element::ElementType::new_button(),
        ggui::element::ElementPosition::new_anchor(ggui::element::Anchor::LeftCenter, None),
        (
            ggui::Value::multiple(
                ggui::Value::magic(ggui::value::MagicValue::ScreenSizeW),
                ggui::value::ValueOperation::Mul,
                ggui::Value::fixed(0.1),
            ),
            ggui::Value::multiple(
                ggui::Value::magic(ggui::value::MagicValue::ScreenSizeH),
                ggui::value::ValueOperation::Mul,
                ggui::Value::fixed(0.1),
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
    fn update(&mut self, _ctx: &mut ggez::context::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::context::Context) -> ggez::GameResult {
        ggez::graphics::Canvas::from_frame(ctx, ggez::graphics::Color::BLACK).finish(ctx)?;

        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, None);

        let t = std::time::Instant::now();
        self.ui.draw(ctx, &mut canvas)?;
        println!("Ui draw time {}", t.elapsed().as_secs_f64());

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
        _origin: ggez::event::ErrorOrigin,
        _e: ggez::GameError,
    ) -> bool {
        true
    }
}

fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("Name", "Author").window_mode(ggez::conf::WindowMode {
        resizable: true,
        ..Default::default()
    });
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    ggez::event::run(ctx, event_loop, state)
}
