use std::{fs::File, io::Write, path::Path};

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_tokio_tasks::TokioTasksRuntime;

use crate::interactive::player::MusicVolume;

use super::player::{IsRain, MusicPlaying, RainVolume};

#[derive(Resource, Default, Reflect,PartialEq)]
#[allow(non_camel_case_types)]
pub enum RainType {
    #[default]
    game_rain,
    no_thunder_rain,
}
impl RainType {
    fn to_display_name() {
        todo!("lol i forgor")
    }
    fn to_file_name(&self) -> String {
        match self {
            RainType::game_rain => "game-rain",
            RainType::no_thunder_rain => "no-thunder-rain",
        }
        .to_string()
    }
}
#[derive(Event)]
struct LoadRain(String);

pub struct RainImport;

impl Plugin for RainImport {
    fn build(&self, app: &mut App) {
        app.init_resource::<RainType>()
            .register_type::<RainType>()
            .add_event::<LoadRain>()
            .add_plugins(ResourceInspectorPlugin::<RainType>::new())
            .add_systems(Update, (rain_reflect, load_rain.after(rain_reflect)));
    }
}

fn rain_reflect(
    mut commands: Commands,
    res_rain_vol: Res<RainVolume>,
    mut env_load_rain: EventWriter<LoadRain>,
    res_playing: Res<MusicPlaying>,
    asset_server: Res<AssetServer>,
    raintype: Res<RainType>,
    other_rain: Query<Entity, With<IsRain>>,
) {
    if raintype.is_changed() {
        // remove all rain players
        for ent in &other_rain {
            commands.entity(ent).despawn_recursive();
        }
        // new rain player
        let whatrain = raintype.to_file_name();

        if !Path::new(&format!("./assets/music/rain/{whatrain}.ogg")).exists() {
            info!("rain: path not found");
            env_load_rain.send(LoadRain(whatrain));
        } else {
            commands.spawn(((
                IsRain,
                AudioPlayer::new(asset_server.load(format!("music/rain/{whatrain}.ogg"))),
                PlaybackSettings {
                    mode: PlaybackMode::Loop,
                    volume: Volume::new(res_rain_vol.0),
                    paused: !res_playing.0,
                    ..default()
                },
                Name::new("RainPlayer"),
            ),));
        };
    }
}

fn load_rain(runtime: ResMut<TokioTasksRuntime>, mut env_load_rain: EventReader<LoadRain>) {
    for env in env_load_rain.read() {
        let whatrain = env.0.clone();
        runtime.spawn_background_task(move |mut ctx| async move {
            info!("This task is running on a background thread");
            let path_str = "./assets/music/rain/";
            let path = Path::new(&path_str);
            let target1 = format!("https://d17orwheorv96d.cloudfront.net/rain/{whatrain}.ogg");
            let response = reqwest::get(target1).await.expect("oh noes file not found");

            let mut dest = {
                let fname = response
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .unwrap_or("tmp.bin");

                info!("file to download: '{}'", fname);
                let fname = path.join(fname);
                info!("will be located under: '{:?}'", fname);
                File::create(fname)
            }
            .expect("file creattion go skkr");
            let content = response.bytes().await.expect("uhh 3");
            let res = dest.write_all(&content);
            if res.is_ok() {
                info!("task: all clear")
            } else {
                panic!("task: went to shit")
            };
            ctx.run_on_main_thread(move |ctx| {
                // The inner context gives access to a mutable Bevy World reference.
                let world: &mut World = ctx.world;
                let asset_server = world.resource::<AssetServer>();
                let res_rain_vol = world.resource::<RainVolume>();
                let res_playing = world.resource::<MusicPlaying>();

                world.spawn(((
                    IsRain,
                    AudioPlayer::new(asset_server.load(format!("music/rain/{whatrain}.ogg"))),
                    PlaybackSettings {
                        mode: PlaybackMode::Loop,
                        volume: Volume::new(res_rain_vol.0),
                        paused: !res_playing.0,
                        ..default()
                    },
                    Name::new("rainPlayer"),
                ),));
            })
            .await;
        });
    }
}
