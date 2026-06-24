use winit::event_loop::ActiveEventLoop;

use crate::game::state::scene::Kind;

use super::effect::{actor, cursor, debug, menu, simulation};
use super::Game;

impl Game {
    pub fn update(&mut self, event_loop: &ActiveEventLoop) {
        self.overlay_buffer.clear();
        self.master_sink.set_volume(self.settings.volume);
        self.keyboard.update();
        self.mouse.update();
        self.cache.update();
        self.cross_fader.update();

        self.state_scene.advance();
        if matches!(self.state_scene.scene(), Kind::Quit) {
            event_loop.exit();
            return;
        }

        cursor::update(&self.handle, &self.state_scene);

        menu::intro::update(
            &mut self.state_scene,
            &mut self.state_menu_intro,
            &self.jingle_speaker,
            &self.keyboard,
            &mut self.overlay_buffer,
            self.sprite_resolution,
        );

        menu::home::update(
            &mut self.overlay_buffer,
            &mut self.state_scene,
            &mut self.state_menu_home,
            &self.keyboard,
            &self.select_speaker,
            &self.move_speaker,
        );

        menu::settings::update(menu::settings::UpdateParams {
            buffer: &mut self.overlay_buffer,
            state_scene: &mut self.state_scene,
            state_menu_settings: &mut self.state_menu_settings,
            keyboard: &self.keyboard,
            settings: &mut self.settings,
            surface: self.surface.as_ref(),
            device: self.device.as_ref(),
            surface_config: &mut self.surface_config,
            select_speaker: &self.select_speaker,
            move_speaker: &self.move_speaker,
            tick: self.tick,
        });

        menu::visit::update(menu::visit::UpdateParams {
            buffer: &mut self.overlay_buffer,
            state_scene: &mut self.state_scene,
            state_menu_visit: &mut self.state_menu_visit,
            keyboard: &self.keyboard,
            select_speaker: &self.select_speaker,
            move_speaker: &self.move_speaker,
            kinematics: &mut self.state_kinematics,
            intent: &mut self.state_intent,
            cross_fader: &mut self.cross_fader,
            cache: &mut self.cache,
            tick: self.tick,
        });

        menu::pause::update(
            &mut self.overlay_buffer,
            &mut self.state_scene,
            &mut self.state_menu_pause,
            &self.keyboard,
            &self.select_speaker,
            &self.move_speaker,
            &mut self.cross_fader,
        );

        menu::load::update(menu::load::UpdateParams {
            buffer: &mut self.overlay_buffer,
            resolution: self.sprite_resolution,
            state_menu_load: &mut self.state_menu_load,
            state_scene: &mut self.state_scene,
            keyboard: &self.keyboard,
            kinematics: &mut self.state_kinematics,
            intent: &mut self.state_intent,
            state_debug: &mut self.state_debug,
            cross_fader: &mut self.cross_fader,
            cache: &mut self.cache,
            move_speaker: &self.move_speaker,
        });

        simulation::update(&mut self.state_scene, &self.keyboard, &self.move_speaker);

        debug::update(
            &mut self.overlay_buffer,
            self.sprite_resolution,
            &self.state_scene,
            &self.keyboard,
            &mut self.state_debug,
            &self.log_listener,
        );

        actor::intent::update(
            &self.state_scene,
            &mut self.state_intent,
            &self.keyboard,
            self.mouse.delta(),
            &self.settings,
        );

        actor::kinematics::update(
            &self.state_scene,
            &mut self.state_kinematics,
            &mut self.state_intent,
            &mut self.cache,
            &mut self.cross_fader,
        );

        self.tick += 1;
    }
}
