use core::panic;
//use error_chain::error_chain;
use std::fs::{self, File};
use std::str::FromStr;
use std::{io::Write, path::Path};

use bevy::input::common_conditions::input_toggle_active;
use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_tokio_tasks::TokioTasksRuntime;
use chrono::{Datelike, Local, Timelike, Weekday};
use rand::seq::IndexedRandom;
use reqwest::header::CONTENT_TYPE;

use rsmpeg::error::RsmpegError;
// ffmpeg
//use anyhow::{Context, Result};
use rsmpeg::{
    avcodec::AVPacket,
    avformat::{AVFormatContextInput, AVFormatContextOutput},
    ffi::AVRational,
};
use std::ffi::{CStr, CString};
//

use crate::interactive::player::IsMusic;
use crate::tray2::UserEvent;

use super::{
    game_selector::GameSelector,
    player::{MusicPlaying, MusicVolume},
};
#[derive(Resource, Default, Reflect)]
pub struct GrandfatherMode(pub bool);
#[derive(Resource, Default, Reflect)]
pub struct SaturdayKkMode(pub bool);
#[derive(Resource, Default, Reflect)]
pub struct OfflineMode(pub bool);
#[derive(Resource, Default, Reflect)]
pub struct TownTune(pub bool);

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

pub const kkSongs: [&str; 193] = [
    "Agent K.K.",
    "Aloha K.K.",
    "Animal City",
    "Bubblegum K.K.",
    "Cafe K.K.",
    "Comrade K.K.",
    "DJ K.K.",
    "Drivin",
    "Farewell",
    "Forest Life",
    "Go K.K. Rider!",
    "Hypno K.K.",
    "I Love You",
    "Imperial K.K.",
    "K.K. Adventure",
    "K.K. Aria",
    "K.K. Ballad",
    "K.K. Bazaar",
    "K.K. Birthday",
    "K.K. Blues",
    "K.K. Bossa",
    "K.K. Calypso",
    "K.K. Casbah",
    "K.K. Chorale",
    "K.K. Condor",
    "K.K. Country",
    "K.K. Cruisin",
    "K.K. D&B",
    "K.K. Dirge",
    "K.K. Disco",
    "K.K. Dixie",
    "K.K. Faire",
    "K.K. Flamenco",
    "K.K. Folk",
    "K.K. Fusion",
    "K.K. Groove",
    "K.K. Gumbo",
    "K.K. House",
    "K.K. Island",
    "K.K. Jazz",
    "K.K. Jongara",
    "K.K. Lament",
    "K.K. Love Song",
    "K.K. Lullaby",
    "K.K. Mambo",
    "K.K. Marathon",
    "K.K. March",
    "K.K. Metal",
    "K.K. Milonga",
    "K.K. Moody",
    "K.K. Oasis",
    "K.K. Parade",
    "K.K. Ragtime",
    "K.K. Rally",
    "K.K. Reggae",
    "K.K. Rock",
    "K.K. Rockabilly",
    "K.K. Safari",
    "K.K. Salsa",
    "K.K. Samba",
    "K.K. Ska",
    "K.K. Sonata",
    "K.K. Song",
    "K.K. Soul",
    "K.K. Steppe",
    "K.K. Stroll",
    "K.K. Swing",
    "K.K. Synth",
    "K.K. Tango",
    "K.K. Technopop",
    "K.K. Waltz",
    "K.K. Western",
    "K.K. Etude",
    "King K.K.",
    "Lucky K.K.",
    "Marine Song 2001",
    "Mountain Song",
    "Mr. K.K.",
    "My Place",
    "Neapolitan",
    "Only Me",
    "Pondering",
    "Rockin K.K.",
    "Senor K.K.",
    "Soulful K.K.",
    "Space K.K.",
    "Spring Blossoms",
    "Stale Cupcakes",
    "Steep Hill",
    "Surfin K.K.",
    "The K. Funk",
    "To the Edge",
    "Two Days Ago",
    "Unknown 01",
    "Unknown 02",
    "Wandering",
    "Welcome Horizons",
    "Wild World",
    "Agent K.K. (Radio)",
    "Aloha K.K. (Radio)",
    "Animal City (Radio)",
    "Bubblegum K.K. (Radio)",
    "Cafe K.K. (Radio)",
    "Comrade K.K. (Radio)",
    "DJ K.K. (Radio)",
    "Drivin (Radio)",
    "Farewell (Radio)",
    "Forest Life (Radio)",
    "Go K.K. Rider! (Radio)",
    "Hypno K.K. (Radio)",
    "I Love You (Radio)",
    "Imperial K.K. (Radio)",
    "K.K. Adventure (Radio)",
    "K.K. Aria (Radio)",
    "K.K. Ballad (Radio)",
    "K.K. Bazaar (Radio)",
    "K.K. Birthday (Radio)",
    "K.K. Blues (Radio)",
    "K.K. Bossa (Radio)",
    "K.K. Calypso (Radio)",
    "K.K. Casbah (Radio)",
    "K.K. Chorale (Radio)",
    "K.K. Condor (Radio)",
    "K.K. Country (Radio)",
    "K.K. Cruisin (Radio)",
    "K.K. D&B (Radio)",
    "K.K. Dirge (Radio)",
    "K.K. Disco (Radio)",
    "K.K. Dixie (Radio)",
    "K.K. Faire (Radio)",
    "K.K. Flamenco (Radio)",
    "K.K. Folk (Radio)",
    "K.K. Fusion (Radio)",
    "K.K. Groove (Radio)",
    "K.K. Gumbo (Radio)",
    "K.K. House (Radio)",
    "K.K. Island (Radio)",
    "K.K. Jazz (Radio)",
    "K.K. Jongara (Radio)",
    "K.K. Lament (Radio)",
    "K.K. Love Song (Radio)",
    "K.K. Lullaby (Radio)",
    "K.K. Mambo (Radio)",
    "K.K. Marathon (Radio)",
    "K.K. March (Radio)",
    "K.K. Metal (Radio)",
    "K.K. Milonga (Radio)",
    "K.K. Moody (Radio)",
    "K.K. Oasis (Radio)",
    "K.K. Parade (Radio)",
    "K.K. Ragtime (Radio)",
    "K.K. Rally (Radio)",
    "K.K. Reggae (Radio)",
    "K.K. Rock (Radio)",
    "K.K. Rockabilly (Radio)",
    "K.K. Safari (Radio)",
    "K.K. Salsa (Radio)",
    "K.K. Samba (Radio)",
    "K.K. Ska (Radio)",
    "K.K. Sonata (Radio)",
    "K.K. Song (Radio)",
    "K.K. Soul (Radio)",
    "K.K. Steppe (Radio)",
    "K.K. Stroll (Radio)",
    "K.K. Swing (Radio)",
    "K.K. Synth (Radio)",
    "K.K. Tango (Radio)",
    "K.K. Technopop (Radio)",
    "K.K. Waltz (Radio)",
    "K.K. Western (Radio)",
    "K.K. Etude (Radio)",
    "King K.K. (Radio)",
    "Lucky K.K. (Radio)",
    "Marine Song 2001 (Radio)",
    "Mountain Song (Radio)",
    "Mr. K.K. (Radio)",
    "My Place (Radio)",
    "Neapolitan (Radio)",
    "Only Me (Radio)",
    "Pondering (Radio)",
    "Rockin K.K. (Radio)",
    "Senor K.K. (Radio)",
    "Soulful K.K. (Radio)",
    "Space K.K. (Radio)",
    "Spring Blossoms (Radio)",
    "Stale Cupcakes (Radio)",
    "Steep Hill (Radio)",
    "Surfin K.K. (Radio)",
    "The K. Funk (Radio)",
    "To the Edge (Radio)",
    "Two Days Ago (Radio)",
    "Wandering (Radio)",
    "Welcome Horizons (Radio)",
];
#[derive(Resource, Reflect)]
pub struct kkwhichsongs(pub Vec<String>);
impl Default for kkwhichsongs {
    fn default() -> Self {
        kkwhichsongs(kkSongs.iter().map(|song| song.to_string()).collect())
    }
}
pub struct MusicImport;

impl Plugin for MusicImport {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadMusic>()
            .init_resource::<GrandfatherMode>()
            .init_resource::<kkwhichsongs>()
            .register_type::<kkwhichsongs>()
            .add_plugins(ResourceInspectorPlugin::<kkwhichsongs>::default().run_if(input_toggle_active(true, KeyCode::KeyO)))
            .init_resource::<SaturdayKkMode>()
            .init_resource::<TownTune>()
            .init_resource::<OfflineMode>()
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
    res_whichkk: Res<kkwhichsongs>,
    res_kk: Res<SaturdayKkMode>,
    mut env_load_mus: EventWriter<LoadMusic>,
    other_music: Query<Entity, With<IsMusic>>,
    // 0:minute 1:hour 2: game 3:bool to configure numbers at start
    mut debounce: bevy::prelude::Local<(u32, u32, bool)>,
) {
    if !(*debounce).2 {
        (*debounce).2 = true;
        (*debounce).0 = u32::MAX;
        (*debounce).1 = u32::MAX;
    }
    let thime = Local::now();
    // checks
    if (*debounce).0 == thime.minute() {
        if !game.is_changed() {
            debug!("failed:check 1 at {}", thime.minute());
            return;
        };
    };
    (*debounce).0 = thime.minute();
    debug!("passed:check 1 at {}", thime.minute());

    if (*debounce).1 == thime.hour() {
        if !game.is_changed() {
            debug!("failed:check 2 at {}", thime);
            return;
        }
    };
    (*debounce).1 = thime.hour();
    info!("passed:check 2 at {}", thime);

    // passed
    let (apm, hour) = thime.hour12();
    let apm_str = if apm { "pm" } else { "am" };
    // game manager
    let mut gametype = match *game {
        GameSelector::random => GameSelector::random_game().to_file_name().to_string(),
        _ => (*game.to_file_name()).to_string(),
    };
    // saturday kk night
    if res_kk.0 {
        if thime.weekday() == Weekday::Sat && /*day saturday and */ hour_to_pocket_camp(format!("{hour}{apm_str}")) == "night" {
             gametype = GameSelector::kk_slider.to_file_name()
        }
    }
    // pocket camp manager
    let time_str = match gametype.as_str() {
        "population-growing-rainy" => "12am".to_string(),
        "pocket-camp" => hour_to_pocket_camp(format!("{hour}{apm_str}")),
        "kk-slider-desktop" => {
            let mut rng = rand::rng();
            // sanitise strings with %20, file names with spaces are kinda stupidd
            if let Some(choice) = res_whichkk.0.choose(&mut rng) {
                choice.replace(" ", "%20")
            } else {
                kkSongs.choose(&mut rng).expect("the list is const").replace(" ", "%20")
            }
        }
        _ => format!("{hour}{apm_str}"),
    };
    // let time_str = if *game==GameSelector::random && gametype == "pocket-camp" || *game == GameSelector::pocket_camp  {
    //   hour_to_pocket_camp(format!("{hour}{apm_str}"))
    // } else if *game==Ga {}
    // else {
    //     format!("{hour}{apm_str}")
    // };

    // let time_str = match gametype.as_str() {
    //     "pocket-camp" => hour_to_pocket_camp(format!("{hour}{apm_str}")),
    //     _ => {
    //         format!("{hour}{apm_str}")
    //     }
    // };

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
                mode: if res_grandfather.0 {
                    PlaybackMode::Once
                } else {
                    PlaybackMode::Loop
                },
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
            let download_time_str = time_str.replace("%20", " ");
            let target1 =
                format!("https://d17orwheorv96d.cloudfront.net/{gametype}/{download_time_str}.ogg");
            // if gametype is kk-slider-desktop -> download file with fmpeg instead , stupid stupid ogg theora kk video files
            // whatever dumbass made the kk songs in ogg theora instead of ogg vorbis, i curse your stupid ass!
            // i was only halfways jonking, i learnt about ffmpeg atleast! ;-;
            if gametype == "kk-slider-desktop" {
                let path_str2 = format!("{path_str}/{time_str}.ogg");
                info!(path_str2);
                remux(
                    &CString::from_str(target1.as_str()).unwrap(),
                    &CString::from_str(path_str2.as_str()).unwrap(),
                )
                .unwrap()
            }
            // else, typical downloading
            else {
                let response = reqwest::get(target1).await.expect("oh noes file not found");

                if let Some(content_type) = response.headers().get(CONTENT_TYPE) {
                    info!("{:?}", content_type);
                    if content_type != "audio/ogg" {
                        panic!("Received a non ogg file(probs an xml dangit)!");
                    }
                }

                let mut dest = {
                    let fname = response
                        .url()
                        .path_segments()
                        .and_then(|segments| segments.last())
                        .and_then(|name| if name.is_empty() { None } else { Some(name) })
                        .expect("uh invalid url");

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
                        mode: if res_grandfather.0 {
                            PlaybackMode::Once
                        } else {
                            PlaybackMode::Loop
                        },
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

fn remux(input_path: &CStr, output_path: &CStr) -> Result<(), RsmpegError> {


    let mut input_format_context = AVFormatContextInput::open(input_path, None, &mut None)
        .expect("Create input format context failed.");
    input_format_context
        .dump(0, input_path)
        .expect("Dump input format context failed.");
    let mut output_format_context = AVFormatContextOutput::create(output_path, None)
        .expect("create ourput format context failed");
    let stream_mapping: Vec<_> = {
        let mut stream_index = 0usize;
        input_format_context
            .streams()
            .into_iter()
            .map(|stream| {
                let codec_type = stream.codecpar().codec_type();
                if !codec_type.is_audio() {
                    None
                } else {
                    output_format_context
                        .new_stream()
                        .set_codecpar(stream.codecpar().clone());
                    stream_index += 1;
                    Some(stream_index - 1)
                }
            })
            .collect()
    };
    output_format_context
        .dump(0, output_path)
        .expect("Dump output format context failed.");

    output_format_context
        .write_header(&mut None)
        .expect("Writer header failed.");

    while let Some(mut packet) = input_format_context
        .read_packet()
        .expect("Read packet failed.")
    {
        let input_stream_index = packet.stream_index as usize;
        let Some(output_stream_index) = stream_mapping[input_stream_index] else {
            continue;
        };
        {
            let input_stream = &input_format_context.streams()[input_stream_index];
            let output_stream = &output_format_context.streams()[output_stream_index];
       
            packet.rescale_ts(input_stream.time_base, output_stream.time_base);
            packet.set_stream_index(output_stream_index as i32);
            packet.set_pos(-1);
        }
        output_format_context
            .interleaved_write_frame(&mut packet)
            .expect("Interleaved write frame failed.");
    }
    output_format_context.write_trailer()
}

fn play_music(us_ev:EventReader<UserEvent>){
    // ctx.run_on_main_thread(move |ctx| {
    //     // The inner context gives access to a mutable Bevy World reference.
    //     let world: &mut World = ctx.world;
    //     let asset_server = world.resource::<AssetServer>();
    //     let res_music_vol = world.resource::<MusicVolume>();
    //     let res_playing = world.resource::<MusicPlaying>();
    //     let res_grandfather = world.resource::<GrandfatherMode>();

    //     world.spawn((
    //         IsMusic,
    //         AudioPlayer::new(asset_server.load(format!("music/{gametype}/{time_str}.ogg"))),
    //         PlaybackSettings {
    //             mode: if res_grandfather.0 {
    //                 PlaybackMode::Once
    //             } else {
    //                 PlaybackMode::Loop
    //             },
    //             volume: Volume::new(res_music_vol.0),
    //             paused: !res_playing.0,
    //             ..default()
    //         },
    //         Name::new("MusicPlayer"),
    //     ));
    // })
    // .await;
}