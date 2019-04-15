use std::process::Command;
use std::convert::{ From, Into };
use std::f64;

use plotlib::view::ContinuousView;
use plotlib::page::Page;
use plotlib::line::{ Line, Style };
use plotlib::style::Line as linestyle;

mod rect;
use crate::rect::ZRect;

mod polar;
use crate::polar::ZPolar;

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
