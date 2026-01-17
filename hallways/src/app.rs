use std::time::Duration;
use std::time::Instant;

use glam::Vec2;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoop;
use winit::window::WindowId;

use crate::game::Game;

pub const SIM_STEP: Duration = Duration::from_millis(10);

pub struct App {
    last_update: Instant,
    window_title: String,
    game: Option<Game>,
}

impl App {
    pub fn new() -> Self {
        return Self {
            last_update: Instant::now(),
            window_title: String::new(),
            game: None,
        };
    }

    pub fn run(mut self, title: &str) {
        env_logger::init();
        self.window_title = title.to_string();

        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(&mut self).unwrap();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.last_update = Instant::now();
        self.game = Some(Game::create(&self.window_title, event_loop));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                if let Some(game) = self.game.as_mut() {
                    game.key_event(&event);
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(game) = self.game.as_mut() {
                    game.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(game) = self.game.as_mut() {
                    while self.last_update.elapsed() >= SIM_STEP {
                        game.update(event_loop);
                        self.last_update += SIM_STEP;
                    }

                    game.render();
                    game.schedule();
                }
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        if let DeviceEvent::MouseMotion { delta } = event {
            if let Some(game) = self.game.as_mut() {
                let delta = Vec2::new(delta.0 as f32, delta.1 as f32);
                game.mouse_motion(delta);
            }
        }
    }
}
