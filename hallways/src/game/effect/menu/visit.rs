use std::sync::Arc;

use glam::{Vec2, Vec3};
use strum::{EnumCount, IntoEnumIterator};
use winit::keyboard::KeyCode;

use crate::audio::CrossFader;
use crate::audio::Speaker;
use crate::game::state::actor::Intent;
use crate::game::state::actor::Kinematics;
use crate::game::state::actor::Stance;
use crate::game::state::menu;
use crate::game::state::menu::visit::Item;
use crate::game::state::scene::Kind;
use crate::game::state::Keyboard;
use crate::game::state::Scene;
use crate::gpu::buffer::vertex::overlay;
use crate::level::Cache;
use crate::sprite::text::character::TEXT_SIZE;
use crate::sprite::text::option::State;
use crate::sprite::text::{Input, Option as TextOption};
use crate::sprite::Border;

const BORDER: f32 = 3.0;
const TEXT_PADDING: f32 = 3.0;
const SCREEN_PADDING: f32 = 6.0;
const ITEM_INDENT: f32 = TEXT_SIZE.x + 2.0;
const MAX_ITEM_NAME_LEN: usize = 14;
const MAX_ITEM_VALUE_LEN: usize = 48;
const INSET: f32 = BORDER + TEXT_PADDING;
const ITEM_COUNT: usize = Item::COUNT;
const ROW_WIDTH: f32 = ITEM_INDENT + (MAX_ITEM_NAME_LEN + MAX_ITEM_VALUE_LEN) as f32 * TEXT_SIZE.x;
const BOX_WIDTH: f32 = ROW_WIDTH + INSET * 2.0;
const BOX_HEIGHT: f32 = ITEM_COUNT as f32 * TEXT_SIZE.y + INSET * 2.0;

pub struct UpdateParams<'a> {
    pub buffer: &'a mut Vec<overlay::Data>,
    pub state_scene: &'a mut Scene,
    pub state_menu_visit: &'a mut menu::Visit,
    pub keyboard: &'a Keyboard,
    pub select_speaker: &'a Speaker,
    pub move_speaker: &'a Speaker,
    pub kinematics: &'a mut Kinematics,
    pub intent: &'a mut Intent,
    pub cross_fader: &'a mut CrossFader,
    pub cache: &'a mut Cache,
    pub tick: u64,
}

struct SelectItemParams<'a> {
    item: Item,
    state_menu_visit: &'a mut menu::Visit,
    state_scene: &'a mut Scene,
    kinematics: &'a mut Kinematics,
    intent: &'a mut Intent,
    cross_fader: &'a mut CrossFader,
    cache: &'a mut Cache,
}

struct UpdateItemParams<'a> {
    item: Item,
    state_menu_visit: &'a mut menu::Visit,
    buffer: &'a mut Vec<overlay::Data>,
    position: Vec2,
    hovered: bool,
    active_item: Option<Item>,
    keyboard: &'a Keyboard,
    move_speaker: &'a Speaker,
    tick: u64,
}

pub fn update(params: UpdateParams<'_>) {
    if !matches!(params.state_scene.scene(), Kind::MenuVisit) {
        return;
    }

    if params.state_scene.transitioned() {
        params.state_menu_visit.clear();
        params.kinematics.level_url = None;
    }

    let active_item = params
        .state_menu_visit
        .selected
        .then(|| Item::iter().nth(params.state_menu_visit.hovered))
        .flatten();

    if !params.state_menu_visit.selected && params.keyboard.pressed(KeyCode::ArrowUp, false) {
        params.state_menu_visit.hovered =
            (params.state_menu_visit.hovered + ITEM_COUNT - 1) % ITEM_COUNT;
        params.move_speaker.reset();
        params.move_speaker.play();
    } else if !params.state_menu_visit.selected
        && params.keyboard.pressed(KeyCode::ArrowDown, false)
    {
        params.state_menu_visit.hovered = (params.state_menu_visit.hovered + 1) % ITEM_COUNT;
        params.move_speaker.reset();
        params.move_speaker.play();
    } else if !params.state_menu_visit.selected && params.keyboard.pressed(KeyCode::Escape, false) {
        params.state_scene.set_scene(Kind::MenuHome);
        params.move_speaker.reset();
        params.move_speaker.play();
    } else if !params.state_menu_visit.selected && params.keyboard.pressed(KeyCode::Enter, false) {
        params.select_speaker.reset();
        params.select_speaker.play();
        if let Some(item) = Item::iter().nth(params.state_menu_visit.hovered) {
            select_item(SelectItemParams {
                item,
                state_menu_visit: params.state_menu_visit,
                state_scene: params.state_scene,
                kinematics: params.kinematics,
                intent: params.intent,
                cross_fader: params.cross_fader,
                cache: params.cache,
            });
        }
    }

    let box_pos = Vec2::new(SCREEN_PADDING, SCREEN_PADDING);
    Border::new(box_pos, Vec2::new(BOX_WIDTH, BOX_HEIGHT)).write(params.buffer);

    let content_x = box_pos.x + INSET;
    let content_y = box_pos.y + INSET;

    for (i, item) in Item::iter().enumerate() {
        let y = content_y + i as f32 * TEXT_SIZE.y;
        let hovered = i == params.state_menu_visit.hovered;
        update_item(UpdateItemParams {
            item,
            state_menu_visit: params.state_menu_visit,
            buffer: &mut *params.buffer,
            position: Vec2::new(content_x, y),
            hovered,
            active_item,
            keyboard: params.keyboard,
            move_speaker: params.move_speaker,
            tick: params.tick,
        });
    }
}

fn select_item(params: SelectItemParams<'_>) {
    match params.item {
        Item::LevelUrl => {
            params.state_menu_visit.selected = true;
        }
        Item::Visit => {
            if let Some(url) = params.state_menu_visit.level_url().value.cloned() {
                params.cross_fader.fade_out();
                params.kinematics.level_url = Some(Arc::new(url.clone()));
                params.kinematics.velocity = Vec3::ZERO;
                params.intent.float = false;
                params.intent.float_jump_time = None;
                params.kinematics.stance = Stance::Airborne { crouching: false };
                params.cache.get(&url);
                params.state_scene.set_scene(Kind::MenuLoad);
            }
        }
        Item::GoBack => {
            params.state_scene.set_scene(Kind::MenuHome);
        }
    }
}

fn update_item(params: UpdateItemParams<'_>) {
    if matches!(params.active_item, Some(item) if item == params.item) {
        if params.keyboard.pressed(KeyCode::Escape, false)
            || params.keyboard.pressed(KeyCode::Enter, false)
        {
            params.state_menu_visit.selected = false;
            params.move_speaker.reset();
            params.move_speaker.play();
        } else if let Item::LevelUrl = params.item {
            if params.keyboard.pressed(KeyCode::Backspace, false) {
                params.state_menu_visit.level_url_pop();
            }
            for c in params.keyboard.typed() {
                params.state_menu_visit.level_url_push(*c);
            }
        }
    }

    let active = params.state_menu_visit.selected
        && matches!(params.active_item, Some(item) if item == params.item);
    let level_url = params.state_menu_visit.level_url();
    let option_state = match params.item {
        Item::Visit => {
            if level_url.value.is_none() {
                State::Disabled
            } else if active {
                State::Selected
            } else {
                State::Unselected
            }
        }
        _ => {
            if active {
                State::Selected
            } else {
                State::Unselected
            }
        }
    };

    TextOption::new(
        params.position,
        MAX_ITEM_NAME_LEN,
        params.hovered,
        option_state,
        params.item.name(),
    )
    .write(params.buffer);

    if let Some(input) = matches!(params.item, Item::LevelUrl).then_some(Input::new(
        Vec2::new(
            params.position.x + ITEM_INDENT + MAX_ITEM_NAME_LEN as f32 * TEXT_SIZE.x,
            params.position.y,
        ),
        MAX_ITEM_VALUE_LEN,
        level_url.text,
        active,
        params.tick,
    )) {
        input.write(params.buffer);
    }
}
