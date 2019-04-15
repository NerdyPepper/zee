use crate::polar::ZPolar;

#[derive(Debug, Clone)]
pub struct ZRect(pub f64, pub f64);

impl ZRect {
    pub fn new(x: f64, y: f64) -> Self {
        ZRect(x, y)
    }
    pub fn to_polar(&self) -> ZPolar {
        let angle = {
            if self.0 > 0. && self.1 > 0. {
                (self.1/self.0).atan().to_degrees()
            } else if self.0 <= 0. && self.1 > 0. {
                180. - (self.1/-self.0).atan().to_degrees()
            } else if self.0 < 0. && self.1 < 0. {
                -180. + (self.1/self.0).atan().to_degrees()
            } else {
                - (-self.1/self.0).atan().to_degrees()
            }
        };
        ZPolar(
            (self.0*self.0 + self.1*self.1).sqrt(),
            angle
        )
    }
}

impl From<ZRect> for (f64, f64) {
    fn from(x: ZRect) -> Self {
        (x.0, x.1)
    }
}

