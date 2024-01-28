use bevy::{prelude::*, transform};

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Renderable;

#[derive(Component)]
struct KeyboardMovable;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Collider;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            ..default()
        },
        Renderable,
        Position { x: 153.0, y: 130.0 },
        KeyboardMovable,
        Collider,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.15, 0.15, 0.25),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            ..default()
        },
        Renderable,
        Position { x: 53.0, y: 30.0 },
        Enemy,
        Collider,
    ));
}

fn draw_renderable(mut query: Query<(&Position, &mut Transform), With<Renderable>>) {
    for (position, mut transform) in &mut query {
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

fn keyboard_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Position, &KeyboardMovable)>,
) {
    for (mut position, _) in &mut query {
        if keyboard_input.pressed(KeyCode::Left) {
            position.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            position.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            position.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            position.y -= 1.0;
        }
    }
}

fn enemy_move(
    (mut query, player): (
        Query<(&mut Position, &Enemy)>,
        Query<(&Position, &KeyboardMovable), Without<Enemy>>,
    ),
) {
    let (player_position, keyboard_movable) = player.single();
    for (mut position, _) in &mut query {
        if position.x < player_position.x {
            position.x += 0.8;
        }
        if position.x > player_position.x {
            position.x -= 0.8;
        }
        if position.y < player_position.y {
            position.y += 0.8;
        }
        if position.y > player_position.y {
            position.y -= 0.8;
        }
    }
}

fn detect_collision(
    (mut query, player): (
        Query<(&Sprite, &Position, &Enemy, &Collider)>,
        Query<(&Sprite, &Position, &KeyboardMovable, &Collider), Without<Enemy>>,
    ),
) {
    let (player_sprite, player_position, _, _) = player.single();
    for (enemy_sprite, enemy_position, _, _) in &mut query {
        let enemy_right = enemy_position.x + enemy_sprite.custom_size.unwrap().x;
        let enemy_bottom = enemy_position.y + enemy_sprite.custom_size.unwrap().y;
        let player_right = player_position.x + player_sprite.custom_size.unwrap().x;
        let player_bottom = player_position.y + player_sprite.custom_size.unwrap().y;

        if enemy_position.x < player_right
            && enemy_right > player_position.x
            && enemy_position.y < player_bottom
            && enemy_bottom > player_position.y
        {
            println!("collision");
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (keyboard_move, enemy_move, draw_renderable, detect_collision),
        )
        .run();
}
