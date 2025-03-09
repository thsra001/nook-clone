use bevy::prelude::*;

use crate::colours;

// drop down box - selected test and a arrow down, pressing opens a selector window
#[derive(Component)]
#[require(Node(||{Node {
            width: Val::Px(312.0),
            height: Val::Px(42.0),
            align_items: AlignItems::Center,
            padding: UiRect::left(Val::Px(12.0)),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        }}))]
#[require(BackgroundColor(||{BackgroundColor(colours::SELECTOR_PURBLE)}))]
#[require(Button)]
pub struct  DropDown;
// what res does this dropdown manage
#[derive(Component)]
pub enum DropDownRes {
    Song,
    Lang,
}
pub struct DropdownImport;

impl Plugin for DropdownImport {
    fn build(&self, app: &mut App) {
        app;
    }
}

// body.spawn((
//     Node {
//         width: Val::Px(312.0),
//         height: Val::Px(42.0),
//         align_items: AlignItems::Center,
//         padding: UiRect::left(Val::Px(12.0)),
//         justify_content: JustifyContent::SpaceBetween,
//         ..default()
//     },
//     BackgroundColor(colours::SELECTOR_PURBLE),
//     Name::new("gameSelectorBollock"),
//     GamesSelectorButton,
//     Button,
// ))
// .with_children(|selector| {
//     // current choice
//     selector.spawn((
//         Text::new(GameSelector::population_growing.to_display_name()),
//         I18Key::AcPopulationGrowingGc,
//         TextFont {
//             font: asset_server.load("fonts/inter-lig.ttf"),
//             font_size: 14.0,
//             ..default()
//         },
//         GameSelectorText,
//         PickingBehavior {
//             should_block_lower: false,
//             is_hoverable: false,
//         },
//     ));
//     // drop down
//     selector
//         .spawn((
//             Node {
//                 width: Val::Px(42.0),
//                 height: Val::Px(42.0),
//                 align_items: AlignItems::Center,
//                 justify_content: JustifyContent::Center,
//                 ..default()
//             },
//             BackgroundColor(SELECTOR_PURBLE2),
//             PickingBehavior {
//                 should_block_lower: false,
//                 is_hoverable: false,
//             },
//         ))
//         .with_child((
//             Text::new(""),
//             TextFont {
//                 font: asset_server.load("fonts/nerd-symbols-reg.ttf"),
//                 font_size: 24.0,
//                 ..default()
//             },
//             PickingBehavior {
//                 should_block_lower: false,
//                 is_hoverable: false,
//             },
//         ));
// });
