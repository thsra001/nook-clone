//use error_chain::error_chain;
use std::fs::{self, File};
use std::{io::Write, path::Path};

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_tokio_tasks::TokioTasksRuntime;
use chrono::{Local, Timelike};

use crate::interactive::player::IsMusic;

use super::{
    game_selector::GameSelector,
    player::{MusicPlaying, MusicVolume},
};
#[derive(Resource,Default,Reflect)]
pub struct GrandfatherMode(pub bool);
#[derive(Resource,Default,Reflect)]
pub struct SaturdayKkMode(pub bool);

fn hour_to_pocket_camp(hour: String) -> String {
    // const MORNING:[&str] = ["5am", "6am", "7am", "8am"];
    // const DAY:[&str] = ["9am", "10am", "11am", "12pm", "1pm", "2pm", "3pm", "4pm"];
    // const EVENING:[&str] = ["5pm", "6pm"];
    // const NIGHT:[&str] = ["7pm", "8pm", "9pm", "10pm", "11pm", "12am", "1am", "2am", "3am", "4am"];

    match hour.as_str() {
        "5am" | "6am" | "7am" | "8am" => String::from("morning"),
        "9am" | "10am" | "11am" | "12pm" | "1pm" | "2pm" | "3pm" | "4pm" => String::from("day"),
        "5pm" | "6pm" => String::from("evening"),
        "7pm" | "8pm" | "9pm" | "10pm" | "11pm" | "12am" | "1am" | "2am" | "3am" | "4am" => {
            String::from("night")
        }
        _ => panic!("oh hell naw jigsaw yo hour string tweakin!"),
    }
    // if (~morning.indexOf(hour)) return "morning"
    // if (~day.indexOf(hour)) return "day"
    // if (~evening.indexOf(hour)) return "evening"
    // if (~night.indexOf(hour)) return "night"
}

#[derive(Event)]
// 0: game as string, 1: time as string
struct LoadMusic(String, String);

pub struct MusicImport;

impl Plugin for MusicImport {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadMusic>()
           .init_resource::<GrandfatherMode>()
           .init_resource::<SaturdayKkMode>()
           .register_type::<GrandfatherMode>()
           .register_type::<SaturdayKkMode>()
           .add_plugins(ResourceInspectorPlugin::<GrandfatherMode>::default())
           .add_plugins(ResourceInspectorPlugin::<SaturdayKkMode>::default())
            .add_systems(Update, (player, load_music.after(player)));
    }
}
fn player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Res<GameSelector>,
    res_music_vol: Res<MusicVolume>,
    res_playing: Res<MusicPlaying>,
    res_grandfather: Res<GrandfatherMode>,
    mut env_load_mus: EventWriter<LoadMusic>,
    other_music: Query<Entity, With<IsMusic>>,
    // 0:minute 1:hour 2: game 3:bool to configure numbers at start
    mut debounce: bevy::prelude::Local<(u32, u32, GameSelector, bool)>,
) {
    if !(*debounce).3 {
        (*debounce).3 = true;
        (*debounce).0 = u32::MAX;
        (*debounce).1 = u32::MAX;
    }
    let thime = Local::now();
    // checks
    if (*debounce).0 == thime.minute() {
        if (*debounce).2 == *game {
            debug!("failed:check 1");
            return;
        };
    };
    debug!("passed:check 1");
    (*debounce).0 = thime.minute();
    if (*debounce).1 == thime.hour() {
        if (*debounce).2 == *game {
            return;
        }
        (debounce.2) = game.clone();
    };
    (*debounce).1 = thime.hour();
    info!("passed:check 2");

    // passed

    let (apm, hour) = thime.hour12();
    let apm_str = if apm { "pm" } else { "am" };

    // game manager
    let gametype = match *game {
        GameSelector::random => GameSelector::random_game().to_file_name().to_string(),
        _ => (*game.to_file_name()).to_string(),
    };
    // pocket camp manager
    let time_str = if *game==GameSelector::random && gametype == "pocket-camp" || *game == GameSelector::pocket_camp  {
      hour_to_pocket_camp(format!("{hour}{apm_str}"))
    } else {
        format!("{hour}{apm_str}")
    };
    // let time_str = match gametype.as_str() {
    //     "pocket-camp" => hour_to_pocket_camp(format!("{hour}{apm_str}")),
    //     _ => {
    //         format!("{hour}{apm_str}")
    //     }
    // };
    //kk manager
    if *game == GameSelector::kk_slider {
        todo!("kk go fuck yourself")
    }
    info!("now playing: music/{gametype}/{time_str}.ogg");
    for ent in &other_music {
        commands.entity(ent).despawn_recursive();
    }
    if !Path::new(&format!("./assets/music/")).exists() {
        fs::create_dir(&format!("./assets/music/")).expect("music folder err");
    }
    if !Path::new(&format!("./assets/music/{gametype}")).exists() {
        fs::create_dir(&format!("./assets/music/{gametype}")).expect("game folder err");
    }

    if !Path::new(&format!("./assets/music/{gametype}/{time_str}.ogg")).exists() {
        info!("path not found");
        env_load_mus.send(LoadMusic(gametype, time_str));
    } else {
        commands.spawn((
            IsMusic,
            AudioPlayer::new(asset_server.load(format!("music/{gametype}/{time_str}.ogg"))),
            PlaybackSettings {
                mode: if res_grandfather.0 {PlaybackMode::Once} else {PlaybackMode::Loop},
                volume: Volume::new(res_music_vol.0),
                paused: !res_playing.0,
                ..default()
            },
            Name::new("MusicPlayer"),
        ));
    };
}
fn load_music(runtime: ResMut<TokioTasksRuntime>, mut env_load_mus: EventReader<LoadMusic>) {
    for env in env_load_mus.read() {
        let gametype = env.0.clone();
        let time_str = env.1.clone();
        runtime.spawn_background_task(move |mut ctx| async move {
            info!("This task is running on a background thread");
            let path_str = format!("./assets/music/{gametype}");
            let path = Path::new(&path_str);
            let target1 =
                format!("https://d17orwheorv96d.cloudfront.net/{gametype}/{time_str}.ogg");
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
                let res_music_vol = world.resource::<MusicVolume>();
                let res_playing = world.resource::<MusicPlaying>();
                let res_grandfather = world.resource::<GrandfatherMode>();

                world.spawn((
                    IsMusic,
                    AudioPlayer::new(asset_server.load(format!("music/{gametype}/{time_str}.ogg"))),
                    PlaybackSettings {
                        mode: if res_grandfather.0 {PlaybackMode::Once} else {PlaybackMode::Loop},
                        volume: Volume::new(res_music_vol.0),
                        paused: !res_playing.0,
                        ..default()
                    },
                    Name::new("MusicPlayer"),
                ));
            })
            .await;
        });
    }
}
