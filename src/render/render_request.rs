#[derive(Default)]
pub struct RenderRequest { // Can't find a good name for that
    inner: Vec<(RenderRequestBit, super::DrawParam)>,
}

#[derive(Clone, Debug)]
pub enum RenderRequestBit {
    Image(ggez::graphics::Image),
    Mesh(ggez::graphics::Mesh),
    MeshBuilder(ggez::graphics::MeshBuilder),
    Text(ggez::graphics::Text),
}

impl RenderRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<T: Into<RenderRequestBit>>(&mut self, render_request_bit: T, dp: super::DrawParam) {
        self.inner.push((render_request_bit.into(), dp))
    }

    pub fn add_batch<T: Into<RenderRequestBit>>(&mut self, batch: Vec<(T, super::DrawParam)>) {
        for (render_request_bit, dp) in batch {
            self.add(render_request_bit, dp)
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }
}

// Not used anymore but was good so im keeping it
// fn get_order_value_from_drawable(
//     d: &impl ggez::graphics::Drawable,
//     ctx: &mut ggez::Context,
// ) -> i32 {
//     match d.dimensions(ctx) {
//         Some(d) => (d.y + (d.h * 0.5)) as i32,
//         None => {
//             error!("Could not get dimensions of Drawable");
//             0
//         }
//     }
// }

impl std::ops::Deref for RenderRequest {
    type Target = Vec<(RenderRequestBit, super::DrawParam)>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RenderRequest {
    // type Target = HashMap<Layer, Vec<RenderRequestBit>>;

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<ggez::graphics::Image> for RenderRequestBit {
    fn from(image: ggez::graphics::Image) -> RenderRequestBit {
        RenderRequestBit::Image(image)
    }
}

impl From<ggez::graphics::Mesh> for RenderRequestBit {
    fn from(mesh: ggez::graphics::Mesh) -> RenderRequestBit {
        RenderRequestBit::Mesh(mesh)
    }
}

impl From<ggez::graphics::MeshBuilder> for RenderRequestBit {
    fn from(mesh: ggez::graphics::MeshBuilder) -> RenderRequestBit {
        RenderRequestBit::MeshBuilder(mesh)
    }
}

impl From<ggez::graphics::Text> for RenderRequestBit {
    fn from(text: ggez::graphics::Text) -> RenderRequestBit {
        RenderRequestBit::Text(text)
    }
}
