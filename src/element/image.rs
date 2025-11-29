pub struct Image {
    id: crate::Id,
    position: crate::Position,
    size: crate::Vector,
    style: crate::Style,
    image: ggez::graphics::Image,
}

impl Image {
    pub fn new(
        id: crate::Id,
        position: crate::Position,
        size: crate::Vector,
        style: crate::Style,
        image: ggez::graphics::Image,
    ) -> Self {
        Self {
            id,
            position,
            size,
            style,
            image,
        }
    }
}

impl super::TElement for Image {
    fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        back_mesh: &mut ggez::graphics::MeshBuilder,
        _ui_mesh: &mut ggez::graphics::MeshBuilder,
        front_mesh: &mut ggez::graphics::MeshBuilder,
        render_request: &mut crate::render::RenderRequest,
    ) -> ggez::GameResult {
        let rect = self.get_computed_rect(ctx);

        // draw background
        if let Some(bg) = self.style.get_bg() {
            bg.draw(back_mesh, render_request, rect)?
        }

        // draw border
        if let Some(border) = self.style.get_border() {
            border.draw(front_mesh, rect)?;
        };

        render_request.add(
            self.image.clone(),
            crate::render::DrawParam::default()
                .dest(rect.center())
                .size(rect.size())
                .color(*self.style.get_color()),
        );

        Ok(())
    }
    fn get_size_value(&self) -> &crate::Vector {
        &self.size
    }
    fn get_pos_value(&self) -> &crate::Position {
        &self.position
    }
    fn get_id(&self) -> crate::Id {
        self.id.clone()
    }
}
