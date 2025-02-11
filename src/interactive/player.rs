use bevy::{audio::{PlaybackMode, Volume}, prelude::*};
use bevy_inspector_egui::{prelude::*, quick::ResourceInspectorPlugin};

use super::{topbar::PlayerButton, UiReflectSet};

pub struct PlayerImport;

#[derive(Resource, Reflect)]
pub struct MusicPlaying(pub bool);
impl Default for MusicPlaying {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Resource, Reflect)]
pub struct MusicVolume(f32);
impl Default for MusicVolume {
    fn default() -> Self {
        Self(0.5)
    }
}
#[derive(Component)]
pub struct IsMusic;

#[derive(Resource, Reflect)]
pub struct RainVolume(pub f32);
impl Default for RainVolume {
    fn default() -> Self {
        Self(0.5)
    }
}
#[derive(Component)]
pub struct IsRain;

impl Plugin for PlayerImport {
    fn build(&self, app: &mut App) {
        app
            // init resources
            .init_resource::<MusicPlaying>()
            .init_resource::<MusicVolume>()
            .init_resource::<RainVolume>()
            // register resources
            .register_type::<MusicPlaying>()
            .register_type::<MusicVolume>()
            .register_type::<RainVolume>()
            //make egui
            .add_plugins(ResourceInspectorPlugin::<MusicVolume>::default())
            .add_plugins(ResourceInspectorPlugin::<RainVolume>::default())
            // add systems
            .add_systems(Startup, setup)
            .add_systems(Update, update.in_set(UiReflectSet));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(((
        IsMusic,
        AudioPlayer::new(asset_server.load("music/12am.ogg")),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume:Volume::new(0.5),
            ..default()
        },
        Name::new("MusicPlayer")
    ),));
    commands.spawn(((
        IsRain,
        AudioPlayer::new(asset_server.load("music/game-rain.ogg")),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(0.5),
            ..default()
        },
        Name::new("RainPlayer")
    ),));
}

fn update(
    // playing
    res_playing: Res<MusicPlaying>,
    mut q_player: Query<&mut Text,With<PlayerButton>>,
    // rain volume
    res_rainvol: Res<RainVolume>,
    q_rainvol: Query<&AudioSink, With<IsRain>>,
    // music volume
    res_muswol: Res<MusicVolume>,
    q_musvol: Query<&AudioSink, With<IsMusic>>,
) {
    for music in &q_musvol {
        for rain in &q_rainvol {
            if res_playing.is_changed() {
                // state logic and icon logic
                if res_playing.0 {
                    music.play();
                    rain.play();
                    if let Ok(mut player_icon) = q_player.get_single_mut(){
                        **player_icon = String::from("");
                    }
                } else {
                    music.pause();
                    rain.pause();
                    if let Ok(mut player_icon) = q_player.get_single_mut(){
                        **player_icon = String::from("");
                    }
                }
            }

            if res_muswol.is_changed() {
                music.set_volume(res_muswol.0);
            }

            if res_rainvol.is_changed() {
                rain.set_volume(res_rainvol.0);
            }
        }
    }
}
