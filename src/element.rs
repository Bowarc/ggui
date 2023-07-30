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
    Topright,
    LeftCenter,
    Botleft,
    BotCenter,
    Botright,
    RightCenter,
}

pub struct Element {
    pub t: ElementType,
    pub id: crate::Id,
    pub position: ElementPosition,
    pub size: crate::Point,
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

                anchor_position - offset
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
            Anchor::Topright => crate::Point::new(drawable_size.x - element_size.x, 0.),
            Anchor::LeftCenter => crate::Point::new(
                drawable_size.x - element_size.x,
                drawable_size.y / 2. - element_size.y / 2.,
            ),
            Anchor::Botleft => crate::Point::new(
                drawable_size.x - element_size.x,
                drawable_size.y - element_size.y,
            ),
            Anchor::BotCenter => crate::Point::new(
                drawable_size.x / 2. - element_size.x / 2.,
                drawable_size.y - element_size.y,
            ),
            Anchor::Botright => crate::Point::new(0., drawable_size.y - element_size.y),
            Anchor::RightCenter => {
                crate::Point::new(0., drawable_size.y / 2. - element_size.y / 2.)
            }
        }
    }
}

impl Element {
    pub fn new(t: ElementType, position: ElementPosition, size: crate::Point) -> Self {
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
        canvas: &mut ggez::graphics::Canvas,
    ) -> ggez::GameResult {
        let position = self.position.compute(ctx, self.size);

        match &self.t {
            ElementType::Button(_btn) => {
                let mesh = ggez::graphics::Mesh::new_rectangle(
                    ctx,
                    ggez::graphics::DrawMode::fill(),
                    ggez::graphics::Rect::new(
                        position.x as f32,
                        position.y as f32,
                        self.size.x as f32,
                        self.size.y as f32,
                    ),
                    ggez::graphics::Color::from_rgb(0, 175, 150),
                )?;

                canvas.draw(&mesh, ggez::graphics::DrawParam::default())
            }
        }

        Ok(())
    }
}
