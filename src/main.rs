use bevy::{prelude::*, ui::RelativeCursorPosition, winit::WinitSettings};
use bevy_inspector_egui::{egui::Slider, quick::WorldInspectorPlugin};
use tray_icon::{
    menu::{self, AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIcon, TrayIconBuilder, TrayIconEvent, TrayIconEventReceiver,
};
mod colours;
use colours::*;
mod interactive;
use gtk::prelude::*;
use interactive::topbar::PlayerButton;
use interactive::*;
use winit::event_loop::EventLoop;

use crate::topbar::{CloseButton, MiniButton};
use crate::{
    game_selector::{GameSelector, GameSelectorText, GamesSelectorButton},
    interactive::slider::*,
};

#[derive(Component)]
struct ValSub(f32);

enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InterImport)
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(ClearColor(Color::Srgba(Srgba::hex("#16072b").unwrap())))
        .add_systems(Startup, build_ui)
        .add_systems(Startup, tray_icon)
        .run();
}

// bar #090410
// bg #17092c

fn tray_icon(asset_server: Res<AssetServer>) {
    let _img: Handle<Image> = asset_server.load("nook.png");
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
            BackgroundColor(Color::Srgba(Srgba::hex("#17092c").unwrap())),
            Name::new("window"),
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
            nod.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    padding: UiRect::top(Val::Px(18.0)),
                    ..default()
                },
                Name::new("body"),
            ))
            .with_children(|body| {
                //music,rain sliders
                body.spawn((
                    Node {
                        column_gap: Val::Px(55.0),
                        ..default()
                    },
                    Name::new("musicBollock"),
                ))
                .with_children(|mbollock| {
                    // music slider
                    mbollock
                        .spawn((Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },))
                        .with_children(|mbollock2| {
                            mbollock2.spawn((
                                Text::new("Music Volume"),
                                TextFont {
                                    font: asset_server.load("fonts/inter-lig.ttf"),
                                    font_size: 18.0,
                                    ..default()
                                },
                            ));
                            mbollock2
                                .spawn((
                                    Node {
                                        height: Val::Px(16.0),
                                        width: Val::Px(128.0),
                                        ..default()
                                    },
                                    BackgroundColor(colours::SLIDER_BLUE),
                                    Slider,
                                    SliderRes::MusVol,
                                ))
                                .with_child((
                                    Node {
                                        height: Val::Px(16.0),
                                        width: Val::Px(16.0),
                                        ..default()
                                    },
                                    BackgroundColor(colours::SLIDER_HEAD_TEAL),
                                    SliderHead,
                                    SliderRes::MusVol,
                                    PickingBehavior {
                                        should_block_lower: false,
                                        is_hoverable: false,
                                    },
                                ));
                        });
                    // rain volume
                    mbollock
                        .spawn((Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },))
                        .with_children(|rbollock2| {
                            rbollock2.spawn((
                                Text::new("Rain Volume"),
                                TextFont {
                                    font: asset_server.load("fonts/inter-lig.ttf"),
                                    font_size: 18.0,
                                    ..default()
                                },
                            ));
                            // slider min left:0, max left 116
                            // transalate to volume: 0-2
                            rbollock2
                                .spawn((
                                    Node {
                                        height: Val::Px(16.0),
                                        width: Val::Px(128.0),
                                        ..default()
                                    },
                                    BackgroundColor(colours::SLIDER_BLUE),
                                    Slider,
                                    SliderRes::RainVol,
                                ))
                                .with_child((
                                    Node {
                                        height: Val::Px(16.0),
                                        width: Val::Px(16.0),
                                        ..default()
                                    },
                                    BackgroundColor(colours::SLIDER_HEAD_TEAL),
                                    SliderHead,
                                    SliderRes::RainVol,
                                    PickingBehavior {
                                        should_block_lower: false,
                                        is_hoverable: false,
                                    },
                                ));
                        });
                });
                //game selector
                body.spawn((
                    Node {
                        width: Val::Px(312.0),
                        height: Val::Px(42.0),
                        align_items: AlignItems::Center,
                        padding: UiRect::left(Val::Px(12.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    BackgroundColor(colours::SELECTOR_PURBLE),
                    Name::new("gameSelectorBollock"),
                    GamesSelectorButton,
                    Button,
                ))
                .with_children(|selector| {
                    // current choice
                    selector.spawn((
                        Text::new(GameSelector::population_growing.to_display_name()),
                        TextFont {
                            font: asset_server.load("fonts/inter-lig.ttf"),
                            font_size: 14.0,
                            ..default()
                        },
                        GameSelectorText,
                        PickingBehavior {
                            should_block_lower: false,
                            is_hoverable: false,
                        },
                    ));
                    // drop down
                    selector
                        .spawn((
                            Node {
                                width: Val::Px(42.0),
                                height: Val::Px(42.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(SELECTOR_PURBLE2),
                            PickingBehavior {
                                should_block_lower: false,
                                is_hoverable: false,
                            },
                        ))
                        .with_child((
                            Text::new(""),
                            TextFont {
                                font: asset_server.load("fonts/nerd-symbols-reg.ttf"),
                                font_size: 24.0,
                                ..default()
                            },
                            PickingBehavior {
                                should_block_lower: false,
                                is_hoverable: false,
                            },
                        ));
                });
                //patreon & changelog
                body.spawn((
                    Node {
                        column_gap: Val::Px(20.0),
                        ..default()
                    },
                    Name::new("linkBollock"),
                ))
                .with_children(|lbollock| {
                    lbollock.spawn((
                        Text::new("patreon"),
                        TextFont {
                            font: asset_server.load("fonts/inter-reg.ttf"),
                            font_size: 10.0,
                            ..default()
                        },
                    ));
                    lbollock.spawn((
                        Text::new("changelog"),
                        TextFont {
                            font: asset_server.load("fonts/inter-reg.ttf"),
                            font_size: 10.0,
                            ..default()
                        },
                    ));
                });
            });
        });
}
