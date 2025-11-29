#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderLog {
    images: i32,
    meshes: i32,
    texts: i32,
    draw_calls: i32,
}
impl RenderLog {
    pub fn new() -> Self {
        Self {
            images: 0,
            meshes: 0,
            texts: 0,
            draw_calls: 0,
        }
    }
    pub fn on_draw_call(&mut self) {
        self.draw_calls += 1;
    }
    pub fn on_image(&mut self) {
        self.images += 1;
    }
    pub fn on_mesh(&mut self) {
        self.meshes += 1;
    }
    pub fn on_text(&mut self) {
        self.texts += 1;
    }
    pub fn elements(&self) -> i32 {
        self.images + self.meshes + self.texts
    }
    pub fn images(&self) -> i32 {
        self.images
    }
    pub fn meshes(&self) -> i32 {
        self.meshes
    }
    pub fn texts(&self) -> i32 {
        self.texts
    }
    pub fn draw_calls(&self) -> i32 {
        self.draw_calls
    }
}

impl std::ops::Add<RenderLog> for RenderLog {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            images: self.images + other.images,
            meshes: self.meshes + other.meshes,
            texts: self.texts + other.texts,
            draw_calls: self.draw_calls + other.draw_calls,
        }
    }
}

impl std::ops::Sub<RenderLog> for RenderLog {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            images: self.images - other.images,
            meshes: self.meshes - other.meshes,
            texts: self.texts - other.texts,
            draw_calls: self.draw_calls - other.draw_calls,
        }
    }
}

impl std::ops::AddAssign for RenderLog {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl std::ops::SubAssign for RenderLog {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
