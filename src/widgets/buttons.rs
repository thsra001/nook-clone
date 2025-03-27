use bevy::{picking::focus::PickingInteraction, prelude::*};

use crate::{colours, interactive::{ButtonReflectSet, ButtonSet}, sites::CurrentSite};

// wide buttons w:22 h:132
#[derive(Component)]
#[require(Node(||{Node{min_width:Val::Px(152.0),min_height:Val::Px(32.0),align_items:AlignItems::Center,justify_content:JustifyContent::Center,..default()}}))]
#[require(BackgroundColor(||{BackgroundColor(colours::WIDE_N_LAME)}))]
#[require(BorderRadius(||{BorderRadius::all(Val::Px(6.0))}))]
#[require(Button, PickingInteraction)]
pub struct WideButton;
pub struct ButtonImport;

impl Plugin for ButtonImport {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update,(setting_button,custom_kk_button,custom_town_button).in_set(ButtonSet))
        .add_systems(Update, (reflect_setting_button).in_set(ButtonReflectSet));
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
