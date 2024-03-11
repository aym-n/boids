use nannou::{ease::circ, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Boid {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
}

struct Model {
    flock: Vec<Boid>,
}

impl Model {
    fn new(flock: &[Boid]) -> Self {
        // make the circles nested
        let mut flock = flock.to_vec();
        Self { flock }
    }

    fn draw(&self, draw: &Draw) {
        let circle_color = WHITE;
        let circle_stroke_weight = 3.0;

        draw.background().color(DARKSLATEGRAY);

        for circle in &self.flock {
            draw.ellipse()
                .x_y(circle.position.x, circle.position.y)
                .radius(10.0)
                .stroke_color(circle_color)
                .stroke_weight(circle_stroke_weight)
                .no_fill();
        }
    }

    fn update(&mut self) {
        // align 
        let flock_dup = self.flock.clone();
        for circle in &mut self.flock {
            let mut steering = vec2(0.0, 0.0);
            let perception_radius = 50.0;
            let mut total = 0;
            for other in &flock_dup {
                let d = circle.position.distance(other.position);
                if d < perception_radius && *other != *circle {
                    steering += other.velocity;
                    total += 1;
                }
            }
            if total > 0 {
                steering /= total as f32;
                steering -= circle.velocity;
            }
            circle.acceleration = steering;
        }

        for circle in &mut self.flock {
            circle.position += circle.velocity;
            circle.velocity += circle.acceleration;
        }
    }
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    let circles: Vec<Boid> = (0..100)
        .rev()
        .map(|_| Boid {
            position: vec2(random_range(-500.0, 500.0), random_range(-500.0, 500.0)),
            velocity: vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0)),
            acceleration: vec2(0.0, 0.0),
        })
        .collect();
    Model::new(&circles)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    model.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
