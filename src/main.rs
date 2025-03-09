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
use widgets::{buttons::SettingsButton, *};
mod interactive;
use gtk::prelude::*;
use interactive::topbar::PlayerButton;
use interactive::*;
mod sites;
use sites::*;
use winit::event_loop::EventLoop;
mod tray2 ;
use tray2::*;

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
        .add_plugins(TrayImport)
        .add_plugins((SitesImport,WidgetImport,InterImport))
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource( WinitSettings {
            focused_mode: UpdateMode::reactive(Duration::from_secs(5)),
            unfocused_mode: UpdateMode::reactive_low_power(Duration::from_secs(15)),
        })
        .insert_resource(ClearColor(Color::Srgba(Srgba::hex("#16072b").unwrap())))
        .add_systems(Startup, build_ui)
        .run();
}

// bar #090410
// bg #17092c



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
                overflow:Overflow::scroll_y(),
                ..default()
            },
            BackgroundColor(colours::BACKGROUND_PURBLE),
            Name::new("uiRoot"),
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
                            SettingsButton,
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