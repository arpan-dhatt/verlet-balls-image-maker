use std::env::args;

use nannou::prelude::*;

struct Model {
    balls: Vec<Ball>
}

#[derive(Clone, Debug)]
struct Ball {
    x: f32,
    y: f32,
    x1: f32,
    y1: f32,
    r: f32
}

const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 1024.0;
const GRAV: f32 = -980.0;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().size(WIDTH as u32, HEIGHT as u32).event(event).view(view).build().unwrap();
    Model { balls: (0..args().into_iter().nth(1).unwrap().parse().unwrap()).map(|_| rand_ball()).collect() }
}

fn rand_ball() -> Ball {
    let x = random::<f32>() * WIDTH - WIDTH/2.0;
    let y = random::<f32>() * HEIGHT - HEIGHT/2.0;
    Ball {
        x, y, x1: x, y1: y,
        r: random::<f32>() + 5.0
    }
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(_) => {
            model.balls.push(Ball {
                x: app.mouse.x,
                y: app.mouse.y,
                x1: app.mouse.x,
                y1: app.mouse.y,
                r: 10.0
            });
        }
        _ => {}
    }
}

fn step(model: &mut Model, mut delta: f32, substeps: usize) {
    delta /= substeps as f32;
    for _ in 0..substeps {
        for i in 0..model.balls.len() {
            let (front, back) = model.balls.split_at_mut(i + 1);
            let ball_a = unsafe { front.last_mut().unwrap_unchecked() };
            (ball_a.x, ball_a.x1) = (2.0*ball_a.x - ball_a.x1 + 0.0f32, ball_a.x);
            (ball_a.y, ball_a.y1) = (2.0*ball_a.y - ball_a.y1 + GRAV * delta * delta, ball_a.y);

            ball_a.x = ball_a.x.clamp(-WIDTH/2.0 + ball_a.r, WIDTH/2.0 - ball_a.r);
            ball_a.y = ball_a.y.clamp(-HEIGHT/2.0 + ball_a.r, HEIGHT/2.0 - ball_a.r);

            for ball_b in back {
                let diff = (ball_b.x - ball_a.x, ball_b.y - ball_a.y);
                let dist = ((diff.0 * diff.0) + (diff.1 * diff.1)).sqrt();
                if dist < (ball_a.r + ball_b.r) {
                    let norm = (diff.0 / dist, diff.1 / dist);
                    ball_a.x -= 0.5f32 * (((ball_a.r + ball_b.r) - dist) * norm.0);
                    ball_a.y -= 0.5f32 * (((ball_a.r + ball_b.r) - dist) * norm.1);

                    ball_b.x += 0.5f32 * (((ball_a.r + ball_b.r) - dist) * norm.0);
                    ball_b.y += 0.5f32 * (((ball_a.r + ball_b.r) - dist) * norm.1);
                }
            }
        }
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    step(model, 0.016, 8);
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(PLUM);

    // Draw a blue ellipse with default size and position.
    for ball in &model.balls {
        draw
            .ellipse()
            .color(STEELBLUE)
            .x_y(ball.x, ball.y)
            .radius(ball.r);
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}