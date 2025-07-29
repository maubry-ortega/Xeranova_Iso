use bevy::prelude::*;
use crate::utils::map_size::{MapSize, SelectedMapSize};
use crate::utils::colors::*;
use crate::GameState;

#[derive(Component)]
pub struct MenuRoot;

#[derive(Component)]
pub struct MenuCamera2D;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 1.0, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 1.0, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 1.0, 0.35);

pub fn spawn_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Cámara UI
    commands.spawn((
        Camera2d::default(),
        MenuCamera2D,
    ));

    // Menú UI
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(OVERLAY_BLACK),
            MenuRoot,
        ))
        .with_children(|parent| {
            for (label, size) in [
                ("Pequeño", MapSize::Small),
                ("Mediano", MapSize::Medium),
                ("Grande", MapSize::Large),
            ] {
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(NORMAL_BUTTON),
                        BorderColor(Color::BLACK),
                        size,
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(label),
                            TextFont {
                                font: asset_server.load("fonts/FiraSans-ExtraBoldItalic.ttf"),
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            }
        });
}

pub fn handle_menu_input(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &MapSize,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut selected: ResMut<SelectedMapSize>,
    mut next_state: ResMut<NextState<GameState>>,
    root_query: Query<Entity, With<MenuRoot>>,
    camera_query: Query<Entity, With<MenuCamera2D>>,
) {
    for (interaction, mut bg, mut border, size, children) in &mut interaction_query {
        let Ok(mut text) = text_query.get_mut(children[0]) else { continue };

        match *interaction {
            Interaction::Pressed => {
                text.0 = "Seleccionado".into();
                *bg = PRESSED_BUTTON.into();
                border.0 = Color::srgb(1.0, 0.0, 0.0);

                selected.0 = Some(*size);

                // Eliminar menú y cámara UI
                for entity in &root_query {
                    commands.entity(entity).despawn();
                }
                for entity in &camera_query {
                    commands.entity(entity).despawn();
                }

                next_state.set(GameState::Generating);
            }
            Interaction::Hovered => {
                text.0 = "Pasando...".into();
                *bg = HOVERED_BUTTON.into();
                border.0 = Color::WHITE;
            }
            Interaction::None => {
                text.0 = match size {
                    MapSize::Small => "Pequeño",
                    MapSize::Medium => "Mediano",
                    MapSize::Large => "Grande",
                }
                .into();
                *bg = NORMAL_BUTTON.into();
                border.0 = Color::BLACK;
            }
        }
    }
}
