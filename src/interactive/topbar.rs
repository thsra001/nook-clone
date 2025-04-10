use bevy::{picking::focus::PickingInteraction, prelude::*};

use super::{player::MusicPlaying, ButtonSet};


#[derive(Component)]
pub struct PlayerButton;

#[derive(Component)]
pub struct MiniButton;

#[derive(Component)]
pub struct CloseButton;
pub struct TopbarImport;
impl Plugin for TopbarImport {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, topbar_buttons.in_set(ButtonSet));
}}
fn topbar_buttons(
    // player button
    q_player: Query<&PickingInteraction,With<PlayerButton>>,
    mut res_playing: ResMut<MusicPlaying>,
    // minimise button
    q_mini: Query<&PickingInteraction,With<MiniButton>>,
    mut window: Query<&mut Window>,
    // close button
    q_close: Query<&PickingInteraction,With<CloseButton>>,
    mut exit: EventWriter<AppExit>,
    mut commands:Commands,
    mut windows:Query<Entity,With<Window>>
){
    if let Ok(player) = q_player.get_single(){
        if *player == PickingInteraction::Pressed{
           *&mut res_playing.0=!*&mut res_playing.0;
        }
        
    }
    if let Ok(mini) = q_mini.get_single(){
        if *mini == PickingInteraction::Pressed{
        //   window.set_minimized(true);
        //   window.visible=false;
          info!("minimised");
          for wind in &windows{
            commands.entity(wind).despawn_recursive();
          }
        }
    }
    if let Ok(close) = q_close.get_single(){
        if *close == PickingInteraction::Pressed{
            exit.send(AppExit::Success);
        }
    }

}
