use std::time::Duration;

use bevy::{prelude::*, ui::RelativeCursorPosition, window::WindowResolution, winit::{UpdateMode, WinitSettings}};
use bevy_inspector_egui::{egui::Slider, quick::WorldInspectorPlugin};
use bevy_tokio_tasks::TokioTasksRuntime;
use tray_icon::{
    menu::{self, AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIcon, TrayIconBuilder, TrayIconEvent, TrayIconEventReceiver,
};
mod colours;
use colours::*;
mod widgets;
use widgets::*;
mod interactive;
use gtk::prelude::*;
use interactive::topbar::PlayerButton;
use interactive::*;
mod sites;
use sites::*;
use winit::event_loop::EventLoop;

use crate::topbar::{CloseButton, MiniButton};
use crate::{
    game_selector::{GameSelector, GameSelectorText, GamesSelectorButton},
    interactive::slider::*,
};

#[derive(Component)]
struct SiteHolder;

enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window:Some(Window{
                resolution:WindowResolution::new(400.0, 216.0),
                ..default()
            }),
            ..default()
        }))
      //  .add_systems(PreStartup, load_persistant)
        .add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
        .add_plugins((SitesImport,WidgetImport,InterImport))
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource( WinitSettings {
            focused_mode: UpdateMode::reactive(Duration::from_secs(5)),
            unfocused_mode: UpdateMode::reactive_low_power(Duration::from_secs(15)),
        })
        .insert_resource(ClearColor(Color::Srgba(Srgba::hex("#16072b").unwrap())))
        .add_systems(Startup, build_ui)
        .add_systems(Startup, tray_icon)
        .run();
}

// bar #090410
// bg #17092c

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

fn build_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("hello nook");
    commands.spawn(Camera2d);
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colours::BACKGROUND_PURBLE),
            Name::new("window"),
            SiteHolder,
        ))
        .with_children(|nod| {
            nod.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.0),
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::horizontal(Val::Px(15.0)),
                    ..default()
                },
                BackgroundColor(colours::TOPBAR_NOIR),
                Name::new("topbar"),
            ))
            .with_children(|topbar| {
                topbar
                    .spawn(
                        (Node {
                            column_gap: Val::Px(15.0),
                            ..default()
                        }),
                    )
                    .with_children(|bollock1| {
                        //settings and return to home icon: aligned left  
                        bollock1.spawn((
                            Text::new(""),
                            TextFont {
                                font: asset_server.load("fonts/nerd-symbols-reg.ttf"),
                                font_size: 24.0,
                                ..default()
                            },
                            Button,
                        ));
                        //pause/play icon: aligned left  
                        bollock1.spawn((
                            Text::new(""),
                            TextFont {
                                font: asset_server.load("fonts/nerd-symbols-reg.ttf"),
                                font_size: 24.0,
                                ..default()
                            },
                            PlayerButton,
                            Button,
                        ));
                    });
                //title: aligned center
                topbar.spawn((
                    Node {
                        justify_self: JustifySelf::Center,
                        ..default()
                    },
                    Text::new("nook"),
                    TextFont {
                        font: asset_server.load("fonts/inter-reg.ttf"),
                        font_size: 18.0,
                        ..default()
                    },
                    Name::new("title"),
                ));
                topbar
                    .spawn(
                        (Node {
                            column_gap: Val::Px(15.0),
                            ..default()
                        }),
                    )
                    .with_children(|bollock2| {
                        // minimise icon: aligned right 
                        bollock2.spawn((
                            Node {
                                justify_self: JustifySelf::End,
                                ..default()
                            },
                            Text::new(""),
                            TextFont {
                                font: asset_server.load("fonts/nerd-symbols-reg.ttf"),
                                font_size: 24.0,
                                ..default()
                            },
                            MiniButton,
                            Button,
                        ));
                        // close icon: aligned right 
                        bollock2.spawn((
                            Node {
                                justify_self: JustifySelf::End,
                                ..default()
                            },
                            Text::new(""),
                            TextFont {
                                font: asset_server.load("fonts/nerd-symbols-reg.ttf"),
                                font_size: 24.0,
                                ..default()
                            },
                            CloseButton,
                            Button,
                        ));
                    });
            });
        });
}
// fn load_persistant(mut commands: Commands) {
//     let config_dir = dirs::config_dir().unwrap().join("nook-linux");
//     commands.insert_resource(
//         Persistent::<GameSelector>::builder()
//             .name("games selector")
//             .format(StorageFormat::Toml)
//             .path(config_dir.join("save.toml"))
//             .default(GameSelector::new_horizons)
//             .build()
//             .expect("failed to init gameselector")
//     )
// }