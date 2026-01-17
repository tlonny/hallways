use glam::Vec2;
use strum::{EnumCount, IntoEnumIterator};
use winit::keyboard::KeyCode;

use crate::audio::CrossFader;
use crate::audio::Speaker;
use crate::game::state::menu::pause::Item;
use crate::game::state::menu::Pause;
use crate::game::state::scene::Kind;
use crate::game::state::Keyboard;
use crate::game::state::Scene;
use crate::gpu::buffer::vertex::overlay;
use crate::sprite::text::character::TEXT_SIZE;
use crate::sprite::text::option::State;
use crate::sprite::text::Option as TextOption;
use crate::sprite::Border;

const BORDER: f32 = 3.0;
const TEXT_PADDING: f32 = 3.0;
const SCREEN_PADDING: f32 = 6.0;
const MAX_ITEM_LEN: usize = 6;
const INSET: f32 = BORDER + TEXT_PADDING;
const ITEM_INDENT: f32 = TEXT_SIZE.x + 2.0;

const ROW_WIDTH: f32 = ITEM_INDENT + MAX_ITEM_LEN as f32 * TEXT_SIZE.x;
const BOX_WIDTH: f32 = ROW_WIDTH + INSET * 2.0;
const BOX_HEIGHT: f32 = Item::COUNT as f32 * TEXT_SIZE.y + INSET * 2.0;

pub fn update(
    buffer: &mut Vec<overlay::Data>,
    state_scene: &mut Scene,
    state_menu_pause: &mut Pause,
    keyboard: &Keyboard,
    select_speaker: &Speaker,
    move_speaker: &Speaker,
    cross_fader: &mut CrossFader,
) {
    if !matches!(state_scene.scene(), Kind::MenuPause) {
        return;
    }

    if state_scene.transitioned() {
        state_menu_pause.clear();
    }

    if keyboard.pressed(KeyCode::ArrowUp, false) {
        state_menu_pause.selected = (state_menu_pause.selected + Item::COUNT - 1) % Item::COUNT;
        move_speaker.reset();
        move_speaker.play();
    } else if keyboard.pressed(KeyCode::ArrowDown, false) {
        state_menu_pause.selected = (state_menu_pause.selected + 1) % Item::COUNT;
        move_speaker.reset();
        move_speaker.play();
    } else if keyboard.pressed(KeyCode::Escape, false) {
        move_speaker.reset();
        move_speaker.play();
        state_scene.set_scene(Kind::Simulation);
    } else if keyboard.pressed(KeyCode::Enter, false) {
        select_speaker.reset();
        select_speaker.play();
        if let Some(item) = Item::iter().nth(state_menu_pause.selected) {
            select_item(item, state_scene, cross_fader);
        }
    }

    let box_pos = Vec2::new(SCREEN_PADDING, SCREEN_PADDING);
    Border::new(box_pos, Vec2::new(BOX_WIDTH, BOX_HEIGHT)).write(buffer);

    let content_x = box_pos.x + INSET;
    let content_y = box_pos.y + INSET;

    for (i, item) in Item::iter().enumerate() {
        let y = content_y + i as f32 * TEXT_SIZE.y;
        let hovered = i == state_menu_pause.selected;
        TextOption::new(
            Vec2::new(content_x, y),
            MAX_ITEM_LEN,
            hovered,
            State::Unselected,
            item.name(),
        )
        .write(buffer);
    }
}

fn select_item(item: Item, state_scene: &mut Scene, cross_fader: &mut CrossFader) {
    match item {
        Item::Cancel => state_scene.set_scene(Kind::Simulation),
        Item::Exit => {
            cross_fader.fade_out();
            state_scene.set_scene(Kind::MenuVisit);
        }
    }
}
