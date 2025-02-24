use bevy::prelude::*;

use crate::SiteHolder;

use super::{CurrentSite, SiteRoot};

pub struct customKKImport;

impl Plugin for customKKImport {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_customKK.run_if(if_load_customKK));
    }
}
fn if_load_customKK(q_site:Res<CurrentSite>) -> bool{
  q_site.is_changed() && *q_site == CurrentSite::CustomKK
}
fn load_customKK(mut commands:Commands,q_holder:Single<Entity,With<SiteHolder>>,q_site:Res<CurrentSite>,q_other_site:Query<Entity,With<SiteRoot>>){
  for ent in &q_other_site{
    commands.entity(ent).despawn_recursive();
  }
  let site = commands.spawn((SiteRoot)).id();
  commands.entity(*q_holder).add_child(site);

}