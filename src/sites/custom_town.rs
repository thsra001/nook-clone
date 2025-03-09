use bevy::prelude::*;
use crate::{i18evy::I18Key, widgets::{buttons::WideButton, textStyle::{h1, h2, h3}}, SiteHolder};
use super::{CurrentSite, SiteRoot};

#[derive(Resource)]
pub struct TownMode(pub bool);

pub struct customTownImport;

impl Plugin for customTownImport {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_customTown.run_if(if_load_customTown));
    }
}
fn if_load_customTown(q_site:Res<CurrentSite>) -> bool{
  q_site.is_changed() && *q_site == CurrentSite::CustomTown
}
fn load_customTown(mut commands:Commands,asset_server:ResMut<AssetServer>,q_holder:Single<Entity,With<SiteHolder>>,q_site:Res<CurrentSite>,q_other_site:Query<Entity,With<SiteRoot>>){
  for ent in &q_other_site{
    commands.entity(ent).despawn_recursive();
  }
  let site = commands.spawn((SiteRoot,Name::new("town-site"))).with_children(|site|{
    
    site.spawn((h1(&asset_server),Text::new("tune settings"),I18Key::TuneSettings));
    site.spawn(ImageNode::new(asset_server.load("images/tune_bg.png")));
    // mouse wheel tip
    site.spawn((
      Text::new("tip:you can use the mouse wheel to adjust notes!"),
      I18Key::TipYouCanUseTheMouseWheelToAdjustNotes,
      TextFont {
          font: asset_server.load("fonts/inter-lig.ttf"),
          font_size: 12.0,
          ..default()
      },
      TextColor(Color::srgb(170.0/255.0, 245.0/255.0, 89.9/255.0)),
  ));
  // wide
  site.spawn(Node{column_gap: Val::Px(10.0),
    justify_content: JustifyContent::Center,..default()}).with_children(|wide| {
    // | customise k.k playlist |
    wide.spawn((WideButton, ))
        .with_child((Text::new("save"),I18Key::SaveLovercase, h3(&asset_server),PickingBehavior::IGNORE,));
    // | customise town tune |
    wide.spawn((WideButton, ))
        .with_child((Text::new("play"),I18Key::PlayLowercase, h3(&asset_server),PickingBehavior::IGNORE));
});
  }).id();

  commands.entity(*q_holder).add_child(site);

}