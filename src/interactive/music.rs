//use error_chain::error_chain;
use std::fs::File;
use std::{io::Write, path::Path};

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_tokio_tasks::TokioTasksRuntime;
use chrono::{Local, Timelike};

use crate::interactive::player::IsMusic;

use super::{
    game_selector::GameSelector,
    player::{MusicPlaying, MusicVolume},
};

#[derive(Event)]
// 0: game as string, 1: time as string
struct LoadMusic(String, String);

pub struct MusicImport;

impl Plugin for MusicImport {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadMusic>()
            .add_systems(Update, (player,load_music.after(player)));
    }
}
fn player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Res<GameSelector>,
    res_music_vol: Res<MusicVolume>,
    res_playing: Res<MusicPlaying>,
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
    if (*debounce).2 == *game {
        return;
    }
    (debounce.2) = game.clone();
    // passed

    let (apm, hour) = thime.hour12();
    let apm_str = if apm { "am" } else { "pm" };
    let time_str = format!("{hour}{apm_str}");
    let gametype = (*game.to_file_name()).to_string();
    info!("now playing: music/{gametype}/{time_str}.ogg");
    for ent in &other_music {
        commands.entity(ent).despawn_recursive();
    }
    if !Path::new(&format!("./assets/music/{gametype}/{time_str}.ogg")).exists() {
        info!("path not found");
        env_load_mus.send(LoadMusic(gametype, time_str));
    } else {
        commands.spawn((
            IsMusic,
            AudioPlayer::new(asset_server.load(format!("music/{gametype}/{time_str}.ogg"))),
            PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(res_music_vol.0),
                paused: !res_playing.0,
                ..default()
            },
            Name::new("MusicPlayer"),
        ));
    };
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

                world.spawn((
                    IsMusic,
                    AudioPlayer::new(asset_server.load(format!("music/{gametype}/{time_str}.ogg"))),
                    PlaybackSettings {
                        mode: PlaybackMode::Loop,
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
// #[tokio::main]
// async fn main() -> Result<(),()> {
//     let tmp_dir = Builder::new().prefix("example").tempdir()?;
//     let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
//     let response = reqwest::get(target).await?;

//     let mut dest = {
//         let fname = response
//             .url()
//             .path_segments()
//             .and_then(|segments| segments.last())
//             .and_then(|name| if name.is_empty() { None } else { Some(name) })
//             .unwrap_or("tmp.bin");

//         println!("file to download: '{}'", fname);
//         let fname = tmp_dir.path().join(fname);
//         println!("will be located under: '{:?}'", fname);
//         File::create(fname)?
//     };
//     let content =  response.bytes().await?;
//     dest.write_all(&content)?;
//     Ok(())
// }
