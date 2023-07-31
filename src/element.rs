mod button;

pub enum ElementType {
    Button(button::Button),
}

pub enum ElementPosition {
    Value(ggez::mint::Point2<crate::Value>),
    Anchor {
        anchor: Anchor,
        offset: ggez::mint::Point2<crate::Value>,
    },
}

pub enum Anchor {
    CenterCenter,
    Topleft,
    TopCenter,
    TopRight,
    RightCenter,
    BotRight,
    BotCenter,
    BotLeft,
    LeftCenter,
}

pub struct Element {
    pub t: ElementType,
    pub id: crate::Id,
    pub position: ElementPosition,
    pub size: ggez::mint::Point2<crate::Value>,
}

impl ElementType {
    pub fn new_button() -> Self {
        Self::Button(button::Button {})
    }
}

impl ElementPosition {
    pub fn new_value(value: impl Into<ggez::mint::Point2<crate::Value>>) -> Self {
        Self::Value(value.into())
    }
    pub fn new_anchor(
        anchor: Anchor,
        offset: impl Into<Option<ggez::mint::Point2<crate::Value>>>,
    ) -> Self {
        let offset = offset.into().unwrap_or_else(|| {
            ggez::mint::Point2::from([crate::Value::Fixed(0.), crate::Value::Fixed(0.)])
        });
        Self::Anchor { anchor, offset }
    }

    pub fn compute(&self, ctx: &mut ggez::Context, element_size: crate::Point) -> crate::Point {
        match self {
            ElementPosition::Value(pt) => crate::Point::new(pt.x.compute(ctx), pt.y.compute(ctx)),
            ElementPosition::Anchor { anchor, offset } => {
                let offset = crate::Point::new(offset.x.compute(ctx), offset.y.compute(ctx));
                let drawable_size: crate::Point = ctx.gfx.drawable_size().into();

                let anchor_position = anchor.compute(drawable_size, element_size);

                anchor_position + offset
            }
        }
    }
}

impl Anchor {
    /// Returns the topleft point of the element
    pub fn compute(&self, drawable_size: crate::Point, element_size: crate::Point) -> crate::Point {
        match self {
            Anchor::CenterCenter => {
                crate::Point::new(drawable_size.x / 2., drawable_size.y / 2.) - element_size / 2.
            }
            Anchor::Topleft => crate::Point::ZERO,
            Anchor::TopCenter => crate::Point::new(drawable_size.x / 2. - element_size.x / 2., 0.),
            Anchor::TopRight => crate::Point::new(drawable_size.x - element_size.x, 0.),
            Anchor::RightCenter => crate::Point::new(
                drawable_size.x - element_size.x,
                drawable_size.y / 2. - element_size.y / 2.,
            ),
            Anchor::BotRight => crate::Point::new(
                drawable_size.x - element_size.x,
                drawable_size.y - element_size.y,
            ),
            Anchor::BotCenter => crate::Point::new(
                drawable_size.x / 2. - element_size.x / 2.,
                drawable_size.y - element_size.y,
            ),
            Anchor::BotLeft => crate::Point::new(0., drawable_size.y - element_size.y),
            Anchor::LeftCenter => crate::Point::new(0., drawable_size.y / 2. - element_size.y / 2.),
        }
    }
}

impl Element {
    pub fn new(
        t: ElementType,
        position: ElementPosition,
        size: (impl Into<crate::Value>, impl Into<crate::Value>),
    ) -> Self {
        let x = size.0.into();
        let y = size.1.into();
        let size = ggez::mint::Point2::from([x, y]);
        Element {
            t,
            id: String::from("This is a test"),
            position,
            size,
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        _canvas: &mut ggez::graphics::Canvas,
        global_mesh: &mut ggez::graphics::MeshBuilder,
    ) -> ggez::GameResult {
        let size = crate::Point::new(self.size.x.compute(ctx), self.size.y.compute(ctx));

        let position = self.position.compute(ctx, size);

        match &self.t {
            ElementType::Button(_btn) => {
                global_mesh.rectangle(
                    ggez::graphics::DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        position.x as f32,
                        position.y as f32,
                        size.x as f32,
                        size.y as f32,
                    ),
                    ggez::graphics::Color::from_rgb(0, 175, 150),
                )?;
            }
        }

        Ok(())
    }
}
