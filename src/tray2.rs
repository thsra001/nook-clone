use std::{sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
}, thread::sleep, time::Duration};

use bevy::{ecs::query, prelude::*, transform::commands, window::PrimaryWindow, winit::{EventLoopProxy, EventLoopProxyWrapper}};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuId, MenuItem},
    TrayIconBuilder, TrayIconEvent,
};

use crate::{async_handler::{AppExtensions, ChannelSender}, player::MusicPlaying};
#[derive(Debug)]
pub enum PlayAudio {
    Music(String),
    Rain(String)
}
#[derive(Event,Debug,Default)]
pub enum UserEvent {
    #[default]
    Wakeup,
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
    PlayAudio(PlayAudio),
}

pub struct TrayImport;

impl Plugin for TrayImport {
    fn build(&self, app: &mut App) {
        
        let (sdr,rcr) = mpsc::channel::<UserEvent>();
         app.add_event_channel(rcr)//.init_resource::<TrayChannel>()
           .insert_resource(ChannelSender(sdr))
            .add_systems(Startup, tray_icon)
             .add_systems(Update, read_tray_icon);
    }
}

fn tray_icon(mut commands: Commands, mut res_channel: Res<ChannelSender<UserEvent>>,event_loop_proxy:Res<EventLoopProxyWrapper<UserEvent>>) {
    let event_loop_proxy = event_loop_proxy.clone();
    let event_loop_proxy2 = event_loop_proxy.clone();
    let event_loop_proxy3 = event_loop_proxy.clone();
    let tx = res_channel.0.clone();
    let tx2 = tx.clone();
    TrayIconEvent::set_event_handler(Some(move |event:TrayIconEvent| {
        event_loop_proxy.send_event(UserEvent::TrayIconEvent(event.clone())).unwrap();
        sleep(Duration::from_secs_f32(0.2));
        event_loop_proxy.send_event(UserEvent::Wakeup).unwrap();
        //tx2.send(UserEvent::TrayIconEvent(event)).unwrap();
    }));
    MenuEvent::set_event_handler(Some(move |event:MenuEvent| {
        event_loop_proxy2.send_event(UserEvent::MenuEvent(event.clone())).unwrap();
        sleep(Duration::from_secs_f32(0.2));
        event_loop_proxy2.send_event(UserEvent::Wakeup).unwrap();
        //tx.send(UserEvent::MenuEvent(event)).unwrap();
    }));
    // todo: remove, debugging
    // std::thread::spawn(move ||{
    //     loop {
    //         sleep(Duration::from_secs(15));
    //         info!("wakeup");
    //         event_loop_proxy3.send_event(UserEvent::MenuEvent(MenuEvent { id: MenuId("play-pause".to_string()) })).unwrap();

    //     }
    // });
    std::thread::spawn(|| {
        gtk::init().unwrap();
        let path = "assets/images/nookTray.png";
        info!(path);
        let icon = load_icon(std::path::Path::new(path));

        let item1 = MenuItem::with_id("open", "open", true, None);
        let item2 = MenuItem::with_id("play-pause", "play/pause", true, None);
        let item3 = MenuItem::with_id("quit", "quit", true, None);

        let tray_menu = Menu::with_items(&[&item1, &item2, &item3]).unwrap();

        let _tray_icon = TrayIconBuilder::new()
            .with_menu_on_left_click(false)
            .with_menu(Box::new(tray_menu))
            .with_tooltip("nook-linux")
            .with_icon(icon)
            .build()
            .unwrap();

        gtk::main();
    });
    info!("made the icon :D")
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

fn read_tray_icon(mut commands: Commands,mut res_channel: EventReader<UserEvent>,mut exit: EventWriter<AppExit>,mut res_playing: ResMut<MusicPlaying>,iswindow:Query<&Window,With<PrimaryWindow>>,) {
    for recived in res_channel.read(){
        match recived {
            UserEvent::MenuEvent(ev) => {
                info!("{:?}",ev);
                match ev.id.0.as_str() {
                    "open" => {
                        if let Ok(bob) = iswindow.get_single()  {
                            info!("wait theres a window");
                            return;
                        }
                        commands.spawn((Window{title:String::from("nook"),..default()},PrimaryWindow));
                    },
                    "play-pause" => { *&mut res_playing.0=!*&mut res_playing.0;
              },
                    "quit" => {exit.send(AppExit::Success);},
                    _ => panic!("wtf is not possible")
                };
            },
            UserEvent::TrayIconEvent(ev) => todo!(), 
            _ => return
        } 
    }
   
}
