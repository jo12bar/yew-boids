use crate::math::{self, smallest_angle_between, Mean, Vec2, WeightedMean};
use crate::settings::Settings;
use crate::simulation::SIZE;
use rand::Rng;
use std::{f64::consts::TAU, iter};
use yew::{html, Html};

/// Idk a bird or something.
#[derive(Clone, Debug, PartialEq)]
pub struct Boid {
    position: Vec2,
    velocity: Vec2,
    radius: f64,
    hue: f64,
}

impl Boid {
    /// Create a new random boid, based off of user settings.
    pub fn new_random<R: Rng + ?Sized>(rng: &mut R, settings: &Settings) -> Self {
        let max_radius = settings.min_distance / 2.0;
        let min_radius = max_radius / 6.0;

        // by using the third power large boids become rarer
        let radius = min_radius + rng.gen::<f64>().powi(3) * (max_radius - min_radius);

        Self {
            position: Vec2::new(rng.gen::<f64>() * SIZE.x, rng.gen::<f64>() * SIZE.y),
            velocity: Vec2::from_polar(rng.gen::<f64>() * TAU, settings.max_speed),
            radius,
            hue: rng.gen::<f64>() * TAU,
        }
    }

    /// Calculate the coherence this boid should have with others that it can see.
    fn coherence(&self, boids: VisibleBoidIter, factor: f64) -> Vec2 {
        Vec2::weighted_mean(
            boids.map(|other| (other.boid.position, other.boid.radius * other.boid.radius)),
        )
        .map(|mean| (mean - self.position) * factor)
        .unwrap_or_default()
    }

    /// Calculate the seperating force between this boid and others that it can see.
    fn separation(&self, boids: VisibleBoidIter, settings: &Settings) -> Vec2 {
        let accel = boids
            .filter_map(|other| {
                if other.distance > settings.min_distance {
                    None
                } else {
                    Some(-other.offset)
                }
            })
            .sum::<Vec2>();
        accel * settings.separation_factor
    }

    /// Calculate the alignment force between this boid and others that it can see.
    fn alignment(&self, boids: VisibleBoidIter, factor: f64) -> Vec2 {
        Vec2::mean(boids.map(|other| other.boid.velocity))
            .map(|mean| (mean - self.velocity) * factor)
            .unwrap_or_default()
    }

    /// Change this boid's colour to be closer to those around it.
    fn adapt_color(&mut self, boids: VisibleBoidIter, factor: f64) {
        let mean = f64::mean(boids.filter_map(|other| {
            if other.boid.radius > self.radius {
                Some(smallest_angle_between(self.hue, other.boid.hue))
            } else {
                None
            }
        }));

        if let Some(avg_hue_offset) = mean {
            self.hue += avg_hue_offset * factor;
        }
    }

    /// Keep the boid within the simulation area by making it turn a lot if it gets close to the
    /// edges.
    fn keep_in_bounds(&mut self, settings: &Settings) {
        let min = SIZE * settings.border_margin;
        let max = SIZE - min;

        let mut v = Vec2::default();

        let turn_speed = self.velocity.magnitude() * settings.turn_speed_ratio;
        let pos = self.position;

        if pos.x < min.x {
            v.x += turn_speed;
        }
        if pos.x > max.x {
            v.x -= turn_speed;
        }

        if pos.y < min.y {
            v.y += turn_speed;
        }
        if pos.y > max.y {
            v.y -= turn_speed;
        }

        self.velocity += v;
    }

    fn update_velocity(&mut self, settings: &Settings, boids: VisibleBoidIter) {
        let v = self.velocity
            + self.coherence(boids.clone(), settings.cohesion_factor)
            + self.separation(boids.clone(), settings)
            + self.alignment(boids, settings.alignment_factor);
        self.velocity = v.clamp_magnitude(settings.max_speed);
    }

    fn update(&mut self, settings: &Settings, boids: VisibleBoidIter, time_delta_ms: f64) {
        self.adapt_color(boids.clone(), settings.color_adapt_factor);
        self.update_velocity(settings, boids);
        self.keep_in_bounds(settings);
        self.position += self.velocity * time_delta_ms * 10.0e-3;
    }

    /// Update all boids by some time delta.
    pub fn update_all(settings: &Settings, boids: &mut [Self], time_delta_ms: f64) {
        for i in 0..boids.len() {
            let (before, after) = boids.split_at_mut(i);
            let (boid, after) = after.split_first_mut().unwrap();
            let visible_boids =
                VisibleBoidIter::new(before, after, boid.position, settings.visible_range);

            boid.update(settings, visible_boids, time_delta_ms);
        }
    }

    /// Render the boid as a SVG `<polygon>` element.
    pub fn render(&self) -> Html {
        let color = format!("hsl({:.3}rad, 100%, 50%)", self.hue);

        let mut points = String::new();
        for offset in iter_shape_points(self.radius, self.velocity.angle()) {
            let Vec2 { x, y } = self.position + offset;
            points.push_str(&format!("{x:.2},{y:.2} "));
        }

        html! { <polygon {points} fill={color} /> }
    }
}

fn iter_shape_points(radius: f64, rotation: f64) -> impl Iterator<Item = Vec2> {
    const SHAPE: [(f64, f64); 3] = [
        (0.0 * math::FRAC_TAU_3, 2.0),
        (1.0 * math::FRAC_TAU_3, 1.0),
        (2.0 * math::FRAC_TAU_3, 1.0),
    ];
    SHAPE
        .iter()
        .copied()
        .map(move |(angle, radius_mul)| Vec2::from_polar(angle + rotation, radius_mul * radius))
}

/// A [`Boid`] that is visible to another.
#[derive(Debug)]
struct VisibleBoid<'a> {
    boid: &'a Boid,
    offset: Vec2,
    distance: f64,
}

/// An iterator over all boids that a specific [`Boid`] can see.
#[derive(Clone, Debug)]
struct VisibleBoidIter<'boid> {
    /// this iterator's type is the result of chaining `before` and `after` in the function [`Self::new()`]
    /// together
    it: iter::Chain<std::slice::Iter<'boid, Boid>, std::slice::Iter<'boid, Boid>>,
    position: Vec2,
    visible_range: f64,
}

impl<'boid> VisibleBoidIter<'boid> {
    fn new(
        before: &'boid [Boid],
        after: &'boid [Boid],
        position: Vec2,
        visible_range: f64,
    ) -> Self {
        Self {
            it: before.iter().chain(after),
            position,
            visible_range,
        }
    }
}

impl<'boid> Iterator for VisibleBoidIter<'boid> {
    type Item = VisibleBoid<'boid>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            ref mut it,
            position,
            visible_range,
        } = *self;

        it.find_map(move |other| {
            let offset = other.position - position;
            let distance = offset.magnitude();

            if distance > visible_range {
                None
            } else {
                Some(VisibleBoid {
                    boid: other,
                    offset,
                    distance,
                })
            }
        })
    }
}
