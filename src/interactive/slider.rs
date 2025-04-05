use bevy::{picking::focus::PickingInteraction, prelude::*, ui::RelativeCursorPosition};

use crate::interactive;

use super::{
    player::{MusicVolume, RainVolume},
    ButtonReflectSet, ButtonSet, LooseInputSet,
};

#[derive(Component)]
#[require(RelativeCursorPosition, Button, SliderRes, Changing)]
#[require(Name(||{Name::new("Slider")}))]
pub struct Slider;
#[derive(Component)]
pub struct SliderHead;
#[derive(Component, Default)]
pub enum SliderRes {
    MusVol,
    RainVol,
    #[default]
    Missing,
}
#[derive(Component, Default)]
pub struct Changing(bool);
pub(crate) struct SliderImport;

impl Plugin for SliderImport {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                slider_update.in_set(LooseInputSet),
                slider_head_update.in_set(ButtonReflectSet),
            ),
        );
    }
}
fn slider_update(
    mut q_relcur_pick: Query<
        (
            &RelativeCursorPosition,
            &PickingInteraction,
            &SliderRes,
            &mut Changing,
        ),
        With<Slider>,
    >,
    mut res_mus_vol: ResMut<MusicVolume>,
    mut res_rain_vol: ResMut<RainVolume>,
    res_mouse: Res<ButtonInput<MouseButton>>,
) {
    for (cursor_rel_pos, pick_inter, slider_res, mut changing) in &mut q_relcur_pick {
        if *pick_inter == PickingInteraction::Pressed && res_mouse.just_pressed(MouseButton::Left) {
            changing.0 = true
        } else if res_mouse.just_released(MouseButton::Left) {
            changing.0 = false
        };

        if changing.0 {
            if let Some(relative_cursor_position) = cursor_rel_pos.normalized {
                match slider_res {
                    SliderRes::MusVol => {
                        *&mut res_mus_vol.0 = relative_cursor_position.x.clamp(0.0, 1.0)*2.0;
                    }
                    SliderRes::RainVol => {
                        *&mut res_rain_vol.0 = relative_cursor_position.x.clamp(0.0, 1.0)*2.0;
                    }
                    SliderRes::Missing => panic!("remember to add sliderRes comp with other type"),
                    _ => panic!("sliderRes type not implemented"),
                };
            };
        }
    }
}
fn slider_head_update(
    res_mus_vol: ResMut<MusicVolume>,
    res_rain_vol: Res<RainVolume>,
    mut q_slider_head: Query<(&SliderRes, &mut Node), With<SliderHead>>,
) {
    if res_mus_vol.is_changed() | res_rain_vol.is_changed() {
        for (slider_res, mut node) in &mut q_slider_head {
            match slider_res {
                SliderRes::MusVol => node.left = Val::Px(res_mus_vol.0 * 56.0),
                SliderRes::RainVol => node.left = Val::Px(res_rain_vol.0 * 56.0),
                SliderRes::Missing => {
                    panic!("remember to add sliderRes with a valid variant")
                }
                _ => {
                    panic!("SliderRes type not implemented")
                }
            }
        }
    }
}
