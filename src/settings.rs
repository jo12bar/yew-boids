use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Settings {
    /// Number of boids.
    pub boids: usize,
    /// View distance of a boid.
    pub visible_range: f64,
    /// Distance boids try to keep between each other.
    pub min_distance: f64,
    /// Max speed.
    pub max_speed: f64,
    /// Force multiplier for pulling boids together.
    pub cohesion_factor: f64,
    /// Force multiplier for seperating boids.
    pub separation_factor: f64,
    /// Force multiplier for matching velocity of other boids.
    pub alignment_factor: f64,
    /// Controls turn speed to avoid leacing boundary.
    pub turn_speed_ratio: f64,
    /// Percentage of the size to the boundary at which a boid starts turning more.
    pub border_margin: f64,
    /// Factor for adapting the average color of the swarm.
    pub color_adapt_factor: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            boids: 300,
            visible_range: 80.0,
            min_distance: 15.0,
            max_speed: 20.0,
            alignment_factor: 0.15,
            cohesion_factor: 0.05,
            separation_factor: 0.6,
            turn_speed_ratio: 0.25,
            border_margin: 0.1,
            color_adapt_factor: 0.05,
        }
    }
}

impl Settings {
    const KEY: &'static str = "ca.jbarnard.yew-boids.settings";

    pub fn load() -> Self {
        LocalStorage::get(Self::KEY).unwrap_or_default()
    }

    pub fn remove() {
        LocalStorage::delete(Self::KEY);
    }

    pub fn store(&mut self) {
        // we explicitly don't care about setting storage errors - not mission-critical
        let _ = LocalStorage::set(Self::KEY, self);
    }
}
