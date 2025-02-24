use bevy::{prelude::*, transform::commands};

use crate::{colours::{self, SELECTOR_PURBLE2}, game_selector::{GameSelector, GameSelectorText, GamesSelectorButton}, slider::{Slider, SliderHead, SliderRes}, SiteHolder};

use super::{CurrentSite, SiteRoot};

pub struct StartImport;

impl Plugin for StartImport {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_start.run_if(if_load_start));
    }
}
fn if_load_start(q_site: Res<CurrentSite>) -> bool {
    q_site.is_changed() && *q_site == CurrentSite::Start
}
fn load_start(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    q_holder: Single<Entity, With<SiteHolder>>,
    q_site: Res<CurrentSite>,
    q_other_site: Query<Entity, With<SiteRoot>>,
) {
    for ent in &q_other_site {
        commands.entity(ent).despawn_recursive();
    }
    let site = commands
        .spawn((
            SiteRoot,
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                padding: UiRect::top(Val::Px(18.0)),
                ..default()
            },
            Name::new("start-site"),
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
        })
        .id();

    commands.entity(*q_holder).add_child(site);
}
