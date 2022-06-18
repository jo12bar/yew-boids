use crate::{boid::Boid, math::Vec2, settings::Settings};
use gloo::render::{request_animation_frame, AnimationFrame};
use yew::{html, Component, Context, Html, Properties};

/// The size of the simulation area.
pub const SIZE: Vec2 = Vec2::new(1600.0, 1000.0);

#[derive(Debug)]
pub enum Msg {
    Tick(f64),
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub settings: Settings,
    #[prop_or_default]
    pub generation: usize,
    #[prop_or_default]
    pub paused: bool,
}

#[derive(Debug)]
pub struct Simulation {
    boids: Vec<Boid>,
    animation_frame_handle: Option<AnimationFrame>,
    last_render_timestamp_ms: f64,
}

impl Component for Simulation {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut rng = rand::thread_rng();

        let settings = &ctx.props().settings;
        let boids = (0..settings.boids)
            .map(|_| Boid::new_random(&mut rng, settings))
            .collect();

        let mut this = Self {
            boids,
            animation_frame_handle: None,
            last_render_timestamp_ms: 0.0,
        };

        this.request_animation_frame(ctx);

        this
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick(timestamp) => {
                let Props {
                    ref settings,
                    paused,
                    ..
                } = *ctx.props();

                if paused {
                    false
                } else {
                    let time_delta = timestamp - self.last_render_timestamp_ms;
                    self.last_render_timestamp_ms = timestamp;

                    Boid::update_all(settings, &mut self.boids, time_delta);
                    self.request_animation_frame(ctx);
                    true
                }
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.boids.clear();

        let mut rng = rand::thread_rng();

        let settings = &ctx.props().settings;
        self.boids
            .resize_with(settings.boids, || Boid::new_random(&mut rng, settings));

        self.request_animation_frame(ctx);

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let view_box = format!("0 0 {} {}", SIZE.x, SIZE.y);

        html! {
            <svg class="simulation-window" viewBox={view_box}>
                { for self.boids.iter().map(Boid::render) }
            </svg>
        }
    }
}

impl Simulation {
    fn request_animation_frame(&mut self, ctx: &Context<Self>) {
        // as soon as the previous task is dropped it is cancelled. we don't need to worry about
        // manually stopping it.
        self.animation_frame_handle = Some({
            let link = ctx.link().clone();
            request_animation_frame(move |timestamp| link.send_message(Msg::Tick(timestamp)))
        });
    }
}
