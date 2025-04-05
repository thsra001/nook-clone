use bevy::{picking::focus::PickingInteraction, prelude::*};

use crate::{
    colours,
    interactive::{ButtonReflectSet, ButtonSet},
    music::{kkSongs, kkwhichsongs},
    sites::CurrentSite,
};

// wide buttons w:22 h:132
#[derive(Component, Default)]
#[require(Node(||{Node{min_width:Val::Px(152.0),min_height:Val::Px(32.0),align_items:AlignItems::Center,justify_content:JustifyContent::Center,..default()}}))]
#[require(BackgroundColor(||{BackgroundColor(colours::WIDE_N_LAME)}))]
#[require(BorderRadius(||{BorderRadius::all(Val::Px(6.0))}))]
#[require(Button, PickingInteraction)]
pub struct WideButton;
pub struct ButtonImport;

impl Plugin for ButtonImport {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                setting_button,
                custom_kk_button,
                custom_town_button,
                patreon_button,
            )
                .in_set(ButtonSet),
        )
        .add_systems(Update, (reflect_setting_button,custom_kk_buttons).in_set(ButtonReflectSet));
    }
}

// settings button:
// home: show settings icon and url to settings site
// other: show home icon and url to home
#[derive(Component)]
#[require(Button, PickingInteraction)]
pub struct SettingsButton;

fn setting_button(
    mut res_current_site: ResMut<CurrentSite>,
    inter: Single<&PickingInteraction, With<SettingsButton>>,
) {
    // match current site
    if **inter == PickingInteraction::Pressed {
        match *res_current_site {
            CurrentSite::Settings => *res_current_site = CurrentSite::Start,
            _ => *res_current_site = CurrentSite::Settings,
        }
    }
}
fn reflect_setting_button(
    mut res_current_site: ResMut<CurrentSite>,
    mut tex: Single<&mut Text, With<SettingsButton>>,
) {
    if res_current_site.is_changed() {
        // match current site
        match *res_current_site {
            CurrentSite::Settings => tex.0 = "".to_string(),
            _ => tex.0 = "".to_string(),
        }
    }
}
// pause button:
// toggle paused resource

// nook-linux button:
//  right click:
// (nook) -> nook github
// (linux) -> nook-linux github

// minimise button

// close button: -> appexit event

// patreon button: -> nook patreon

// customise k.k. playlist -> goto kk site
#[derive(Component)]
pub struct KkButton;

fn custom_kk_button(
    mut res_current_site: ResMut<CurrentSite>,
    inter: Option<Single<&PickingInteraction, With<KkButton>>>,
) {
    // match current site
    if let Some(inter_real) = inter {
        if **inter_real == PickingInteraction::Pressed {
            *res_current_site = CurrentSite::CustomKK
        }
    }
}

// customise town tune -> goto town site

#[derive(Component)]
pub struct TownButton;

fn custom_town_button(
    mut res_current_site: ResMut<CurrentSite>,
    inter: Option<Single<&PickingInteraction, With<TownButton>>>,
) {
    // match current site
    if let Some(inter_real) = inter {
        if **inter_real == PickingInteraction::Pressed {
            *res_current_site = CurrentSite::CustomTown
        }
    }
}

// pateron site
#[derive(Component)]
pub struct PatreonButton;

fn patreon_button(
    inter: Option<Single<&PickingInteraction, With<PatreonButton>>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if let Some(inter_real) = inter {
        if **inter_real == PickingInteraction::Pressed {
            // link to patreon
            if let Err(_) = open::that_detached("https://www.patreon.com/mattu") {
                error!("link open failed")
            };
        }
    }
}
// buttons on customise kk site
#[derive(Component)]
#[require(WideButton)]
pub enum Kkbuttons {
    CheckAll,
    UncheckAll,
    RadioOnly,
    LiveOnly,
}
fn custom_kk_buttons(
    inter: Query<(&PickingInteraction, &Kkbuttons)>,
    mut res_whichkk: ResMut<kkwhichsongs>,
) {
    for (inter, button) in &inter {
        if *inter == PickingInteraction::Pressed {
            // match what enum val to action
            match button {
                Kkbuttons::CheckAll => {
                    res_whichkk.0 = kkSongs.iter().map(|song| song.to_string()).collect()
                }
                Kkbuttons::UncheckAll => res_whichkk.0.clear(),
                Kkbuttons::RadioOnly => {
                    res_whichkk.0 = {
                        let mut bob = kkSongs
                            .iter()
                            .map(|song| song.to_string())
                            .collect::<Vec<String>>();
                        bob.retain(|song| song.contains("(Radio)"));
                        bob
                    }
                }
                Kkbuttons::LiveOnly =>  res_whichkk.0 = {
                    let mut bob = kkSongs
                        .iter()
                        .map(|song| song.to_string())
                        .collect::<Vec<String>>();
                    bob.retain(|song| !song.contains("(Radio)"));
                    bob
                },
            }
        }
    }
}
