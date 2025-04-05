use bevy::{input::{common_conditions::input_toggle_active, mouse::{MouseScrollUnit, MouseWheel}}, picking::focus::{HoverMap, PickingInteraction}, prelude::*, window::PrimaryWindow};
mod start;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use start::*;
mod settings;
use settings::*;
mod custom_kk;
use custom_kk::*;
mod custom_town;
use custom_town::*;
use tray_icon::{menu::MenuEvent, TrayIconEvent};

use crate::interactive::ButtonReflectSet;

#[derive(Component,Default)]
pub struct ProperHeight;

#[derive(Resource, Default, Reflect, PartialEq)]
pub enum CurrentSite {
    #[default]
    Start,
    Settings,
    CustomKK,
    CustomTown,
}
#[derive(Component)]
#[require(ProperHeight,Node(||{Node{
    width:Val::Percent(100.0),
    flex_direction: FlexDirection::Column,
    align_items:AlignItems::Center,
    row_gap: Val::Px(20.0),
    padding: UiRect::top(Val::Px(18.0)),
    overflow: Overflow::scroll_y(),
    ..default()
}}))]
pub struct SiteRoot;

pub struct SitesImport;

impl Plugin for SitesImport {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentSite>()
            .register_type::<CurrentSite>()
            .add_plugins(ResourceInspectorPlugin::<CurrentSite>::default().run_if(input_toggle_active(true, KeyCode::KeyO)))
            .add_systems(Update, (proper_height.after(ButtonReflectSet),proper_height_scroll).chain())
            .add_plugins((
                StartImport,
                SettingsImport,
                customKKImport,
                customTownImport,
            ));
    }
}

fn proper_height(
    mut q_siteholder: Query<&mut Node,With<ProperHeight>>,
    window: Query<&Window,With<PrimaryWindow>>,

){
  if let Ok(win) = &window.get_single(){
    for mut sitehol in &mut q_siteholder{
        sitehol.max_height = Val::Px(win.resolution.physical_height() as f32 - 50.0)
    }
  }
}
fn proper_height_scroll(
    // a propheig and scrollpos ent
    mut q_node:Query<(&mut ScrollPosition,&PickingInteraction), With<ProperHeight>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
){
   for mouse_ev in mouse_wheel_events.read(){
    let mut y = match mouse_ev.unit {
        MouseScrollUnit::Line => mouse_ev.y * 30.0,
        MouseScrollUnit::Pixel => mouse_ev.y
    };
    for (mut scrollpos,pickinter) in &mut q_node{
       if *pickinter != PickingInteraction::None {
           scrollpos.offset_y -= y;
       }
    }
   }
}
