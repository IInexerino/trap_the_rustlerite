use bevy::{app::{AppExit, Plugin, Update}, color::{palettes::css::ORANGE, Color}, ecs::{ component::Component, entity::Entity, event::EventWriter, query::{Changed, With}, schedule::IntoScheduleConfigs, system::{Commands, Query, Res, ResMut}}, prelude::{children, SpawnRelated}, state::{app::AppExtStates, condition::in_state, state::{NextState, OnEnter, OnExit}}, text::{TextColor, TextFont}, ui::{widget::{Button, Text}, AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, Node, UiRect, Val}, utils::default};
use crate::game::game::{save_total_game_stats, AppState, MenuState, TotalGameStats};

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {

        app.init_state::<MenuState>();
        
        app.add_systems(
            OnEnter(AppState::MainMenu), 
            menu_setup
        )
        .add_systems(
            OnEnter(MenuState::Main), 
            setup_main_menu
        )
        .add_systems(
            OnExit(MenuState::Main), 
            despawn_screen::<MainMenuScreen>,
        )
        .add_systems(
            OnEnter(MenuState::Stats), 
            setup_stats_menu
        )
        .add_systems(
            OnExit(MenuState::Stats), 
            despawn_screen::<StatsMenuScreen>,
        )
        .add_systems(
            OnEnter(MenuState::Quit), 
            (save_total_game_stats, exit_game).chain()
        )
        .add_systems(
            Update, 
            (menu_action, button_system)
                .run_if(in_state(AppState::MainMenu))
        );
    }
}

#[derive(Component)]
pub struct MainMenuScreen;

#[derive(Component)]
pub struct StatsMenuScreen;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Stats,
    Quit,
    BackToMainMenu,
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

fn menu_setup(
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    menu_state.set(MenuState::Main);
}

fn setup_main_menu(
    mut commands: Commands,
) {
    let button_node = Node {
            width: Val::Px(300.0),
            height: Val::Px(48.75),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()

        },
        MainMenuScreen,
        children![(
            Node{
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(ORANGE.into()),
            children![
                (
                    Text::new("Trap the Rustlerite"),
                    TextFont {
                        font_size: 67.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: UiRect::all(Val::Px(50.0)),
                        ..default()
                    },

                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Play,
                    children![
                        (
                            Text::new("New Game"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Stats,
                    children![
                        (
                            Text::new("Stats"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Quit,
                    children![
                        (
                            Text::new("Quit"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
            ]
        )],
    ));
}

fn setup_stats_menu(
    mut commands: Commands,
    game_statistics: Res<TotalGameStats>
) {

    let button_text_font = (
        TextFont {
            font_size: 33.0,
            ..default()
        },
    );

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        StatsMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(ORANGE.into()),
            children![
                (
                    Text::new("Total Statistics:"),
                    TextFont {
                        font_size: 45.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: UiRect::all(Val::Px(50.0)),
                        ..default()
                    }
                ),
                (
                    Text::new(format!("Record level: {}", game_statistics.record_level)),
                    TextFont {
                        font_size: 35.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: UiRect::all(Val::Px(30.0)),
                        ..default()
                    }
                ),
                (
                    Text::new(format!("Rustaceans trapped: {}", game_statistics.tigers_trapped)),
                    TextFont {
                        font_size: 35.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: UiRect::all(Val::Px(30.0)),
                        ..default()
                    }
                ),
                (
                    Text::new(format!("Rustaceans escaped: {}", game_statistics.tigers_escaped)),
                    TextFont {
                        font_size: 35.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: UiRect::all(Val::Px(30.0)),
                        ..default()
                    }
                ),
                (
                    Text::new(format!("Tiles tapped: {}", game_statistics.tiles_tapped)),
                    TextFont {
                        font_size: 35.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: UiRect::all(Val::Px(30.0)),
                        ..default()
                    }
                ),
                (
                    Text::new(format!("Games played: {}", game_statistics.games_played)),
                    TextFont {
                        font_size: 35.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: UiRect::all(Val::Px(30.0)),
                        ..default()
                    }
                ),
                (
                    Button,
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::BackToMainMenu,
                    children![
                        (
                            Text::new("Return to Main Menu"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
            ]
        )]
    ));
}

/* 
pub fn main_menu_loop(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_app: ResMut<NextState<AppState>>,
    mut game_statistics: ResMut<TotalGameStats>
) {
    // configure grid settings
    if keys.just_pressed(KeyCode::Digit7) {
        next_app.set(AppState::InGame);
        game_statistics.games_played += 1;
    }
}
*/

fn exit_game(
    mut app_exit_events: EventWriter<AppExit>,
) {
    app_exit_events.write(AppExit::Success);
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}


fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Play => {
                    app_state.set(AppState::InGame);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Stats => menu_state.set(MenuState::Stats),
                MenuButtonAction::Quit => menu_state.set(MenuState::Quit),
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
            }
        }
    }
}


// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}