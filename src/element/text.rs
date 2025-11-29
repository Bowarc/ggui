/*
    Maybe an optimisation could be to compute the rows only once ?

    can't it change size and pos over time (magic values?)
*/

pub struct Text {
    id: crate::Id,
    position: crate::Position,
    req_size: crate::Value,
    real_size: crate::Vector,
    style: crate::Style,
    bits: Vec<TextBit>,
}

#[derive(Clone, Debug)]
pub enum TextBit {
    Text {
        raw: String,
        color_opt: Option<crate::render::Color>,
    },
    Image {
        image: ggez::graphics::Image,
        color_opt: Option<crate::render::Color>,
    },
    NewLine,
}

#[derive(Clone)]
enum ComputedTextBit {
    Text(ggez::graphics::Text),
    Image(ggez::graphics::Image, Option<crate::render::Color>),
}

fn compute_text_bits(bits: Vec<TextBit>) -> Vec<TextBit> {
    if bits.is_empty() {
        return bits;
    }
    let mut new_bits = Vec::new();

    for bit in bits {
        match &bit {
            TextBit::Text { raw, color_opt } => {
                if raw.contains('\n') {
                    let raws = raw.split('\n').collect::<Vec<&str>>();
                    for (i, splitted) in raws.iter().enumerate() {
                        new_bits.push(TextBit::Text {
                            raw: splitted.to_string(),
                            color_opt: *color_opt,
                        });
                        if i < raws.len() - 1 {
                            new_bits.push(TextBit::NewLine);
                        }
                    }
                } else {
                    new_bits.push(bit)
                }
            }
            _ => new_bits.push(bit),
        }
    }

    // Remove all empty strings
    let mut i = 0;
    while i < new_bits.len() {
        let mut remove = false;
        if let TextBit::Text { raw, .. } = new_bits.get(i).unwrap() {
            if raw.is_empty() {
                remove = true;
            }
        }

        if remove {
            new_bits.remove(i);
        } else {
            i += 1;
        }
    }

    // Do we pop if the last bit is a new line ?
    /*unsure */
    // NO THIS FUCKS UP THE last text rendering
    // {
    //     while let Some(TextBit::NewLine) = new_bits.last() {
    //         new_bits.pop();
    //     }
    // }
    // i need to make sure that the last bit is a new line (it makes the multibit render)
    {
        if !matches!(new_bits.last(), Some(TextBit::NewLine)) {
            new_bits.push(TextBit::NewLine)
        }
    }
    // debug!("{new_bits:?}");

    new_bits
}

impl Text {
    pub fn new(
        id: crate::Id,
        position: crate::Position,
        req_size: crate::Value,
        style: crate::Style,
        bits: Vec<TextBit>,
    ) -> Self {
        let new_bits = compute_text_bits(bits);

        Self {
            id,
            position,
            req_size,
            real_size: crate::Vector::new(0., 0.),
            style,
            bits: new_bits,
        }
    }

    fn draw_bits_single_text(
        &mut self,
        ctx: &mut ggez::Context,
        target_size: f64,
        real_rect: &math::Rect,
        render_request: &mut crate::render::RenderRequest,
    ) {
        use ggez::graphics::Drawable as _;

        // This is called under the assumption that there is no image in the bits !

        let mut global_text = ggez::graphics::Text::new("");
        global_text.set_layout(ggez::graphics::TextLayout::center());
        for bit in self.bits.iter() {
            match bit {
                TextBit::Text { raw, color_opt } => {
                    let mut f =
                        ggez::graphics::TextFragment::new(raw.clone()).scale(target_size as f32);
                    f.color = color_opt.map(|c| c.into());
                    global_text.add(f);
                }
                TextBit::NewLine => {
                    global_text.add('\n');
                }
                TextBit::Image { .. } => {
                    unreachable!("You're not supposed to draw images in this loop")
                }
            }
        }
        let size = global_text.dimensions(ctx).unwrap().size();

        self.real_size = crate::Vector::new(
            crate::Value::fixed(size.x.into()),
            crate::Value::fixed(size.y.into()),
        );

        render_request.add(
            global_text,
            crate::render::DrawParam::default().pos(real_rect.center()),
        );
    }

    fn draw_bits_multi_text(
        &mut self,
        ctx: &mut ggez::Context,
        target_size: f64,
        real_rect: &math::Rect,
        render_request: &mut crate::render::RenderRequest,
    ) {
        use ggez::graphics::Drawable as _;
        let mut draw_curr_row =
            |curr_row: Vec<ComputedTextBit>, curr_width: f64, curr_height: f64| {
                let mut x = 0.;
                for computed_bit in curr_row {
                    match computed_bit {
                        ComputedTextBit::Text(ggtext) => {
                            let w = ggtext.dimensions(ctx).unwrap().w;
                            render_request.add(
                                ggtext,
                                crate::render::DrawParam::default().pos(
                                    real_rect.center()
                                        + math::Point::new(
                                            x - curr_width * 0.5,
                                            0. + curr_height - real_rect.height() * 0.5,
                                        ),
                                ),
                            );
                            x += w as f64;
                        }
                        ComputedTextBit::Image(image, color_opt) => {
                            render_request.add(
                                image,
                                crate::render::DrawParam::default()
                                    .pos(
                                        real_rect.center()
                                            + math::Point::new(
                                                x - curr_width * 0.5,
                                                0. + curr_height - real_rect.height() * 0.5,
                                            )
                                            + math::Vec2::new(0.5, 0.5) * target_size,
                                    )
                                    .color(color_opt.unwrap_or(crate::render::Color::WHITE))
                                    .size(target_size),
                            );
                            x += target_size;
                        }
                    }
                }
            };

        let mut total_size = math::Vec2::ZERO;

        let mut curr_row = Vec::new();
        let mut curr_height = 0.;
        let mut curr_text: Option<ggez::graphics::Text> = None;
        for (i, bit) in self.bits.iter().enumerate() {
            let mut need_draw = false;
            match bit {
                TextBit::Text { raw, color_opt } => {
                    let mut f = ggez::graphics::TextFragment::new(raw).scale(target_size as f32);
                    f.color = color_opt.map(|c| c.into());

                    if let Some(text) = &mut curr_text {
                        text.add(f);
                    } else {
                        let text = ggez::graphics::Text::new(f);
                        curr_text = Some(text);
                    };
                }
                TextBit::Image {
                    image,
                    color_opt,
                } => {
                    if let Some(text) = curr_text {
                        curr_row.push(ComputedTextBit::Text(text));
                        curr_text = None;
                    }
                    curr_row.push(ComputedTextBit::Image(image.clone(), *color_opt));
                }
                TextBit::NewLine => {
                    if let Some(text) = curr_text {
                        curr_row.push(ComputedTextBit::Text(text));
                        curr_text = None;
                    }
                    need_draw = true;
                }
            }

            if need_draw || i == self.bits.len() - 1 {
                let line_width: f64 = curr_row
                    .iter()
                    .map(|elem| match elem {
                        ComputedTextBit::Text(t) => t.dimensions(ctx).unwrap().w as f64,
                        ComputedTextBit::Image(_, _) => target_size,
                    })
                    .sum();
                draw_curr_row(curr_row, line_width, curr_height);
                curr_row = Vec::new();

                if line_width > total_size.x {
                    total_size.x = line_width;
                }
                curr_height += target_size;
            }
        }
        total_size.y = curr_height;

        self.real_size = crate::Vector::new(
            crate::Value::fixed(total_size.x),
            crate::Value::fixed(total_size.y),
        );
    }
    pub fn replace_bits(&mut self, new_bits: Vec<TextBit>) {
        self.bits = compute_text_bits(new_bits)
    }
}

impl super::TElement for Text {
    fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        back_mesh: &mut ggez::graphics::MeshBuilder,
        _ui_mesh: &mut ggez::graphics::MeshBuilder,
        front_mesh: &mut ggez::graphics::MeshBuilder,
        render_request: &mut crate::render::RenderRequest,
    ) -> ggez::GameResult {
        let real_rect = self.get_computed_rect(ctx);
        let target_size = self.req_size.compute(ctx);
        // draw background
        if let Some(bg) = self.style.get_bg() {
            bg.draw(back_mesh, render_request, real_rect)?
        }

        // draw border
        if let Some(border) = self.style.get_border() {
            border.draw(front_mesh, real_rect)?;
        };

        let image_count: i32 = self
            .bits
            .iter()
            .map(|bit| {
                if matches!(bit, TextBit::Image { .. }) {
                    1
                } else {
                    0
                }
            })
            .sum();

        if image_count > 0 {
            self.draw_bits_multi_text(ctx, target_size, &real_rect, render_request);
        } else {
            self.draw_bits_single_text(ctx, target_size, &real_rect, render_request);
        }

        Ok(())
    }
    fn get_size_value(&self) -> &crate::Vector {
        &self.real_size
    }
    fn get_pos_value(&self) -> &crate::Position {
        &self.position
    }
    fn get_id(&self) -> crate::Id {
        self.id.clone()
    }
}

impl TextBit {
    // pub fn new_text(raw: impl Into<String>, color_opt: Option<crate::render::Color>) -> Self {
    //     Self::Text {
    //         raw: raw.into(),
    //         color_opt,
    //     }
    // }
    // pub fn new_img(
    //     image: crate::assets::sprite::SpriteId,
    //     color_opt: Option<crate::render::Color>,
    // ) -> Self {
    //     Self::Image {
    //         image,
    //         color_opt,
    //     }
    // }
}

impl From<&str> for TextBit {
    fn from(value: &str) -> Self {
        TextBit::Text {
            raw: value.to_string(),
            color_opt: None,
        }
    }
}

impl From<String> for TextBit {
    fn from(value: String) -> Self {
        TextBit::Text {
            raw: value,
            color_opt: None,
        }
    }
}

impl From<(&str, crate::render::Color)> for TextBit {
    fn from(value: (&str, crate::render::Color)) -> Self {
        TextBit::Text {
            raw: value.0.to_string(),
            color_opt: Some(value.1),
        }
    }
}

impl From<(String, crate::render::Color)> for TextBit {
    fn from(value: (String, crate::render::Color)) -> Self {
        TextBit::Text {
            raw: value.0,
            color_opt: Some(value.1),
        }
    }
}

impl From<ggez::graphics::Image> for TextBit {
    fn from(value: ggez::graphics::Image) -> Self {
        TextBit::Image {
            image: value,
            color_opt: None,
        }
    }
}

impl From<(ggez::graphics::Image, crate::render::Color)> for TextBit {
    fn from(value: (ggez::graphics::Image, crate::render::Color)) -> Self {
        TextBit::Image {
            image: value.0,
            color_opt: Some(value.1),
        }
    }
}
