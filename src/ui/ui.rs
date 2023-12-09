use bevy::prelude::*;

pub struct MainMenuUI;

#[derive(Component)]
pub struct StartButton;
impl Plugin for MainMenuUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_start_menu);
        //app.add_systems(Update, button_system);
    }
}


/*fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}*/

fn spawn_start_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,

                ..default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),

            ..default()
        },
        Name::new("Main Menu Background"),
    ));
    commands
        .spawn(NodeBundle {
            style: Style {
                max_width: Val::Percent(36.0),
                position_type: PositionType::Absolute,
                border: UiRect::all(Val::Percent(0.6)),
                left: Val::Px(6.0),
                margin: UiRect::all(Val::Percent(0.45)),
                ..default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
            border_color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|parent| {
            // text
            parent.spawn((
                TextBundle::from_section(
                    "Circuit Cider",
                    TextStyle {
                        font: asset_server.load("TauroCondensed-eZrGB.ttf"),
                        font_size: 60.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.)),
                    top: Val::Px(-3.0),
                    ..default()
                }),
                Name::new("CircuitCider Text"),
            ));
        });

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(200.0),
                margin: UiRect::top(Val::VMin(5.)),
                left: Val::Percent(8.5),
                top: Val::Vw(5.0),
                position_type: PositionType::Absolute,
                border: UiRect::all(Val::Percent(0.425)),
                ..default()
            },
            // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
            background_color: Color::WHITE.into(),
            border_color: Color::BLACK.into(),
            ..default()
        },
        Name::new("Logo"),
        UiImage::new(asset_server.load("bevyteam5_upscaled.png")),
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(255.0),
                    height: Val::Px(300.0),
                    margin: UiRect::top(Val::VMin(5.)),
                    left: Val::Percent(7.5),
                    top: Val::Percent(40.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            },
            Name::new("Button Nodes"),
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(241.0),
                        height: Val::Px(75.0),
                        border: UiRect::all(Val::Percent(2.0)),
                        top: Val::Percent(-35.0),
                        left: Val::Percent(64.2),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: Color::BLACK.into(),
                    background_color: Color::rgb_u8(88, 117, 79).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font: asset_server.load("TauroCondensed-eZrGB.ttf"),
                            font_size: (40.0),
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    )
                        .with_style(Style {
                            margin: UiRect{
                                top: Val::Px(0.0),
                                bottom: Val::Px(0.0),
                                left: Val::Px(10.5),
                                right: Val::Px(10.5),
                            },
                            ..default()
                        }),
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(241.0),
                        height: Val::Px(75.0),
                        border: UiRect::all(Val::Percent(2.0)),
                        top: Val::Percent(0.0),
                        left: Val::Px(-19.5),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: Color::BLACK.into(),
                    background_color: Color::rgb_u8(58, 78, 108).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                        "CUSTOMIZE",
                        TextStyle {
                            font: asset_server.load("TauroCondensed-eZrGB.ttf"),
                            font_size: (40.0),
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    )
                    .with_style(Style {
                        margin: UiRect{
                            top: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            left: Val::Px(10.5),
                            right: Val::Px(10.5),
                        },
                        ..default()
                    }),
                ));
            });
    })
        .with_children(|parent| {
            parent
                .spawn(
                    ButtonBundle {
                    style: Style {
                        width: Val::Px(241.0),
                        height: Val::Px(75.0),
                        border: UiRect::all(Val::Percent(2.0)),
                        top: Val::Percent(35.5),
                        left: Val::Px(-195.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: Color::BLACK.into(),
                    background_color: Color::rgb_u8(148, 52, 52).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                        "EXIT",
                        TextStyle {
                            font: asset_server.load("TauroCondensed-eZrGB.ttf"),
                            font_size: (40.0),
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    )
                    .with_style(Style {
                        margin: UiRect{
                            top: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            left: Val::Px(10.5),
                            right: Val::Px(10.5),
                        },
                        ..default()
                    }),
                ));
            });
    });
}
