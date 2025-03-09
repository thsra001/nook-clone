use bevy::prelude::*;

use super::{CurrentSite, SiteRoot};
use crate::i18evy::I18Key;
use crate::widgets::buttons::{KkButton, TownButton, WideButton};
use crate::widgets::textStyle::{h1, h2, h3};
use crate::widgets::tickbox::Tickbox;
use crate::{colours, SiteHolder};
pub struct SettingsImport;

impl Plugin for SettingsImport {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_settings.run_if(if_load_settings));
    }
}
fn if_load_settings(q_site: Res<CurrentSite>) -> bool {
    q_site.is_changed() && *q_site == CurrentSite::Settings
}
fn load_settings(
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
        .spawn((SiteRoot, Name::new("settings-site")))
        .with_children(|settings_site| {
            // player settings big text
            settings_site.spawn((Text::new("player settings"),I18Key::PlayerSettings, h1(&asset_server)));
            // switchbox with tickboxes
            settings_site
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|player_box| {
                    // [] grandfather clock mode (plays once, no loop)
                    player_box
                        .spawn(Node {
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|wide| {
                            wide.spawn((Name::new("GrandfatherTickbox"), Tickbox::Grandfather));
                            wide.spawn((
                                Name::new("GrandFatherText"),
                                Text::new("Grandfather clock mode"),
                                I18Key::GrandfatherClockMode,
                                h2(&asset_server),
                            ));
                            wide.spawn((
                                Name::new("GrandFatherSubtext"),
                                Text::new("(plays once, doesn't loop)"),
                                I18Key::PlaysOnceNoLoop,
                                h3(&asset_server),
                                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                            ));
                        });
                    // [] use game rain sound
                    player_box
                        .spawn(Node {
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|wide| {
                            wide.spawn((Name::new("RainTickbox"), Tickbox::Rain));
                            wide.spawn((
                                Name::new("RainText"),
                                Text::new("Use game rain/no-rain sound"),
                                // todo: i18key is inaccurate
                                I18Key::UseGameRainSound,
                                h2(&asset_server),
                            ));
                        });
                    // [] don't download music (saves space, but no offline)
                    player_box
                        .spawn(Node {
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|wide| {
                            wide.spawn((Name::new("OfflineTickbox"), Tickbox::DontDownload));
                            wide.spawn((Text::new("Don't download music"),I18Key::DontDownloadMusic, h2(&asset_server)));
                            wide.spawn((
                                Name::new("OfflineSubtext"),
                                Text::new("(saves space, but no offline)"),
                                I18Key::SavesSpaceButNoOffline,
                                h3(&asset_server),
                                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                            ));
                        });
                    // [] enable town tune
                    player_box
                        .spawn(Node {
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|wide| {
                            wide.spawn((Name::new("TownTickbox"), Tickbox::TownTune));
                            wide.spawn((
                                Name::new("TownText"),
                                Text::new("Enable town tune"),
                                I18Key::EnableTownTune,
                                h2(&asset_server),
                            ));
                        });
                    // [] play K.K. music on Saturday nights
                    player_box
                        .spawn(Node {
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|wide| {
                            wide.spawn((Name::new("KKTickbox"), Tickbox::PlayKk));
                            wide.spawn((
                                Name::new("KKText"),
                                Text::new("Play K.K. music on Saturday nights"),
                                I18Key::PlayKkMusicOnSaturdayNights,
                                h2(&asset_server),
                            ));
                        });
                    // [] open on startup
                    player_box
                        .spawn(Node {
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|wide| {
                            wide.spawn((Name::new("StartupTickbox"), Tickbox::Startup));
                            wide.spawn((Text::new("Open on startup"),I18Key::OpenOnStartup, h2(&asset_server)));
                        });
                });
            // two wide buttons
            settings_site
                .spawn(Node {
                    column_gap: Val::Px(10.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|wide| {
                    // | customise k.k playlist |
                    wide.spawn((WideButton, KkButton))
                        .with_child((Text::new("customise k.k. playlist"),I18Key::CustomizeKKPlaylist, h3(&asset_server),PickingBehavior::IGNORE,));
                    // | customise town tune |
                    wide.spawn((WideButton, TownButton))
                        .with_child((Text::new("customise town tune"),I18Key::CustomizeTownTune, h3(&asset_server),PickingBehavior::IGNORE));
                });
            // language big text
            settings_site.spawn((Text::new("language"),I18Key::Language, h1(&asset_server)));

            // | language chooser dropdown |

            // offline big text
            settings_site.spawn((Text::new("offline"),I18Key::Offline, h1(&asset_server)));

            // {} offline hourly music files downloaded
            settings_site.spawn((
                Text::new("dont/know hourly music files downladed"),
                I18Key::OfflineFilesTotalFilesOfflineHourlyMusicFilesDownloaded,
                TextFont {
                    font: asset_server.load("fonts/inter-lig.ttf"),
                    font_size: 12.0,
                    ..default()
                },
            ));
            // {} offline kk music files downloaded
            settings_site.spawn((
                Text::new("fuck/you offline k.k. music files downloaded"),
                I18Key::OfflineKkfilesTotalKkfilesOfflineKKMusicFilesDownloaded,
                TextFont {
                    font: asset_server.load("fonts/inter-lig.ttf"),
                    font_size: 12.0,
                    ..default()
                },
            ));

             // two wide buttons
             settings_site
             .spawn(Node {
                 column_gap: Val::Px(10.0),
                 justify_content: JustifyContent::Center,
                 ..default()
             })
             .with_children(|wide| {
                 // | customise k.k playlist |
                 wide.spawn((WideButton, ))
                     .with_child((Text::new("download all hourly music"),I18Key::DownloadAllHourlyMusic, h3(&asset_server),PickingBehavior::IGNORE,));
                 // | customise town tune |
                 wide.spawn((WideButton, ))
                     .with_child((Text::new("download all k.k. music"),I18Key::DownloadAllKKMusic, h3(&asset_server),PickingBehavior::IGNORE));
             });
            // | download all hourly music |
            // | download all k.k music |
            // clear local files and settings red text
            settings_site.spawn((
                Text::new("clear local files and settings"),
                I18Key::ClearLocalFilesAndSettings,
                TextFont {
                    font: asset_server.load("fonts/inter-lig.ttf"),
                    font_size: 10.0,
                    ..default()
                },
                TextColor(colours::REMOVAL_RED),
            ));
        })
        .id();
    commands.entity(*q_holder).add_child(site);
}
