use bevy::prelude::*;
pub mod textStyle;
pub mod tickbox;
use tickbox::*;
pub mod buttons;
use buttons::*;
pub mod topbar2;
use topbar2::*;
pub mod drop_down;
use drop_down::*;
pub struct WidgetImport;

impl Plugin for WidgetImport {
    fn build(&self, app: &mut App) {
      app.add_plugins(TickboxImport)
      .add_plugins(ButtonImport);
    }}