use nannou::prelude::*;
use crate::flowfield::FlowField;
use crate::particle::{Particle, ParticleSegment};

mod geometry;
mod flowfield;
mod particle;

// https://www.reddit.com/r/proceduralgeneration/comments/10m1175/flow_fields_in_godot/
// https://guide.nannou.cc/getting_started/create_a_project.html

fn main() {
    nannou::app(model)
        .size(1600, 900)
        .update(update)
        .simple_window(view)
        .run();
}


struct Model {
    flow_field: FlowField,
    particles: Vec<Particle>,
    particle_alive: f32,
    particle_colors: (Srgba<f64>, Srgba<f64>)
}

fn model(app: &App) -> Model {
    let window = app.window_rect();
    let origin = srgba(1.0, 0.34901, 0.0901, 1.0);
    let target = srgba(0.9294, 0.5882, 0.0470, 0.0);

    Model {
        flow_field: FlowField::new(&window),
        particles: (0..1).flat_map(|_| {
            let x = random_range(window.left(), window.right());
            let y = random_range(window.bottom(), window.top());
            (0..100).map(move |_| {
                Particle::new(
                    x + ((random_f32() - 0.5) * 30.0),
                    y + ((random_f32() - 0.5) * 30.0),
                )
            })
        }).collect(),
        particle_alive: 2.0,
        particle_colors: (origin, target),
    }
}

fn color_lerp(start: &Srgba<f64>, end: &Srgba<f64>, t: f64) -> Srgba<f64> {
    return srgba(
        start.red + (end.red - start.red) * t,
        start.green + (end.green - start.green) * t,
        start.blue + (end.blue - start.blue) * t,
        start.alpha + (end.alpha - start.alpha) * t,
    );
}
fn update(app: &App, model: &mut Model, update: Update) {
    // check mouse input

    if app.mouse.buttons.left().is_down() {
        let mouse_position = app.mouse.position();
        model.particles.push(Particle::new(
                mouse_position.x + ((random_f32() - 0.5) * 30.0),
                mouse_position.y + ((random_f32() - 0.5) * 30.0),
        ));
    }

    let delta_time = update.since_last.secs();

    for particle in &mut model.particles {
        let update_result = particle.update(&model.flow_field, &app.window_rect(), delta_time);
        let current_position = particle.pos;

        if !update_result.jumped {
            particle.particle_trail.push(ParticleSegment {
                position: current_position.clone(),
                color: model.particle_colors.0.clone(),
                time_left: model.particle_alive + delta_time as f32,
            });
        } else {
            particle.particle_trail.clear();
        }
    }

    for particle in &mut model.particles {
        particle.particle_trail.iter_mut().for_each(|trail| {
            trail.time_left -= delta_time as f32;
            trail.color = color_lerp(
                &model.particle_colors.0,
                &model.particle_colors.1,
                1.0 - (trail.time_left / model.particle_alive) as f64
            );
        });
    }

    for particle in &mut model.particles {
        particle.particle_trail = particle.particle_trail.clone()
            .into_iter()
            .filter(|s| s.time_left > 0.0)
            .collect();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(srgba(0.1, 0.1, 0.1, 1.0));
    draw_framerate_and_frametime(app, &draw);

    // draw grid
    for (x, y) in &model.flow_field {
        let (sample_x, sample_y) = model.flow_field.scale(x, y);
        let normalized_value = model.flow_field.sample(sample_x, sample_y);

        let angle = normalized_value * 2.0 * PI;
        let x_target = x + 15.0 * angle.cos();
        let y_target = y + 15.0 * angle.sin();

        draw.line().start(Point2::new(x, y)).end(Point2::new(x_target, y_target))
            .weight(0.5)
            .color(WHITESMOKE);
    }

    // draw particles
    for particle in &model.particles {
        draw.ellipse().x_y(particle.pos.x, particle.pos.y).radius(3.0).color(RED);
    }

    // draw particle segments

    for particles in &model.particles {
        draw.polyline()
            .points_colored(particles.particle_trail.iter().map(|p| (p.position, p.color)));

    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_framerate_and_frametime(app: &App, draw: &Draw) {
    let framerate = app.fps();
    let frametime = app.duration.since_prev_update.as_secs_f32();

    let s = draw.text(&format!("{:.2} | {:.4}", framerate, frametime))
        .right_justify()
        .x_y(app.window_rect().left() + 10.0, app.window_rect().top() - 20.0);
}

