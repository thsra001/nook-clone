use bevy::prelude::*;
mod start;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use start::*;
mod settings;
use settings::*;
mod custom_kk;
use custom_kk::*;
mod custom_town;
use custom_town::*;

#[derive(Resource, Default, Reflect, PartialEq)]
pub enum CurrentSite {
    #[default]
    Start,
    Settings,
    CustomKK,
    CustomTown,
}
#[derive(Component)]
#[require(Node(||{Node{
    width:Val::Percent(100.0),
    flex_direction: FlexDirection::Column,
    align_items:AlignItems::Center,
    row_gap: Val::Px(20.0),
    padding: UiRect::top(Val::Px(18.0)),
    ..default()
}}))]
pub struct SiteRoot;

pub struct SitesImport;

impl Plugin for SitesImport {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentSite>()
            .register_type::<CurrentSite>()
            .add_plugins(ResourceInspectorPlugin::<CurrentSite>::default())
            .add_plugins((
                StartImport,
                SettingsImport,
                customKKImport,
                customTownImport,
            ));
    }
}
