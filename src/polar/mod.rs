use crate::rect::ZRect;

#[derive(Debug, Clone)]
pub struct ZPolar (pub f64, pub f64);

impl ZPolar {
    pub fn new(x: f64, y: f64) -> Self {
        ZPolar(x, y)
    }
    pub fn to_rect(&self) -> ZRect {
        ZRect(
            self.0 * self.1.cos(),
            self.0 * self.1.sin()
        )
    }
}

impl From<ZPolar> for (f64, f64) {
    fn from(x: ZPolar) -> Self {
        (x.0, x.1)
    }
}

