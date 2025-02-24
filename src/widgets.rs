use bevy::prelude::*;
pub mod textStyle;
pub mod tickbox;
use tickbox::*;
pub struct WidgetImport;

impl Plugin for WidgetImport {
    fn build(&self, app: &mut App) {
      app.add_plugins(TickboxImport);
    }}