use std::process::Command;
use std::convert::{ From, Into };
use std::f64;
use std::rc::Rc;

use plotlib::view::ContinuousView;
use plotlib::page::Page;
use plotlib::line::{ Line, Style };
use plotlib::style::Line as linestyle;

use cursive::Cursive;
use cursive::views::{
    Dialog, TextView, DummyView, EditView,
    LinearLayout, Button, IdView
};

use cursive::view::{ Identifiable, Boxable };

mod rect;
use crate::rect::ZRect;

pub enum Reactance {
    I(f64),
    C(f64)
}

mod polar;
use crate::polar::ZPolar;

fn main() {
    let mut siv = Cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    let dialog = Dialog::around(
        LinearLayout::vertical()
        .child(TextView::new("Input Impedance (X + jY)"))
        .child(
            EditView::new()
            .with_id("imp_rect")
            .fixed_width(20)
        )
        .child(TextView::new("Frequency (Hz)"))
        .child(
            EditView::new()
            .with_id("freq")
            .fixed_width(20)
        )
        .child(
            Button::new("Submit", |s| {
                let imp_rect = s.
                    call_on_id("imp_rect", |view: &mut EditView| {
                        view.get_content()
                    }).unwrap();
                let freq = s.
                    call_on_id("freq", |view: &mut EditView| {
                        view.get_content()
                    }).unwrap();
                eprintln!("woop");
                let (imp_rect_real, imp_rect_img) = parser(&imp_rect);
                let freq = freq.parse::<f64>().unwrap();
                componentize(s, (imp_rect_real, imp_rect_img), freq);
            })
        ) 
        .child(
            Button::new("Quit", |s| s.quit())
        )
    ).title("Results of Calculation");

    siv.add_layer(dialog);
    siv.run();
}

fn componentize(s: &mut Cursive, z: (f64, f64), f: f64) {
    let z1_polar: ZPolar = ZRect::new(z.0, z.1).to_polar();
    let z1: (f64, f64) = z.clone().into();

    let r: f64 = z1.0.abs();

    let x: Reactance = match (z1.1).is_sign_negative() {
        true => Reactance::I(-z1.1),
        _ => Reactance::C(z1.1),
    };

    let omega = 2.0 * f64::consts::PI * f;

    let component = match x {
        Reactance::I(x_l) => format!("Inductance: {:.5} H", x_l as f64 / omega as f64),
        Reactance::C(x_c) => format!("Capacitance: {:.5} C", 1. / (x_c as f64 * omega as f64)),
    };

    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
            .child(TextView::new(format!("Impedance (Rectangular): {:.5} {:+.5}j", z1.0, z1.1)))
            .child(TextView::new(format!("Impedance (Polar): {:.5} ∠ {:.5}", z1_polar.0, z1_polar.1)))
            .child(TextView::new(component))
            .child(TextView::new(format!("Resistance: {:.5}", r)))
        )
        .button("Ok", |s| match s.pop_layer() {
            _ => {}
        })
        .button("Show Graph", |_| {
            Command::new("firefox")
                .arg("-new-window")
                .arg("./line.svg")
                .output()
                .unwrap();
        })
    );
    draw_graph(z);
}

fn draw_graph(z: (f64, f64)) {
    let x_axis = Line::new(&[(-100., 0.), (100., 0.)])
        .style(
            &Style::new().colour("#efefef")
        );
    let y_axis = Line::new(&[(0., 100.), (0., -100.)])
        .style(
            &Style::new().colour("#efefef")
        );
    let z_resultant = Line::new(&[(0., 0.), z]).style(
        &Style::new().colour("red")
    );
    let resistance = Line::new(&[(0., 0.), (z.0, 0.)]).style(
        &Style::new().colour("blue")
    );
    let reactance = Line::new(&[(0., 0.), (0., z.1)]).style(
        &Style::new().colour("blue")
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

}

fn parser(input: &str) -> (f64, f64){
    let basic = input.trim();
    let basic = basic.replace(" ", "");

    let mut letter_buf = vec![];
    let mut parts: Vec<String> = vec![];


    for letter in basic.chars() {
        match letter {
            '+' | '-' => {
                parts.push(letter_buf.iter().collect());
                letter_buf.clear();
            }
            _ => {
                letter_buf.push(letter);
            }
        }
    };

    parts.push(letter_buf.iter().collect());

    let part_1 = parts[0].parse::<f64>().unwrap();
    let part_2 = parts[1].replace("j", "").parse::<f64>().unwrap();

    (part_1, part_2)
}
