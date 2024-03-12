use nannou::prelude::*;
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
        let flock = flock.to_vec();
        Self { flock }
    }

    fn draw(&self, draw: &Draw) {
        let boid_color = WHITE;

        draw.background().color(BLACK);

        for boid in &self.flock {
            draw.ellipse()
                .x_y(boid.position.x, boid.position.y)
                .radius(3.0)
                .color(boid_color);
        }
    }

    fn update(&mut self, app: &App) {
        let window = app.window_rect();

        let max_force = 0.1;
        let max_speed = 5.0;

        let flock_dup = self.flock.clone();

        for boid in &mut self.flock {
            let mut steering = vec2(0.0, 0.0);
            let mut cohesion = vec2(0.0, 0.0);
            let mut seperation = vec2(0.0, 0.0);

            let perception_radius = 50.0;
            let seperation_radius = 25.0;
            let mut total = 0;
            let mut seperation_total = 0;

            for other in &flock_dup {
                let d = boid.position.distance(other.position);
                if d < perception_radius && *other != *boid {
                    steering += other.velocity;
                    cohesion += other.position;

                    total += 1;
                }

                if d < seperation_radius && *other != *boid {
                    let mut diff = boid.position - other.position;
                    diff /= d * d;
                    seperation += diff;

                    seperation_total += 1;
                }
            }
            if total > 0 {
                steering /= total as f32;
                steering = steering.normalize() * max_speed;
                steering -= boid.velocity;
                steering = steering.clamp_length_max(max_force);

                cohesion /= total as f32;
                cohesion -= boid.position;
                cohesion = cohesion.normalize() * max_speed;
                cohesion -= boid.velocity;
                cohesion = cohesion.clamp_length_max(max_force);
            }

            if seperation_total > 0 {
                seperation /= seperation_total as f32;
                seperation = seperation.normalize() * max_speed;
                seperation -= boid.velocity;
                seperation = seperation.clamp_length_max(max_force);
            }

            boid.acceleration += cohesion * 0.37 + steering * 0.85 + seperation;

            boid.position += boid.velocity;
            boid.velocity += boid.acceleration;

            boid.velocity = boid.velocity.clamp_length_max(max_speed);

            if boid.position.x > window.right() {
                boid.position.x = window.left();
            } else if boid.position.x < window.left() {
                boid.position.x = window.right();
            }

            if boid.position.y > window.top() {
                boid.position.y = window.bottom();
            } else if boid.position.y < window.bottom() {
                boid.position.y = window.top();
            }
        }
    }
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    let circles: Vec<Boid> = (0..300)
        .rev()
        .map(|_| Boid {
            position: vec2(random_range(-500.0, 500.0), random_range(-500.0, 500.0)),
            velocity: vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0)),
            acceleration: vec2(0.0, 0.0),
        })
        .collect();
    Model::new(&circles)
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.update(app);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    model.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
