use crate::{
    colours::{self, SELECTOR_PURBLE2},
    music::{GrandfatherMode, SaturdayKkMode},
    rain::{self, RainType},
};
use bevy::{picking::focus::PickingInteraction, prelude::*};

#[derive(Component, Debug)]
#[require(Node(||{Node{width:Val::Px(14.0),height:Val::Px(14.0), ..default()}}))]
#[require(Button)]
pub enum Tickbox {
    Grandfather,
    Rain,
    DontDownload,
    TownTune,
    PlayKk,
    Startup,
}
#[derive(Component, Debug)]
pub struct TickTick;

pub struct TickboxImport;
impl Plugin for TickboxImport {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                hydrate_tickbox,
                react_tickbox.after(hydrate_tickbox),
                draw_tickbox.after(react_tickbox),
            ),
        );
    }
}

fn hydrate_tickbox(mut commands: Commands, q_tick: Query<(Entity, &Tickbox), Added<Tickbox>>) {
    for (ent, tick) in &q_tick {
        commands
            .entity(ent)
            .insert((BackgroundColor(colours::SLIDER_BLUE)));
    }
}
fn react_tickbox(
    q_tick: Query<(&Tickbox, &PickingInteraction)>,
    mut res_rain: ResMut<RainType>,
    mut res_grandfather: ResMut<GrandfatherMode>,
    mut res_kk: ResMut<SaturdayKkMode>,
) {
    // for every tickbox, toggle res if pressed
    for (tick, pick) in &q_tick {
        if *pick == PickingInteraction::Pressed {
            info!("pressed: {:?}", tick);
            match tick {
                Tickbox::Rain => {
                    if *res_rain == RainType::game_rain {
                        *res_rain = RainType::no_thunder_rain
                    } else {
                        *res_rain = RainType::game_rain
                    }
                }
                Tickbox::Grandfather => res_grandfather.0 = !res_grandfather.0,
                Tickbox::PlayKk => res_kk.0 = !res_kk.0,
                _ => todo!(),
            };
        }
    }
}
fn draw_tickbox(
    mut q_tick: Query<(&Tickbox, &mut BackgroundColor), With<Tickbox>>,
    res_rain: Res<RainType>,
    res_grandfather: ResMut<GrandfatherMode>,
    res_kk: ResMut<SaturdayKkMode>,
) {
    if !res_rain.is_changed() & !res_grandfather.is_changed() & res_kk.is_changed() {
        return;
    }
    for (tick, mut bgcol) in &mut q_tick {
        let state = match tick {
            Tickbox::Rain => *res_rain == RainType::game_rain,
            Tickbox::Grandfather => res_grandfather.0,
            Tickbox::PlayKk => res_kk.0,
            _ => todo!(),
        };
        if state {
            bgcol.0 = colours::SLIDER_BLUE
        } else {
            bgcol.0 = Color::WHITE
        }
    }
}
