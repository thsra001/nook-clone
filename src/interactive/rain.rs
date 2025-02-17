use bevy::{audio::{PlaybackMode, Volume}, prelude::*};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use super::player::{IsRain, MusicPlaying, RainVolume};

#[derive(Resource,Default,Reflect)]
#[allow(non_camel_case_types)]
pub enum RainType {
    #[default]
    game_rain,
    no_thunder_rain
}
impl RainType {
    fn to_display_name(){todo!("lol i forgor")}
    fn to_file_name(&self) -> String{
      match self {
          RainType::game_rain => "game-rain",
          RainType::no_thunder_rain => "no-thunder-rain"
      }.to_string()
    }
}

pub struct RainImport;

impl Plugin for RainImport {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<RainType>()
        .register_type::<RainType>()
        .add_plugins(ResourceInspectorPlugin::<RainType>::new())
        .add_systems(Update, rain_reflect);
    }
}

fn rain_reflect(mut commands:Commands,res_rain_vol:Res<RainVolume>,res_playing:Res<MusicPlaying>,asset_server:Res<AssetServer>,raintype:Res<RainType>,other_rain:Query<Entity,With<IsRain>>){
if raintype.is_changed(){
    // remove all rain players
    for ent in &other_rain {
        commands.entity(ent).despawn_recursive();
    }
    // new rain player
    let israin = raintype.to_file_name();
    commands.spawn(((
        IsRain,
        AudioPlayer::new(asset_server.load(format!("music/rain/{israin}.ogg"))),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(res_rain_vol.0),
            paused: !res_playing.0,
            ..default()
        },
        Name::new("MusicPlayer"),
    ),));
}
}