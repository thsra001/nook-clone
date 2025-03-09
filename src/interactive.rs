use bevy::prelude::*;
pub(crate) mod player;
use player::*;
pub(crate) mod topbar;
use topbar::*;
pub(crate) mod slider;
use slider::*;
pub(crate) mod game_selector;
use game_selector::*;
pub(crate) mod music;
use music::*;
pub(crate) mod rain;
use rain::*;
pub(crate) mod i18evy;
use i18evy::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ButtonSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ButtonReflectSet;
pub struct InterImport;

impl Plugin for InterImport {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (ButtonSet.before(ButtonReflectSet),ButtonReflectSet))
        .add_plugins(PlayerImport)
        .add_plugins(TopbarImport)
        .add_plugins(SliderImport)
        .add_plugins(MusicImport)
        .add_plugins(RainImport)
        .add_plugins(GameSelectorImport)
        .add_plugins(I18evyImport);
    }
}
