use crate::buttons::NewGameButton;
use crate::buttons::SurrenderButton;
use crate::events::*;
use crate::game_logic::*;
use crate::socket::SendToServerEvent;
use crate::socket::SocketMessageSender;
use crate::ui::RootUINode;
use bevy::prelude::*;
use bevy::transform::commands;

pub const BOARD_WIDTH: f32 = CELL_SIZE * 7.0;
pub const BOARD_HEIGHT: f32 = CELL_SIZE * 6.0;
pub const CELL_SIZE: f32 = 62.0;
pub const PIECE_RADIUS: f32 = 24.0;
pub const BOARD_COLOR: Color = Color::srgb(0.2, 0.4, 0.8);
pub const HOLE_COLOR: Color = Color::srgb(0.1, 0.2, 0.4);
pub const BOARD_OFFSET_Y: f32 = -60.0; // Offset to position board below UI

#[derive(Component)]
pub struct BoardCell {
    pub row: usize,
    pub col: usize,
}

#[derive(Component)]
pub struct GamePiece {
    pub row: usize,
    pub col: usize,
    pub player: Player,
}

#[derive(Component)]
pub struct AnimatingPiece {
    pub target_row: usize,
    pub col: usize,
    pub start_y: f32,
    pub target_y: f32,
    pub timer: Timer,
}

#[derive(Component)]
pub struct ColumnHighlight {
    pub col: usize,
}

#[derive(Component)]
pub struct PreviewPiece {
    pub col: usize,
}

pub fn setup_board(mut commands: Commands) {
    // Spawn the board background
    commands.spawn((
        Sprite {
            color: BOARD_COLOR,
            custom_size: Some(Vec2::new(BOARD_WIDTH, BOARD_HEIGHT)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, BOARD_OFFSET_Y, 0.0),
            ..default()
        },
    ));

    // Create the grid of holes
    let start_x = -(BOARD_WIDTH / 2.0) + (CELL_SIZE / 2.0);
    let start_y = (BOARD_HEIGHT / 2.0) - (CELL_SIZE / 2.0) + BOARD_OFFSET_Y;

    for row in 0..6 {
        for col in 0..7 {
            let x = start_x + (col as f32 * CELL_SIZE);
            let y = start_y - (row as f32 * CELL_SIZE);

            // Create the hole (visual representation of empty cell)
            commands.spawn((
                Sprite {
                    color: HOLE_COLOR,
                    custom_size: Some(Vec2::new(PIECE_RADIUS * 2.0, PIECE_RADIUS * 2.0)),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(x, y, 1.0),
                    ..default()
                },
                BoardCell { row, col },
            ));
        }
    }

    // Create column highlights (invisible by default)
    for col in 0..7 {
        let x = start_x + (col as f32 * CELL_SIZE);

        commands.spawn((
            Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, 0.0), // Transparent by default
                custom_size: Some(Vec2::new(CELL_SIZE, BOARD_HEIGHT)),
                ..default()
            },
            Transform {
                translation: Vec3::new(x, BOARD_OFFSET_Y, 0.5),
                ..default()
            },
            Visibility::Hidden,
            ColumnHighlight { col },
        ));

        // Hovering preview piece for this column
        commands.spawn((
            Sprite {
                color: Color::srgba(0.8, 0.2, 0.2, 0.7), // Preview piece color (red, semi-transparent)
                custom_size: Some(Vec2::new(PIECE_RADIUS * 2.0, PIECE_RADIUS * 2.0)),
                ..default()
            },
            Transform {
                // Place slightly above the top row
                translation: Vec3::new(x, BOARD_OFFSET_Y + BOARD_HEIGHT / 2. + CELL_SIZE / 2., 3.0),
                ..default()
            },
            Visibility::Hidden,
            PreviewPiece { col },
        ));
    }
}

// #![allow(clippy::too_many_arguments)]
use crate::WsMsg;

#[allow(clippy::too_many_arguments)]
pub fn handle_input(
    _commands: Commands,
    // sender: Res<SocketMessageSender>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    game_state: Res<GameState>,
    mut sender: EventWriter<SendToServerEvent>,
    mut param_set: ParamSet<(
        Query<(&mut Sprite, &mut Visibility, &ColumnHighlight)>,
        Query<(&mut Sprite, &mut Visibility, &PreviewPiece)>,
    )>,
    my_player: Res<crate::MyPlayerInfo>,
) {
    // Only allow move if I'm the active player
    let is_my_turn = match my_player.color {
        Some(Player::One) => game_state.current_player == Player::One,
        Some(Player::Two) => game_state.current_player == Player::Two,
        _ => false,
    };

    let _window = windows.single();
    if let Ok((_camera, camera_transform)) = camera.single() {
        let window = match windows.single() {
            Ok(win) => win,
            Err(_) => return,
        };
        if let Some(cursor_pos) = window.cursor_position() {
            let world_pos = camera_transform
                .compute_matrix()
                .transform_point3(Vec3::new(
                    cursor_pos.x - window.width() / 2.0,
                    cursor_pos.y - window.height() / 2.0,
                    0.0,
                ))
                .truncate();
            let start_x = -(BOARD_WIDTH / 2.0);
            // let start_y = (BOARD_HEIGHT / 2.0) + BOARD_OFFSET_Y;
            // let board_bottom = (BOARD_HEIGHT / 2.0) + BOARD_OFFSET_Y - BOARD_HEIGHT;
            // let board_top = (BOARD_HEIGHT / 2.0) + BOARD_OFFSET_Y - 2.0 * CELL_SIZE;
            if world_pos.x >= start_x
                && world_pos.x <= start_x + BOARD_WIDTH
                && world_pos.y
                    >= ((BOARD_OFFSET_Y - BOARD_HEIGHT + (2.0 * CELL_SIZE) + (CELL_SIZE / 2.0))
                        / 2.0)
                && world_pos.y
                    <= ((BOARD_OFFSET_Y + BOARD_HEIGHT + (2.0 * CELL_SIZE) + (CELL_SIZE / 2.0))
                        / 2.0)
            {
                let col = (((world_pos.x - start_x) / CELL_SIZE).floor()) as i32;
                // Update column highlights
                for (mut sprite, mut visibility, highlight) in param_set.p0().iter_mut() {
                    if (0..7).contains(&col) && highlight.col == col as usize {
                        if game_state.status == GameStatus::Playing
                            && !game_state.is_column_full(col as usize)
                        {
                            *visibility = Visibility::Visible;
                            sprite.color = Color::srgba(1.0, 1.0, 1.0, 0.2);
                        } else {
                            *visibility = Visibility::Hidden;
                        }
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }

                // Update hovering preview piece color/visibility
                for (mut sprite, mut visibility, preview) in param_set.p1().iter_mut() {
                    if (0..7).contains(&col) && preview.col == col as usize {
                        if game_state.status == GameStatus::Playing
                            && !game_state.is_column_full(col as usize)
                        {
                            *visibility = Visibility::Visible;
                            sprite.color = game_state
                                .current_player
                                .color()
                                .expect("could not get color")
                                .with_alpha(0.7);
                        } else {
                            *visibility = Visibility::Hidden;
                        }
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }

                // Handle mouse clicks
                if mouse_input.just_pressed(MouseButton::Left)
                    && (0..7).contains(&col)
                    && is_my_turn
                {
                    let col = col as usize;
                    info!("mouse click for move on col: {col}");
                    if game_state.status == GameStatus::Playing
                        && !game_state.is_column_full(col)
                        && my_player.id.is_some()
                    {
                        info!("sending message to server for move");
                        sender.write(SendToServerEvent(WsMsg::ClientMove {
                            id: my_player.clone().id.expect("id should be some"),
                            col,
                        }));
                    }
                }
            } else {
                // Hide all highlights and previews when not on board
                for (_sprite, mut visibility, _highlight) in param_set.p0().iter_mut() {
                    *visibility = Visibility::Hidden;
                }
                for (_sprite, mut visibility, _preview) in param_set.p1().iter_mut() {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}

pub fn handle_piece_drop(
    mut commands: Commands,
    mut piece_drop_events: EventReader<PieceDropEvent>,
) {
    for event in piece_drop_events.read() {
        // Always use the game state's current player for piece color (matches latest move)
        let piece_color = event.player;

        let target_row = event.row;

        // Calculate positions for animation
        let start_x = -(BOARD_WIDTH / 2.0) + (CELL_SIZE / 2.0);
        let start_y = (BOARD_HEIGHT / 2.0) - (CELL_SIZE / 2.0) + BOARD_OFFSET_Y;

        let x = start_x + (event.column as f32 * CELL_SIZE);
        let start_y_pos = start_y + CELL_SIZE; // Start above the board
        let target_y_pos = start_y - (target_row as f32 * CELL_SIZE);

        // Spawn the animated piece using the state player
        commands.spawn((
            Sprite {
                color: piece_color.color().expect("could not get color"),
                custom_size: Some(Vec2::new(PIECE_RADIUS * 2.0, PIECE_RADIUS * 2.0)),
                ..default()
            },
            Transform {
                translation: Vec3::new(x, start_y_pos, 2.0),
                ..default()
            },
            AnimatingPiece {
                target_row,
                col: event.column,
                start_y: start_y_pos,
                target_y: target_y_pos,
                timer: Timer::from_seconds(0.5, TimerMode::Once),
            },
        ));
    }
}

pub fn handle_change_player(
    mut game_state: ResMut<GameState>,
    mut change_player_events: EventReader<ChangePlayerEvent>,
) {
    for event in change_player_events.read() {
        game_state.current_player = event.player;
    }
}

pub fn animate_pieces(
    mut commands: Commands,
    time: Res<Time>,
    mut animating_pieces: Query<(Entity, &mut Transform, &Sprite, &mut AnimatingPiece)>,
    _game_state: Res<GameState>,
) {
    for (entity, mut transform, sprite, mut anim) in animating_pieces.iter_mut() {
        anim.timer.tick(time.delta());

        let progress = anim.timer.elapsed_secs() / anim.timer.duration().as_secs_f32();
        let eased_progress = ease_out_bounce(progress);

        let current_y = anim.start_y + (anim.target_y - anim.start_y) * eased_progress;
        transform.translation.y = current_y;

        if anim.timer.finished() {
            // Convert to static piece - determine player from sprite color
            let player = if sprite.color == Player::One.color().expect("could not get color") {
                Player::One
            } else {
                Player::Two
            };

            commands.entity(entity).remove::<AnimatingPiece>();
            commands.entity(entity).insert(GamePiece {
                row: anim.target_row,
                col: anim.col,
                player,
            });
        }
    }
}

fn ease_out_bounce(t: f32) -> f32 {
    if t < 1.0 / 2.75 {
        7.5625 * t * t
    } else if t < 2.0 / 2.75 {
        let t = t - 1.5 / 2.75;
        7.5625 * t * t + 0.75
    } else if t < 2.5 / 2.75 {
        let t = t - 2.25 / 2.75;
        7.5625 * t * t + 0.9375
    } else {
        let t = t - 2.625 / 2.75;
        7.5625 * t * t + 0.984375
    }
}

// Clean up pieces when game resets
pub fn cleanup_pieces(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    pieces: Query<Entity, Or<(With<GamePiece>, With<AnimatingPiece>)>>,
    mut ui_query: Query<Entity, With<RootUINode>>,
    new_game_buttons: Query<Entity, With<NewGameButton>>,
    mut reset_events: EventReader<GameResetEvent>,
) {
    for _ in reset_events.read() {
        game_state.current_player = Player::One;
        game_state.status = GameStatus::Playing;
        for entity in pieces.iter() {
            commands.entity(entity).despawn();
        }
        for entity in new_game_buttons.iter() {
            commands.entity(entity).despawn();
        }
        if let Ok(node) = ui_query.single_mut() {
            commands.entity(node).with_children(|parent| {
                parent
                    .spawn((
                        Button,
                        Node {
                            margin: UiRect::all(Val::Px(10.0)),
                            padding: UiRect::all(Val::Px(10.0)),
                            ..Default::default()
                        },
                        BackgroundColor(Color::BLACK),
                        SurrenderButton,
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new("Surrender"),
                            TextFont {
                                // font: asset_server.load("default_font.ttf"),
                                font_size: 20.0,
                                ..Default::default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            });
        }
    }
}

pub fn handle_game_over(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut events: EventReader<GameOverEvent>,
    mut ui_query: Query<Entity, With<RootUINode>>,
    mut surrender_button: Query<Entity, With<SurrenderButton>>,
) {
    for event in events.read() {
        game_state.status = GameStatus::Won(event.winner);
        if let Ok(button) = surrender_button.single_mut() {
            commands.entity(button).despawn();
        }
        if let Ok(node) = ui_query.single_mut() {
            commands.entity(node).with_children(|parent| {
                parent
                    .spawn((
                        Button,
                        Node {
                            margin: UiRect::all(Val::Px(10.0)),
                            padding: UiRect::all(Val::Px(10.0)),
                            ..Default::default()
                        },
                        BackgroundColor(Color::BLACK),
                        NewGameButton,
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new("New Game"),
                            TextFont {
                                // font: asset_server.load("default_font.ttf"),
                                font_size: 20.0,
                                ..Default::default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            });
        }
    }
}
