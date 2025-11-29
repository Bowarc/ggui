#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Color([u8; 4]); // 0 - 255

impl Color {
    pub const WHITE: Self = Self([255; 4]);
    pub const BLACK: Self = Self([0, 0, 0, 255]);
    pub const TRANSPARENT: Self = Self([0; 4]);
    pub fn from_rgba(
        r: impl Into<u8>,
        g: impl Into<u8>,
        b: impl Into<u8>,
        a: impl Into<u8>,
    ) -> Self {
        Self([r.into(), g.into(), b.into(), a.into()])
    }

    pub fn from_rgb(r: impl Into<u8>, g: impl Into<u8>, b: impl Into<u8>) -> Self {
        Self([r.into(), g.into(), b.into(), 255])
    }
    #[allow(clippy::get_first)]
    pub fn from_hex(mut hex: &str) -> Option<Self> {
        if hex.starts_with("#") {
            hex = &hex[1..];
        }

        u32::from_str_radix(hex, 16)
            .ok()
            .map(u32::to_be_bytes)
            .map(Self)
            .or_else(|| {
                warn!("Could not convert hex: #{hex} to rgb, falling back to default color");
                None
            })
    }

    // pub fn random_rgb() -> Self {
    //     Self(
    //         random::get_inc(0u8, 255u8),
    //         random::get_inc(0u8, 255u8),
    //         random::get_inc(0u8, 255u8),
    //         255,
    //     )
    // }
    // pub fn random_rgba() -> Self {
    //     Self(
    //         random::get_inc(0u8, 255u8),
    //         random::get_inc(0u8, 255u8),
    //         random::get_inc(0u8, 255u8),
    //         random::get_inc(0u8, 255u8),
    //     )
    // }

    pub fn rgba(&self) -> [u8; 4] {
        [self.red(), self.green(), self.blue(), self.alpha()]
    }
    pub fn red(&self) -> u8 {
        self.0[0]
    }
    pub fn green(&self) -> u8 {
        self.0[1]
    }
    pub fn blue(&self) -> u8 {
        self.0[2]
    }
    pub fn alpha(&self) -> u8 {
        self.0[3]
    }
    pub fn r(&self) -> u8 {
        self.red()
    }
    pub fn g(&self) -> u8 {
        self.green()
    }
    pub fn b(&self) -> u8 {
        self.blue()
    }
    pub fn a(&self) -> u8 {
        self.alpha()
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl From<ggez::graphics::Color> for Color {
    fn from(ggezcolor: ggez::graphics::Color) -> Color {
        let (r, g, b, a) = ggezcolor.to_rgba();

        Color([r, g, b, a])
    }
}

impl From<Color> for ggez::graphics::Color {
    fn from(color: Color) -> ggez::graphics::Color {
        ggez::graphics::Color::from_rgba(color.r(), color.g(), color.b(), color.a())
    }
}

impl From<[u8; 4]> for Color {
    fn from(u8array: [u8; 4]) -> Color {
        Color(u8array)
    }
}
