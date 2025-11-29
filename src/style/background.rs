#[derive(Debug, Clone)]
pub struct Background {
    color: crate::render::Color,
    img: Option<ggez::graphics::Image>,
}

impl Background {
    pub fn new(color: crate::render::Color, img: Option<ggez::graphics::Image>) -> Self {
        Self { color, img }
    }

    pub fn get_color_mut(&mut self) -> &mut crate::render::Color {
        &mut self.color
    }

    pub fn get_color(&self) -> &crate::render::Color {
        &self.color
    }

    pub fn get_img_mut(&mut self) -> Option<&mut ggez::graphics::Image> {
        self.img.as_mut()
    }

    pub fn get_img(&self) -> Option<&ggez::graphics::Image> {
        self.img.as_ref()
    }

    pub fn draw(
        &self,
        mesh: &mut ggez::graphics::MeshBuilder,
        render_request: &mut crate::render::RenderRequest,
        element_rect: math::Rect,
    ) -> ggez::GameResult {
        if let Some(sprite_id) = self.get_img() {
            render_request.add(
                sprite_id.clone(),
                crate::render::DrawParam::default()
                    .pos(element_rect.center())
                    .size(element_rect.size())
                    .color(*self.get_color()),
            )
        } else {
            mesh.rectangle(
                ggez::graphics::DrawMode::fill(),
                element_rect.into(),
                (*self.get_color()).into(),
            )?;
        }

        Ok(())
    }
}

impl Default for Background {
    fn default() -> Self {
        Background {
            color: crate::render::Color::from_rgb(24, 24, 24),
            img: None,
        }
    }
}
