use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use chrono::{Local, Timelike};

use crate::interactive::player::IsMusic;

use super::{game_selector::GameSelector, player::{MusicPlaying, MusicVolume}};


pub struct MusicImport;

impl Plugin for MusicImport {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player);
    }
}
fn player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Res<GameSelector>,
    res_music_vol: Res<MusicVolume>,
    res_playing: Res<MusicPlaying>,
    other_music: Query<Entity, With<IsMusic>>,
    // 0:minute 1:hour 2:bool to configure numbers at start
    mut debounce: bevy::prelude::Local<(u32, u32, bool)>,
) {
    if !(*debounce).2 {
        (*debounce).2 = true;
        (*debounce).0 = u32::MAX;
        (*debounce).1 = u32::MAX;
    }
    let thime = Local::now();
    // checks
    if !game.is_changed() {
        if (*debounce).0 == thime.minute() {
            return;
        };
        info!("check 1:min passed");
        (*debounce).0 = thime.minute();
        if (*debounce).1 == thime.hour() {
            return;
        };
        (*debounce).1 = thime.hour();
        info!("check 2:hour passed");
    }
    // passed

    let (apm, hour) = thime.hour12();
    let apm_str = if apm { "am" } else { "pm" };
    let time_str = format!("{hour}{apm_str}");
    let gametype = (*game.to_file_name()).to_string();
    info!("now playing: music/{gametype}/{time_str}.ogg");
    for ent in &other_music {
        commands.entity(ent).despawn_recursive();
    }
    commands.spawn(((
        IsMusic,
        AudioPlayer::new(asset_server.load(format!("music/{gametype}/{time_str}.ogg"))),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(res_music_vol.0),
            paused: !res_playing.0,
            ..default()
        },
        Name::new("MusicPlayer"),
    ),));
}
// fn check_assets_ready(
//     mut commands: Commands,
//     server: Res<AssetServer>,
//     loading: Res<AssetsLoading>
// ) {
//     use bevy::asset::LoadState;

//     match server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
//         LoadState::Failed => {
//             // one of our assets had an error
//         }
//         LoadState::Loaded => {
//             // all assets are now ready

//             // this might be a good place to transition into your in-game state

//             // remove the resource to drop the tracking handles
//             commands.remove_resource::<AssetsLoading>();
//             // (note: if you don't have any other handles to the assets
//             // elsewhere, they will get unloaded after this)
//         }
//         _ => {
//             // NotLoaded/Loading: not fully ready yet
//         }
//     }
// }