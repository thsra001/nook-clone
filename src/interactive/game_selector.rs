use std::{slice::Iter, thread::spawn};

use bevy::{
    picking::focus::PickingInteraction, prelude::*, render::camera::RenderTarget, window::{WindowLevel, WindowRef, WindowResolution}
};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use rand::prelude::*;

use crate::colours;

#[derive(Resource, Default, Reflect, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum GameSelector {
    population_growing,
    population_growing_snowy,
    population_growing_cherry,
    population_growing_rainy,
    wild_world,
    wild_world_rainy,
    wild_world_snowy,
    new_leaf,
    new_leaf_rainy,
    new_leaf_snowy,
    #[default]
    new_horizons,
    new_horizons_rainy,
    new_horizons_snowy,
    pocket_camp,
    // special
    kk_slider,
    random,
}
// react to presses
#[derive(Component)]
pub struct GamesSelectorButton;
#[derive(Component)]
pub struct GameSelectorText;
// selector text
#[derive(Component)]
pub struct GameSelectorOption(GameSelector);
// pop up menu
#[derive(Component)]
pub struct GameSelectorWindowClose;

impl GameSelector {
    pub fn to_display_name(&self) -> String {
        match self {
            GameSelector::population_growing => String::from("AC: Population Growing (GC)"),
            GameSelector::population_growing_snowy => {
                String::from("AC: Population Growing (GC) [Snowy]")
            }
            GameSelector::population_growing_cherry => {
                String::from("AC: Population Growing (GC) [Sakura]")
            }
            GameSelector::population_growing_rainy => {
                String::from("AC: Population Growing (GC) [Rainy Day]")
            }
            GameSelector::wild_world => String::from("AC: City Folk (Wii)"),
            GameSelector::wild_world_rainy => String::from("AC: City Folk (Wii) [Rainy]"),
            GameSelector::wild_world_snowy => String::from("AC: City Folk (Wii) [Snowy]"),
            GameSelector::new_leaf => String::from("AC: New Leaf (3DS)"),
            GameSelector::new_leaf_rainy => String::from("AC: New Leaf (3DS) [Rainy]"),
            GameSelector::new_leaf_snowy => String::from("AC: New Leaf (3DS) [Snowy]"),
            GameSelector::new_horizons => String::from("AC: New Horizons (Switch)"),
            GameSelector::new_horizons_rainy => String::from("AC: New Horizons (Switch) [Rainy]"),
            GameSelector::new_horizons_snowy => String::from("AC: New Horizons (Switch) [Snowy]"),
            GameSelector::pocket_camp => String::from("AC: Pocket Camp (Mobile)"),
            GameSelector::kk_slider => String::from("K.K. Slider"),
            GameSelector::random => String::from("Random"),
        }
    }
    pub fn to_file_name(&self) -> String {
        match self {
            GameSelector::population_growing => String::from("population-growing"),
            GameSelector::population_growing_snowy => String::from("population-growing-snowy"),
            GameSelector::population_growing_cherry => String::from("population-growing-cherry"),
            GameSelector::population_growing_rainy => String::from("population-growing-rainy"),
            GameSelector::wild_world => String::from("wild-world"),
            GameSelector::wild_world_rainy => String::from("wild-world-rainy"),
            GameSelector::wild_world_snowy => String::from("wild-world-snowy"),
            GameSelector::new_leaf => String::from("new-leaf"),
            GameSelector::new_leaf_rainy => String::from("new-leaf-rainy"),
            GameSelector::new_leaf_snowy => String::from("new-leaf-snowy"),
            GameSelector::new_horizons => String::from("new-horizons"),
            GameSelector::new_horizons_rainy => String::from("new-horizons-rainy"),
            GameSelector::new_horizons_snowy => String::from("new-horizons-snowy"),
            GameSelector::pocket_camp => String::from("pocket-camp"),
            GameSelector::kk_slider => String::from("kk-slider-desktop"),
            GameSelector::random => String::from("random"),
        }
    }
    pub fn iterator() -> Iter<'static, GameSelector> {
        static DIRECTIONS: [GameSelector; 16] = [
            GameSelector::population_growing,
            GameSelector::population_growing_snowy,
            GameSelector::population_growing_cherry,
            GameSelector::population_growing_rainy,
            GameSelector::wild_world,
            GameSelector::wild_world_rainy,
            GameSelector::wild_world_snowy,
            GameSelector::new_leaf,
            GameSelector::new_leaf_rainy,
            GameSelector::new_leaf_snowy,
            GameSelector::new_horizons,
            GameSelector::new_horizons_rainy,
            GameSelector::new_horizons_snowy,
            GameSelector::pocket_camp,
            GameSelector::kk_slider,
            GameSelector::random,
        ];
        DIRECTIONS.iter()
    }
    pub fn games() -> &'static [GameSelector; 14]{
        &[
            GameSelector::population_growing,
            GameSelector::population_growing_snowy,
            GameSelector::population_growing_cherry,
            GameSelector::population_growing_rainy,
            GameSelector::wild_world,
            GameSelector::wild_world_rainy,
            GameSelector::wild_world_snowy,
            GameSelector::new_leaf,
            GameSelector::new_leaf_rainy,
            GameSelector::new_leaf_snowy,
            GameSelector::new_horizons,
            GameSelector::new_horizons_rainy,
            GameSelector::new_horizons_snowy,
            GameSelector::pocket_camp,
        ]
    }
    // todo: fix this bs band aid fix
    pub fn random_game() ->GameSelector {
        let mut rng = rand::rng();
        let choice = GameSelector::games().choose(&mut rng).unwrap().clone();
        match choice {
            GameSelector::population_growing_rainy => GameSelector::random_game(),
            _ => choice
        }
    }
}
pub struct GameSelectorImport;
impl Plugin for GameSelectorImport {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSelector>()
            .register_type::<GameSelector>()
            .add_plugins(ResourceInspectorPlugin::<GameSelector>::new())
            .add_systems(Update, update)
            .add_systems(Update, (ui_update, game_menu));
    }
}
fn update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mous: Res<ButtonInput<MouseButton>>,
    q_select_button: Query<&PickingInteraction, With<GamesSelectorButton>>,
) {
    // on selector button click spawn
    if let Ok(selectbut) = q_select_button.get_single() {
        if *selectbut == PickingInteraction::Pressed && mous.just_pressed(MouseButton::Left) {
            info!("select button pressed");
            let win2 = commands
                .spawn((GameSelectorWindowClose, Window{
                    resolution:WindowResolution::new(200.0, 200.0),
                    window_level:WindowLevel::AlwaysOnTop,
                    position:WindowPosition::At(IVec2::splat(25)),
                    ..default()
                }))
                .id();
            let win2_cam = commands
                .spawn((
                    Camera2d::default(),
                    GameSelectorWindowClose,
                    Camera {
                        target: RenderTarget::Window(WindowRef::Entity(win2)),
                        ..default()
                    },
                ))
                .id();
            commands
                .spawn((
                    Node {
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(colours::SELECTOR_PURBLE),
                    TargetCamera(win2_cam),
                    GameSelectorWindowClose,
                ))
                .with_children(|win| {
                    for game in GameSelector::iterator() {
                        win.spawn((
                            Text::new(game.to_display_name()),
                            TextFont {
                                font: asset_server.load("fonts/inter-lig.ttf"),
                                font_size: 14.0,
                                ..default()
                            },
                            Button,
                            GameSelectorOption(game.clone()),
                            {
                                match &game {
                                    &GameSelector::new_horizons_snowy => {},
                                    &GameSelector::pocket_camp | &GameSelector::kk_slider => {},
                                    _ => {}
                                }
                                // if game == &GameSelector::new_horizons_snowy {
                                //     Node {
                                //         margin: UiRect::bottom(Val::Px(18.0)),
                                //         ..default()
                                //     }
                                // } else {
                                //     Node { ..default() },BackgroundColor
                                // }
                                // ma
                            },
                        ));
                    }
                });
        }
    }
}
fn ui_update(
    mut q_selector_text: Query<&mut Text, With<GameSelectorText>>,
    res_game: Res<GameSelector>,
) {
    if res_game.is_changed() {
        if let Ok(mut tex) = q_selector_text.get_single_mut() {
            **tex = (*res_game).to_display_name()
        }
    }
}
fn game_menu(
    mut commands:Commands,
    mut res_game: ResMut<GameSelector>,
    q_otion: Query<(&PickingInteraction, &GameSelectorOption)>,
    q_close: Query<Entity,With<GameSelectorWindowClose>>
) {
    for (pressed, option) in &q_otion {
        if *pressed == PickingInteraction::Pressed {
            info!("pressed: {}", option.0.to_display_name());
            *res_game = option.0.clone();
            for ent in &q_close{
                commands.entity(ent).despawn_recursive();
            }
        }
    }
}
