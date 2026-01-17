mod effect;
mod render;
mod resize;
pub mod state;
pub mod update;

use std::sync::Arc;

use glam::Vec2;
use rodio::{OutputStream, Sink};
use strum::{EnumCount, IntoEnumIterator};
use winit::event::KeyEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

use crate::audio::CrossFader;
use crate::audio::Data;
use crate::audio::Speaker;
use crate::game::state::actor::Intent;
use crate::game::state::actor::Kinematics;
use crate::game::state::menu;
use crate::game::state::Debug;
use crate::game::state::Scene;
use crate::gpu::bind_group::Composite;
use crate::gpu::bind_group::Overlay;
use crate::gpu::bind_group::Portal;
use crate::gpu::bind_group::World;
use crate::gpu::buffer::uniform::{self, resolution};
use crate::gpu::buffer::vertex::{self, overlay};
use crate::gpu::pipeline;
use crate::gpu::texture::Array;
use crate::gpu::texture::Color;
use crate::gpu::texture::Depth;
use crate::gpu::texture::OitAccum;
use crate::gpu::texture::OitReveal;
use crate::level::render::PortalFrameBuffer;
use crate::level::Cache;
use crate::settings::Settings;
use crate::sprite::TextureKind;
use crate::util;
use crate::ASSET;

use crate::game::state::{Keyboard, Mouse};

const JINGLE_AUDIO_PATH: &str = "audio/jingle.wav";
const SELECT_AUDIO_PATH: &str = "audio/select.wav";
const MOVE_AUDIO_PATH: &str = "audio/move.wav";
const AUDIO_CHANNEL_COUNT: u16 = 2;
const AUDIO_SAMPLE_RATE: u32 = 48_000;
const OVERLAY_MODEL_VERTEX_CAPACITY: usize = 50_000;
const PORTAL_TEXTURE_COUNT: usize = 6;
const SPRITE_TEXTURE_SIZE: (u32, u32) = (512, 512);

pub struct Game {
    handle: Arc<Window>,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    surface: Arc<wgpu::Surface<'static>>,
    surface_config: wgpu::SurfaceConfiguration,
    pipeline_level: pipeline::level::Opaque,
    pipeline_level_transparent: pipeline::level::Transparent,
    pipeline_composite: pipeline::Composite,
    pipeline_portal: pipeline::Portal,
    pipeline_overlay: pipeline::Overlay,
    depth_texture: Depth,
    overlay_bind_group: Overlay,
    sprite_resolution: glam::Vec2,
    sprite_resolution_buffer: uniform::Resolution,
    overlay_buffer: Vec<overlay::Data>,
    overlay_vertex_buffer: vertex::Overlay,
    oit_accum_texture: OitAccum,
    oit_reveal_texture: OitReveal,
    composite_bind_group: Composite,
    camera_buffer: uniform::Camera,
    clock_buffer: uniform::Clock,
    world_bind_group: World,
    portal_frame_buffers: [PortalFrameBuffer; PORTAL_TEXTURE_COUNT],
    cache: Cache,
    _audio_stream: OutputStream,
    settings: Settings,
    state_scene: Scene,
    state_menu_intro: menu::Intro,
    state_kinematics: Kinematics,
    state_intent: Intent,
    state_menu_home: menu::Home,
    state_debug: Debug,
    log_listener: util::log::Listener,
    state_menu_settings: menu::Settings,
    state_menu_visit: menu::Visit,
    state_menu_pause: menu::Pause,
    state_menu_load: menu::Load,
    tick: u64,
    master_sink: Sink,
    cross_fader: CrossFader,
    jingle_speaker: Speaker,
    select_speaker: Speaker,
    move_speaker: Speaker,
    keyboard: Keyboard,
    mouse: Mouse,
}

impl Game {
    pub fn create(title: &str, event_loop: &ActiveEventLoop) -> Self {
        let settings = Settings::load();
        let state_menu_home = menu::Home::new();
        let state_debug = Debug::new();
        let log_listener = util::log::Listener::new();
        let state_menu_settings = menu::Settings::new(&settings);
        let state_menu_visit = menu::Visit::new(&settings);
        let state_menu_pause = menu::Pause::new();
        let state_menu_load = menu::Load::new();

        let (_audio_stream, audio) = rodio::OutputStream::try_default().unwrap();
        let (mixer_ctrl, mixer_src) =
            rodio::dynamic_mixer::mixer::<f32>(AUDIO_CHANNEL_COUNT, AUDIO_SAMPLE_RATE);
        let master_sink = rodio::Sink::try_new(&audio).unwrap();
        master_sink.append(mixer_src);
        let mut cross_fader = CrossFader::create();
        mixer_ctrl.add(cross_fader.source());
        let jingle_data =
            Data::create(ASSET.get_file(JINGLE_AUDIO_PATH).unwrap().contents(), false).unwrap();
        let jingle_speaker = Speaker::create(jingle_data);
        mixer_ctrl.add(jingle_speaker.source());
        let select_data =
            Data::create(ASSET.get_file(SELECT_AUDIO_PATH).unwrap().contents(), false).unwrap();
        let select_speaker = Speaker::create(select_data);
        mixer_ctrl.add(select_speaker.source());
        let move_data =
            Data::create(ASSET.get_file(MOVE_AUDIO_PATH).unwrap().contents(), false).unwrap();
        let move_speaker = Speaker::create(move_data);
        mixer_ctrl.add(move_speaker.source());
        let state_scene = Scene::new();
        let state_menu_intro = menu::Intro::new();
        let state_kinematics = Kinematics::new(glam::Vec3::ZERO);
        let state_intent = Intent::new();

        let attributes = Window::default_attributes()
            .with_title(title)
            .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        let handle = Arc::new(event_loop.create_window(attributes).unwrap());

        let instance = wgpu::Instance::default();
        let surface = Arc::new(instance.create_surface(handle.clone()).unwrap());
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::DEPTH_CLIP_CONTROL
                    | wgpu::Features::TEXTURE_BINDING_ARRAY
                    | wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING,
                required_limits: wgpu::Limits::default(),
                ..Default::default()
            },
            None,
        ))
        .unwrap();

        let size = handle.inner_size();
        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let mut surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface_config.present_mode = settings.vsync_status.present_mode();

        let pipeline_level = pipeline::level::Opaque::create(&device, surface_config.format);
        let pipeline_level_transparent = pipeline::level::Transparent::create(&device);
        let pipeline_composite = pipeline::Composite::create(&device, surface_config.format);
        let pipeline_portal = pipeline::Portal::create(&device, surface_config.format);
        let pipeline_overlay = pipeline::Overlay::create(&device, surface_config.format);
        let cache = Cache::new(Arc::clone(&device), Arc::clone(&queue));

        let depth_texture = Depth::create(&device, surface_config.width, surface_config.height);
        let oit_accum_texture =
            OitAccum::create(&device, (surface_config.width, surface_config.height));
        let oit_reveal_texture =
            OitReveal::create(&device, (surface_config.width, surface_config.height));
        let composite_bind_group =
            Composite::create(&device, &oit_accum_texture, &oit_reveal_texture);
        let camera_buffer = uniform::Camera::create(&device);
        let clock_buffer = uniform::Clock::create(&device);
        let world_bind_group = World::create(&device, &camera_buffer, &clock_buffer);
        let portal_frame_buffers = std::array::from_fn(|_| {
            let color_texture = Color::create(
                &device,
                (surface_config.width, surface_config.height),
                surface_config.format,
            );
            let bind_group = Portal::create(&device, &color_texture);
            let camera_buffer = uniform::Camera::create(&device);
            let world_bind_group = World::create(&device, &camera_buffer, &clock_buffer);
            return PortalFrameBuffer {
                color_texture,
                bind_group,
                camera_buffer,
                world_bind_group,
            };
        });
        let scale = (surface_config.width as f32 / resize::TARGET_WIDTH)
            .floor()
            .max(1.0);
        let sprite_resolution = glam::Vec2::new(
            surface_config.width as f32 / scale,
            surface_config.height as f32 / scale,
        );

        let sprite_texture = Array::create(&device, SPRITE_TEXTURE_SIZE, TextureKind::COUNT);
        for kind in TextureKind::iter() {
            let data = kind.data();
            let image = image::load_from_memory(ASSET.get_file(data.path).unwrap().contents())
                .unwrap()
                .to_rgba8();
            sprite_texture.write(&queue, data.ix as usize, &image);
        }
        let sprite_resolution_buffer = uniform::Resolution::create(&device);
        sprite_resolution_buffer.write(&queue, resolution::Data::new(sprite_resolution));
        let overlay_bind_group =
            Overlay::create(&device, &sprite_texture, &sprite_resolution_buffer);
        let overlay_buffer: Vec<overlay::Data> = Vec::new();
        let overlay_vertex_buffer = vertex::Overlay::create(&device, OVERLAY_MODEL_VERTEX_CAPACITY);
        let keyboard = Keyboard::new();
        let mouse = Mouse::new();

        let game = Self {
            handle,
            device,
            queue,
            surface,
            surface_config,
            pipeline_level,
            pipeline_level_transparent,
            pipeline_composite,
            pipeline_portal,
            pipeline_overlay,
            depth_texture,
            overlay_bind_group,
            sprite_resolution,
            sprite_resolution_buffer,
            overlay_buffer,
            overlay_vertex_buffer,
            oit_accum_texture,
            oit_reveal_texture,
            composite_bind_group,
            camera_buffer,
            clock_buffer,
            world_bind_group,
            portal_frame_buffers,
            cache,
            _audio_stream,
            settings,
            state_scene,
            state_menu_intro,
            state_kinematics,
            state_intent,
            state_menu_home,
            state_debug,
            log_listener,
            state_menu_settings,
            state_menu_visit,
            state_menu_pause,
            state_menu_load,
            tick: 0,
            master_sink,
            cross_fader,
            jingle_speaker,
            select_speaker,
            move_speaker,
            keyboard,
            mouse,
        };

        game.surface.configure(&game.device, &game.surface_config);
        return game;
    }

    pub fn mouse_motion(&mut self, delta: Vec2) {
        self.mouse.push_motion(delta);
    }

    pub fn key_event(&mut self, event: &KeyEvent) {
        self.keyboard.push(event);
    }

    pub fn schedule(&self) {
        self.handle.request_redraw();
    }
}
