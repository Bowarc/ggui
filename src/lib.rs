pub mod element;
pub mod event;
pub mod point;
pub mod time;
pub mod value;

pub use point::Point;
pub use value::Value;

pub type Id = String;

pub struct Ui {
    elements: Vec<element::Element>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, elem: element::Element) {
        self.elements.push(elem)
    }

    pub fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
    ) -> ggez::GameResult {
        let mut global_mesh = ggez::graphics::MeshBuilder::new();
        for elem in self.elements.iter_mut() {
            elem.draw(ctx, canvas, &mut global_mesh)?
        }

        canvas.draw(
            &ggez::graphics::Mesh::from_data(ctx, global_mesh.build()),
            ggez::graphics::DrawParam::new(),
        );
        Ok(())
    }
}
