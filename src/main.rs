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
        app.add_systems(Update, (calculate_forces, update).chain());
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
    let size = window.size();

    let circle = Mesh2dHandle(meshes.add(Circle { radius: 5.0 }));

    for i in 1..1000 {
        let x = rng.gen_range(0.0..size.x) - size.x / 2.0;
        let y = rng.gen_range(0.0..size.y) - size.y / 2.0;
        let dx = rng.gen_range(-50.0..50.0);
        let dy = rng.gen_range(-50.0..50.0);
        let color = materials.add(Color::hsl(rng.gen_range(0.0..360.0), 0.95, 0.7));
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: circle.clone(),
                material: color.clone(),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            Velocity(Vec2 { x: dx, y: dy }),
            Force(Vec2 { x: 0.0, y: 0.0 }),
        ));
    }
}

fn calculate_forces(mut query: Query<(&mut Force, &mut Transform)>) {
    let mut rng = rand::thread_rng();
    for (mut force, trans) in &mut query {
        let dx = rng.gen_range(-1.0..1.0);
        let dy = rng.gen_range(-1.0..1.0);
        force.0.x += dx;
        force.0.y += dy;
    }
}

fn update(
    time: Res<Time>,
    windows: Query<&Window>,
    mut query: Query<(&mut Velocity, &mut Force, &mut Transform)>,
) {
    let window = windows.single();
    let max_extent = 0.5 * window.size();
    for (mut vel, mut force, mut trans) in &mut query {
        vel.0.x += force.0.x;
        vel.0.y += force.0.y;
        force.0.x = 0.0;
        force.0.y = 0.0;
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
