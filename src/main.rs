use std::process::Command;
use std::convert::{ From, Into };
use std::f64;

use plotlib::view::ContinuousView;
use plotlib::page::Page;
use plotlib::line::{ Line, Style };
use plotlib::style::Line as linestyle;

mod rect;
use crate::rect::ZRect;

pub enum Reactance {
    I(f64),
    C(f64)
}

mod polar;
use crate::polar::ZPolar;

fn main() {
    let x_axis = Line::new(&[(-100., 0.), (100., 0.)])
        .style(
            &Style::new().colour("grey")
        );
    let y_axis = Line::new(&[(0., 100.), (0., -100.)])
        .style(
            &Style::new().colour("grey")
        );
    let z_resultant = Line::new(&[(0., 0.), z1_tuple]).style(
        &Style::new().colour("red")
    );
    let resistance = Line::new(&[(0., 0.), (z1.0, 0.)]).style(
        &Style::new().colour("red")
    );
    let reactance = Line::new(&[(0., 0.), (0., z1.1)]).style(
        &Style::new().colour("red")
    );


    let v = ContinuousView::new()
        .add(&x_axis)
        .add(&y_axis)
        .add(&z_resultant)
        .add(&resistance)
        .add(&reactance)
        .x_range(-10., 10.)
        .y_range(-10., 10.);
    Page::single(&v).save("line.svg").unwrap();

    Command::new("firefox")
        .arg("-new-window")
        .arg("./line.svg")
        .output()
        .unwrap();
}
