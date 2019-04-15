use std::process::Command;
use std::convert::{ From, Into };
use std::f64;

use plotlib::view::ContinuousView;
use plotlib::page::Page;
use plotlib::line::{ Line, Style };
use plotlib::style::Line as linestyle;

// use cursive::Cursive;
// use cursive::views::{
//     Dialog, DummyView, EditView, LinearLayout
// };

mod rect;
use crate::rect::ZRect;

pub enum Reactance {
    I(f64),
    C(f64)
}

mod polar;
use crate::polar::ZPolar;

fn main() {
    let z1 = ZRect::new(3., 4.);
    let z1_tuple: (f64, f64) = z1.clone().into();
    let z1_polar = z1.to_polar();

    println!("Impedance (rectangular): {:.3}{:+.3}j", z1_tuple.0, z1_tuple.1);
    println!("Impedance (polar): {:.3}∠{:.3}", z1_polar.0, z1_polar.1);
    componentize(&z1, 50.);

    let x_axis = Line::new(&[(-100., 0.), (100., 0.)])
        .style(
            &Style::new().colour("#efefef")
        );
    let y_axis = Line::new(&[(0., 100.), (0., -100.)])
        .style(
            &Style::new().colour("#efefef")
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
        .x_label("Voltage")
        .y_label("Current")
        .x_range(-10., 10.)
        .y_range(-10., 10.);
    Page::single(&v).save("line.svg").unwrap();

    Command::new("firefox")
        .arg("-new-window")
        .arg("./line.svg")
        .output()
        .unwrap();
}

fn componentize(z: &ZRect, f: f64) {
    let z1: (f64, f64) = z.clone().into();

    let x: Reactance = match (z1.1).is_sign_negative() {
        true => Reactance::I(-z1.1),
        _ => Reactance::C(z1.1),
    };

    let omega = 2.0 * f64::consts::PI * f;

    let component = match x {
        Reactance::I(x_l) => println!("Inductance: {:.5} H", x_l as f64 / omega as f64),
        Reactance::C(x_c) => println!("Capacitance: {:.5} C", 1. / (x_c as f64 * omega as f64)),
    };

}
