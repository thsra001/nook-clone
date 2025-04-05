use crate::{
    colours::{self, SELECTOR_PURBLE2}, interactive::ButtonSet, music::{kkwhichsongs, GrandfatherMode, OfflineMode, SaturdayKkMode, TownTune}, rain::{self, RainType}
};
use bevy::{picking::focus::PickingInteraction, prelude::*};

#[derive(Resource, Default, PartialEq)]
pub struct StartupMode(pub bool);

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
    Kksong(String)
}
#[derive(Component, Debug)]
pub struct TickTick;

pub struct TickboxImport;
impl Plugin for TickboxImport {
    fn build(&self, app: &mut App) {
        app.init_resource::<StartupMode>().add_systems(
            Update,
            (
                hydrate_tickbox,
                react_tickbox.after(hydrate_tickbox).in_set(ButtonSet),
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
    mut res_town: ResMut<TownTune>,
    mut res_offline: ResMut<OfflineMode>,
    mut res_startup: ResMut<StartupMode>,
    mut res_kksongs: ResMut<kkwhichsongs>
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
                Tickbox::TownTune => res_town.0 = !res_town.0,
                Tickbox::DontDownload => res_offline.0 = !res_offline.0,
                Tickbox::Startup => res_startup.0 = !res_startup.0,
                Tickbox::Kksong(song) => if res_kksongs.0.contains(song) {res_kksongs.0.retain(|val| &val != &song);} else {res_kksongs.0.push(song.to_string());},
            };
        }
    }
}
fn draw_tickbox(
    mut q_tick: Query<(&Tickbox, &mut BackgroundColor), With<Tickbox>>,
    res_rain: Res<RainType>,
    res_grandfather: ResMut<GrandfatherMode>,
    res_kk: ResMut<SaturdayKkMode>,
    res_town: ResMut<TownTune>,
    res_offline: ResMut<OfflineMode>,
    res_startup: ResMut<StartupMode>,
    res_kksongs: Res<kkwhichsongs>
) {
    if res_rain.is_changed()
        | res_grandfather.is_changed()
        | res_kk.is_changed()
        | res_town.is_changed()
        | res_offline.is_changed()
        | res_startup.is_changed()
        | res_kksongs.is_changed()
    {
        return;
    }

    for (tick, mut bgcol) in &mut q_tick {
        let state = match tick {
            Tickbox::Rain => *res_rain == RainType::game_rain,
            Tickbox::Grandfather => res_grandfather.0,
            Tickbox::PlayKk => res_kk.0,
            Tickbox::TownTune => res_town.0,
            Tickbox::DontDownload => res_offline.0,
            Tickbox::Startup => res_startup.0,
            Tickbox::Kksong(song) => {res_kksongs.0.contains(song)}
        };
        if state {
            bgcol.0 = colours::SLIDER_BLUE
        } else {
            bgcol.0 = Color::WHITE
        }
    }
}
