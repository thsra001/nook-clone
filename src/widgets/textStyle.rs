use bevy::prelude::*;

pub fn h1( asset_server: &ResMut<AssetServer>) -> bevy::prelude::TextFont {
    TextFont {
        font: asset_server.load("fonts/inter-reg.ttf"),
        font_size: 18.0,
        ..default()
    }
}
pub fn h2( asset_server: &ResMut<AssetServer>) -> bevy::prelude::TextFont {
    TextFont {
        font: asset_server.load("fonts/inter-lig.ttf"),
        font_size: 16.0,
        ..default()
    }
}
pub fn h3( asset_server: &ResMut<AssetServer>) -> bevy::prelude::TextFont {
    TextFont {
        font: asset_server.load("fonts/inter-lig.ttf"),
        font_size: 10.0,
        ..default()
    }
}
