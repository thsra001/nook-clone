use bevy::prelude::*;
mod player;
use player::*;
pub(crate) mod topbar;
use topbar::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiReflectSet;
pub struct InterImport;

impl Plugin for InterImport {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (UiSet.before(UiReflectSet),UiReflectSet))
        .add_plugins(PlayerImport)
        .add_plugins(TopbarImport);
    }
}
