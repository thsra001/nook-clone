use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use tray_icon::{menu::{Menu, MenuItem}, TrayIconBuilder, TrayIconEvent};
use winit::event_loop::EventLoop;

use crate::UserEvent;
pub struct TrayImport;

impl Plugin for TrayImport {
    fn build(&self, app: &mut App) {
     app.add_systems(Startup, tray_icon)
     .add_systems(Update, read_tray_icon);
    }}

    fn tray_icon(runtime: ResMut<TokioTasksRuntime>) {
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
    
            let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
    
            let proxy = event_loop.create_proxy();
            tray_icon::TrayIconEvent::set_event_handler(Some(move |event| {
                proxy.send_event(UserEvent::TrayIconEvent(event));
            }));
    
            let proxy = event_loop.create_proxy();
            tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
                proxy.send_event(UserEvent::MenuEvent(event));
            }));
        });
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

    fn read_tray_icon(){
        if let Ok(event) = TrayIconEvent::receiver().try_recv() {
            println!("{:?}", event);
        }
    }