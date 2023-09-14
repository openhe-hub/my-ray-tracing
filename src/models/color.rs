pub struct Color {
    r: u32,
    g: u32,
    b: u32,
}

impl Color {
    pub fn new(r: u32, g: u32, b: u32) -> Color {
        Color { r, g, b }
    }

    pub fn empty() -> Color {
        Color { r: 0, g: 0, b: 0 }
    }

    pub fn r(&self) -> u32 {
        self.r
    }
    pub fn g(&self) -> u32 {
        self.g
    }
    pub fn b(&self) -> u32 {
        self.b
    }
}
