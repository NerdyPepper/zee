use std::process::Command;
use std::convert::{ From, Into };

use plotlib::view::ContinuousView;
use plotlib::view::View;
use plotlib::page::Page;
use plotlib::line::{ Line, Style };
use plotlib::style::Line as linestyle;

#[derive(Debug)]
struct ZRect(f64,f64);

#[derive(Debug)]
struct ZPolar(f64,f64);

impl ZRect {
    fn new(x: f64, y: f64) -> Self {
        ZRect(x, y)
    }
    fn to_polar(&self) -> ZPolar {
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

fn main() {
    let z1 = ZRect::new(3., 4.);
    let z1_polar = z1.to_polar();
    let z1_tuple = z1.into();
    let z1_line = Line::new(&[(0., 0.), z1_tuple]).style(
        &Style::new().colour("red")
    );
    let v = ContinuousView::new()
        .add(&z1_line)
        .x_range(-10., 10.)
        .y_range(-10., 10.);
    Page::single(&v).save("line.svg").unwrap();

    Command::new("firefox")
        .arg("-new-window")
        .arg("./line.svg")
        .output()
        .unwrap();
}
