use bevy::{
    math::f32::Vec2,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let circle = Mesh2dHandle(meshes.add(Circle { radius: 50.0 }));
    let color = Color::hsl(0.0, 0.95, 0.7);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: circle,
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                0.0, 0.0, 0.0,
            ),
            ..default()
        },
        Velocity(Vec2 { x: 10.0, y: 50.0 }),
    ));
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
        }
        if trans.translation.y.abs() > max_extent.y {
            vel.0.y *= -1.0;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LifePlugin)
        .run();
}
