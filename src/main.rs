use bevy::{
    math::f32::Vec2,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::prelude::*;

#[derive(Component)]
struct Velocity(Vec2);
#[derive(Component)]
struct Force(Vec2);
pub struct LifePlugin;

impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update);
    }
}

fn setup(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let mut rng = rand::thread_rng();
    let window = windows.single();
    let size = 0.5 * window.size();

    let circle = Mesh2dHandle(meshes.add(Circle { radius: 5.0 }));
    let color = materials.add(Color::hsl(0.0, 0.95, 0.7));

    for i in 1..10 {
        let x = rng.gen_range(0.0..size.x) - size.x / 2.0;
        let y = rng.gen_range(0.0..size.y) - size.y / 2.0;
        let dx = rng.gen_range(0.0..50.0);
        let dy = rng.gen_range(0.0..50.0);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: circle.clone(),
                material: color.clone(),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            Velocity(Vec2 { x: dx, y: dy }),
        ));
    }
}

fn update(
    time: Res<Time>,
    windows: Query<&Window>,
    mut query: Query<(&mut Velocity, &mut Transform)>,
) {
    let window = windows.single();
    let max_extent = 0.5 * window.size();
    for (mut vel, mut trans) in &mut query {
        trans.translation.x += vel.0.x * time.delta_seconds();
        trans.translation.y += vel.0.y * time.delta_seconds();
        if trans.translation.x.abs() > max_extent.x {
            vel.0.x *= -1.0;
            let delta = trans.translation.x.abs() - max_extent.x;
            if trans.translation.x.is_sign_negative() {
                trans.translation.x += delta;
            } else {
                trans.translation.x -= delta;
            }
        }
        if trans.translation.y.abs() > max_extent.y {
            vel.0.y *= -1.0;
            let delta = trans.translation.y.abs() - max_extent.y;
            if trans.translation.y.is_sign_negative() {
                trans.translation.y += delta;
            } else {
                trans.translation.y -= delta;
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LifePlugin)
        .run();
}
