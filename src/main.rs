use std::vec;

use nannou::prelude::*;

fn main() {
    nannou::app(model).simple_window(view).update(update).run();
}

struct Model {
    angle: f32,
    radius: f32,
    angle_constant: f32,
    lines: Vec<Vec<Vec<f32>>>,
    fast_point_x: f32,
    fast_point_y: f32,
    slow_point_x: f32,
    slow_point_y: f32,
}

fn model(_app: &App) -> Model {
    Model {
        angle: 0.0,
        radius: 180.0,
        angle_constant: 2.0,
        lines: vec![],
        fast_point_x: 0.0,
        fast_point_y: 0.0,
        slow_point_x: 0.0,
        slow_point_y: 0.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.angle += deg_to_rad(model.angle_constant);
    model.slow_point_y = model.radius * (model.angle / 2.0).cos();
    model.slow_point_x = model.radius * (model.angle / 2.0).sin();
    model.fast_point_y = model.radius * model.angle.cos();
    model.fast_point_x = model.radius * model.angle.sin();

    model.lines.push(vec![
        vec![model.fast_point_x, model.fast_point_y],
        vec![model.slow_point_x, model.slow_point_y],
    ]);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let fast_point_x = model.fast_point_x;
    let fast_point_y = model.fast_point_y;
    let slow_point_x = model.slow_point_x;
    let slow_point_y = model.slow_point_y;

    draw.polyline()
        .weight(1.0)
        .points_colored((0..=360).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x = radian.sin() * model.radius;
            let y = radian.cos() * model.radius;

            (pt2(x, y), WHITE)
        }));

    draw.background().color(BLACK);

    draw.ellipse()
        .radius(2.0)
        .xy(pt2(slow_point_x, slow_point_y))
        .color(WHITE);
    draw.ellipse()
        .radius(2.0)
        .xy(pt2(fast_point_x, fast_point_y))
        .color(WHITE);

    for line in &model.lines {
        draw.line()
            .start(pt2(line[0][0], line[0][1]))
            .end(pt2(line[1][0], line[1][1]))
            .color(CRIMSON);
    }

    draw.to_frame(app, &frame).unwrap();
}
