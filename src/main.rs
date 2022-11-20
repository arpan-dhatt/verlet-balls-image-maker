use std::{collections::VecDeque, env::args, mem, ops::DerefMut, sync::Mutex};

use lazy_static::lazy_static;
use nannou::{
    color::encoding::Srgb,
    image::{imageops::FilterType, io::Reader as ImageReader, GenericImageView, Pixel},
    prelude::*,
};
use rand::{rngs::SmallRng, Rng, SeedableRng};

lazy_static! {
    static ref RNG: Mutex<SmallRng> = Mutex::new(SmallRng::from_seed([5; 16]));
}

struct Model {
    image: String,
    colors: VecDeque<rgb::Rgb<Srgb, u8>>,
    balls: Vec<Ball>,
    emitters: Vec<Emitter>,
}

#[derive(Clone, Debug)]
struct Ball {
    p: Vec2,
    p1: Vec2,
    r: f32,
    c: rgb::Rgb<Srgb, u8>,
}

#[derive(Clone, Debug)]
struct Emitter {
    count: usize,
    theta: f32,
    rv: f32,
}

const PERIOD: usize = 1;
const POWER: f32 = 8.0;

impl Emitter {
    fn update(&mut self) {
        self.theta += self.rv;
    }

    fn emit(&mut self, colors: &mut VecDeque<rgb::Rgb<Srgb, u8>>) -> Option<Ball> {
        self.count += 1;
        if self.count % PERIOD == 0 && !colors.is_empty() {
            let (x, y) = (
                (RAD - 8.0) * self.theta.cos(),
                (RAD - 8.0) * self.theta.sin(),
            );
            let (x1, y1) = (
                (RAD - 8.0 + POWER) * self.theta.cos(),
                (RAD - 8.0 + POWER) * self.theta.sin(),
            );
            return Some(Ball {
                p: vec2(x, y),
                p1: vec2(x1, y1),
                r: RNG.lock().unwrap().gen_range(4.0, 10.0),
                c: colors.pop_front().unwrap(),
            });
        } else {
            None
        }
    }
}

const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 1024.0;
const RAD: f32 = 480.0;
const GRAV: f32 = -9800.0;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .event(event)
        .view(view)
        .build()
        .unwrap();
    Model {
        image: args().nth(1).unwrap(),
        colors: (0..4000).into_iter().map(|_| rgb(255, 255, 255)).collect(),
        balls: vec![],
        emitters: (0..4).into_iter().map(|_| rand_emitter()).collect(),
    }
}

fn rand_emitter() -> Emitter {
    let mut rng = RNG.lock().unwrap();
    let theta = rng.gen_range(0.0, PI * 2.0);
    return Emitter {
        theta,
        rv: rng.gen_range(4.0 / PI / 60.0, 6.0 / PI / 60.0),
        count: 0,
    };
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(Key::Return) => {
            {
                let mut rng = RNG.lock().unwrap();
                let _ = mem::replace(rng.deref_mut(), SmallRng::from_seed([0; 16]));
                println!("replaced rng");
            }
            get_colors(model);
            *model = Model {
                image: model.image.clone(),
                colors: get_colors(model),
                balls: vec![],
                emitters: (0..4).into_iter().map(|_| rand_emitter()).collect(),
            };
            println!("replaced model");
        }
        _ => {}
    }
}

fn get_colors(model: &mut Model) -> VecDeque<rgb::Rgb<Srgb, u8>> {
    let image = ImageReader::open(&model.image)
        .unwrap()
        .decode()
        .unwrap()
        .resize_exact(WIDTH as u32, HEIGHT as u32, FilterType::Lanczos3);
    model
        .balls
        .iter()
        .map(|b| {
            let (i, j) = coord_to_pix(b.p);
            let mut col = (0, 0, 0);
            let mut count = 0;
            for k in (-b.r as i32)..(b.r as i32) {
                let y = (b.r * b.r - (k * k) as f32).sqrt() as i32;
                for l in (-y)..y {
                    let (x, y) = (i + k, j + l);
                    if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                        let temp = image.get_pixel(x as u32, y as u32);
                        col.0 += temp.0[0] as u32;
                        col.1 += temp.0[1] as u32;
                        col.2 += temp.0[2] as u32;
                        count += 1;
                    }
                }
            }
            if count > 0 {
                rgb(
                    (col.0 / count) as u8,
                    (col.1 / count) as u8,
                    (col.2 / count) as u8,
                )
            } else {
                rgb(0, 0, 0)
            }
        })
        .collect()
    // model.balls.iter_mut().for_each(|b| {
    //     b.c = {
    //         let (i, j) = coord_to_pix(b.p);
    //         let col = image.get_pixel(i, j);
    //         rgb(col.0[0], col.0[1], col.0[2])
    //     }
    // });
}

fn coord_to_pix(c: Vec2) -> (i32, i32) {
    (
        (c.x as i32 + WIDTH as i32 / 2).clamp(0, WIDTH as i32 - 1),
        (-c.y as i32 + HEIGHT as i32 / 2).clamp(0, HEIGHT as i32 - 1),
    )
}

fn step(model: &mut Model, mut delta: f32, substeps: usize) {
    let grav = vec2(0.0, GRAV);
    delta /= substeps as f32;
    for _ in 0..substeps {
        for i in 0..model.balls.len() {
            let (front, back) = model.balls.split_at_mut(i + 1);
            let mut ball_a = unsafe { front.last_mut().unwrap_unchecked() };
            (ball_a.p, ball_a.p1) = (2.0 * ball_a.p - ball_a.p1 + grav * delta * delta, ball_a.p);
            // ball_a.x = ball_a
            //     .x
            //     .clamp(-WIDTH / 2.0 + ball_a.r, WIDTH / 2.0 - ball_a.r);
            // ball_a.y = ball_a
            //     .y
            //     .clamp(-HEIGHT / 2.0 + ball_a.r, HEIGHT / 2.0 - ball_a.r);
            clamp_circle(&mut ball_a);

            for ball_b in back {
                let diff = ball_b.p - ball_a.p;
                let limit = ball_a.r + ball_b.r;
                // check manhatten distance first
                // if diff.0.abs() > limit && diff.1.abs() > limit {
                //     continue;
                // }
                let dist = diff.length();
                if dist < limit {
                    let norm = diff.normalize();
                    let part = (ball_a.r * ball_a.r) / (ball_a.r * ball_a.r + ball_b.r * ball_b.r);
                    ball_a.p -= (1.0 - part) * ((limit - dist) * norm);

                    ball_b.p += part * ((limit - dist) * norm);
                }
            }
        }
    }
}

fn clamp_circle(ball: &mut Ball) {
    let rad = ball.p.length() + ball.r;
    if rad > RAD {
        ball.p += -ball.p.normalize() * (rad - RAD);
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if !model.colors.is_empty() {
        let new_balls = model.emitters.iter_mut().filter_map(|e| {
            e.update();
            e.emit(&mut model.colors)
        });
        model.balls.extend(new_balls);
    }
    step(model, 0.001, 4);
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(BLACK);

    // Draw a blue ellipse with default size and position.
    for ball in &model.balls {
        draw.ellipse()
            .color(ball.c)
            .x_y(ball.p.x, ball.p.y)
            .radius(ball.r);
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
