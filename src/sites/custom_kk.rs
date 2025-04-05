use crate::music::{kkSongs, kkwhichsongs};
use crate::textStyle::*;
use crate::widgets::buttons::Kkbuttons;
use crate::widgets::tickbox::Tickbox;
use crate::{
    i18evy::I18Key,
    widgets::{
        buttons::WideButton,
        drop_down::{DropDown, DropDownRes},
    },
    SiteHolder,
};
use bevy::prelude::*;

use super::{CurrentSite, SiteRoot};

pub struct customKKImport;

impl Plugin for customKKImport {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_customKK.run_if(if_load_customKK));
    }
}
fn if_load_customKK(q_site: Res<CurrentSite>) -> bool {
    q_site.is_changed() && *q_site == CurrentSite::CustomKK
}
fn load_customKK(
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
        .spawn((SiteRoot))
        .with_children(|site| {
            //   site.spawn((DropDown,DropDownRes::Lang));
            // check all | uncheck all | radio only | live only
            site.spawn(Node {
                column_gap: Val::Px(10.0),
                justify_content: JustifyContent::Center,
                ..default()
            })
            .with_children(|wide| {
                // | check all |
                wide.spawn((Kkbuttons::CheckAll)).with_child((
                    Text::new("customise k.k. playlist"),
                    I18Key::CheckAll,
                    h3(&asset_server),
                    PickingBehavior::IGNORE,
                ));
                // | uncheck all |
                wide.spawn((Kkbuttons::UncheckAll)).with_child((
                    Text::new("customise town tune"),
                    I18Key::UncheckAll,
                    h3(&asset_server),
                    PickingBehavior::IGNORE,
                ));
                // radio only
                wide.spawn((Kkbuttons::RadioOnly)).with_child((
                    Text::new("customise k.k. playlist"),
                    I18Key::RadioOnly,
                    h3(&asset_server),
                    PickingBehavior::IGNORE,
                ));
                // live only
                wide.spawn((Kkbuttons::LiveOnly)).with_child((
                    Text::new("customise k.k. playlist"),
                    I18Key::LiveOnly,
                    h3(&asset_server),
                    PickingBehavior::IGNORE,
                ));
            });
            // two rows with songs and checkmarks
            site.spawn(Node {
                column_gap: Val::Px(10.0),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,

                ..default()
            })
            .with_children(|tworows| {
                // row with radio songs
                let mut radio = kkSongs
                        .iter()
                        .map(|song| song.to_string())
                        .collect::<Vec<String>>();
                    radio.retain(|song| song.contains("(Radio)"));
                tworows
                    .spawn(Node {
                        column_gap: Val::Px(10.0),
                        flex_direction: FlexDirection::Column,
                        align_items:AlignItems::Start,
                        ..default()
                    })
                    .with_children(|dlist| {
                        for song in radio.iter() {
                            dlist
                                .spawn(Node {
                                    column_gap: Val::Px(10.0),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                })
                                .with_children(|dlist2| {
                                    dlist2.spawn((Tickbox::Kksong(song.to_string())));
                                    dlist2.spawn((
                                        Node { ..default() },
                                        h2(&asset_server),
                                        Text::new(song.to_string()),
                                    ));
                                });
                        }
                    });
                // row with live songs
                let mut live = kkSongs
                        .iter()
                        .map(|song| song.to_string())
                        .collect::<Vec<String>>();
                    live.retain(|song| !song.contains("(Radio)"));
                  
                tworows
                    .spawn(Node {
                        column_gap: Val::Px(10.0),
                        flex_direction: FlexDirection::Column,
                        align_items:AlignItems::Start,

                        ..default()
                    })
                    .with_children(|dlist| {
                        for song in live.iter() {
                            dlist
                                .spawn(Node {
                                    column_gap: Val::Px(10.0),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                })
                                .with_children(|dlist2| {
                                    dlist2.spawn(Tickbox::Kksong(song.to_string()));
                                    dlist2.spawn((
                                        Node { ..default() },
                                        h2(&asset_server),
                                        Text::new(song.to_string()),
                                    ));
                                });
                        }
                    });
            });
        })
        .id();
    commands.entity(*q_holder).add_child(site);
}
