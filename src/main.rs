use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    render::{camera::Camera, pass::ClearColor},
    sprite::collide_aabb::{collide, Collision},
    window::CursorMoved,
};

/// An implementation of the classic game "Breakout"
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(MousePosition(Vec2::ZERO))
        .add_startup_system(init_scene.system())
        .add_startup_system(init_units.system())
        .add_system(select_unit.system())
        .add_system_to_stage(CoreStage::PreUpdate, update_mouse_position.system())
        .run();
}

struct Unit;
struct Owned {
    owner: Player,
}

#[derive(PartialEq)]
enum Player {
    You,
    Nobody,
}

struct MousePosition(Vec2);

fn init_scene(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn init_units(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, -50.0, 1.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(Unit {})
        .insert(Owned { owner: Player::You });
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 1.0, 0.5).into()),
            transform: Transform::from_xyz(0.0, 50.0, 1.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(Unit {})
        .insert(Owned {
            owner: Player::Nobody,
        });
}

/// This system prints out all mouse events as they come in
fn update_mouse_position(
    mut cursor: EventReader<CursorMoved>,
    mut mouse_position: ResMut<MousePosition>,
) {
    if let Some(cursor_moved) = cursor.iter().last() {
        mouse_position.0 = cursor_moved.position;
    }
}

fn select_unit(
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_position: Res<MousePosition>,
    mut query: Query<(&Unit, &Owned, &SpriteBundle)>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        info!("hey");
        for item in query.iter() {
            info!("poop")
        }
        let collisions: Vec<(&Unit, &Owned, &SpriteBundle)> = query
            .iter()
            .filter(|(_, owned, sprite)| {
                if owned.owner != Player::You {
                    info!("owned by not you");
                    return false;
                }

                let collision = collide(
                    sprite.transform.translation,
                    sprite.sprite.size,
                    Vec3::new(mouse_position.0.x, mouse_position.0.y, 0.0),
                    Vec2::new(1.0, 1.0),
                );

                info!("{:?}", collision);
                collision.is_some()
            })
            .collect();
        for (_, _, sprite) in collisions {
            info!("{:#?}", sprite.transform)
        }
    }
}
