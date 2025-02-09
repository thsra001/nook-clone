use bevy::{color, prelude::*, winit::WinitSettings};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Component)]
struct ValSub(f32);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(ClearColor(Color::Srgba(Srgba::hex("#16072b").unwrap())))
        .add_systems(Startup, build_ui)
        .run();
}


// bar #090410
// bg #17092c
//
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
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor(Color::srgb(0.2, 0.3, 0.4)),
            BackgroundColor(Color::Srgba(Srgba::hex("#17092c").unwrap())),
            Name::new("window"),
        ))
        .with_children(|nod| {
            nod.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    border: UiRect::all(Val::Px(2.0)),
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.0),
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::horizontal(Val::Px(15.0)) ,
                    ..default()
                },
                BorderColor(Color::srgb(0.1, 0.8, 0.3)),
                BackgroundColor(Color::Srgba(Srgba::hex("#07020e").unwrap())),
                Name::new("topbar"),
            ))
            .with_children(|topbar| {
                topbar
                    .spawn((Node {column_gap: Val::Px(10.0), ..default() }))
                    .with_children(|bollock1| {
                        //settings icon: aligned left 
                        bollock1.spawn((
                            Text::new(""),
                            TextFont {
                                font: asset_server.load("fonts/nerd-symbols-reg.ttf"),
                                font_size: 24.0,
                                ..default()
                            },
                        ));
                        //pause/play icon: aligned left  
                        bollock1.spawn((
                            Text::new(""),
                            TextFont {
                                font: asset_server.load("fonts/nerd-symbols-reg.ttf"),
                                font_size: 24.0,
                                ..default()
                            },
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
                topbar.spawn((Node{column_gap: Val::Px(10.0), ..default()})).with_children(|bollock2| {
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
                    ));
                });
            });
            nod.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BorderColor(Color::WHITE),
                Name::new("body"),
            ))
            .with_children(|body| {
                body.spawn((
                    Text::new("peneer"),
                    TextFont {
                        font: asset_server.load("Inter/static/Inter_18pt-Regular.ttf"),
                        ..default()
                    },
                ));
                //music sliders

                //game selector

                //patreon & changelog
            });
        });
}
