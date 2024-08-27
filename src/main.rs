use bevy::{
    math::f32::Vec2,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::prelude::*;

const COLORS: usize = 6;
#[derive(Component)]
struct Velocity(Vec2);
#[derive(Component)]
struct Force(Vec2);
#[derive(Component)]
struct Type(usize);
#[derive(Resource)]
struct ForceMap([[f32; COLORS]; COLORS]);
pub struct LifePlugin;

impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        let mut rng = rand::thread_rng();
        let mut fm: [[f32; COLORS]; COLORS] = [
            [1.0, -1.0, 1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, -1.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, -1.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0, -1.0, 1.0],
            [1.0, 0.0, 0.0, 0.0, 1.0, -1.0],
            [-1.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        ];
        for i in 0..COLORS {
            for j in 0..COLORS {
                fm[i][j] = rng.gen_range(-1.0..1.0);
            }
        }
        app.insert_resource(ForceMap(fm));
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

    let circle = Mesh2dHandle(meshes.add(Circle { radius: 2.0 }));

    for i in 0..COLORS {
        let color = materials.add(Color::hsl(i as f32 * 360.0 / COLORS as f32, 0.95, 0.7));
        for _j in 1..500 {
            let x = rng.gen_range(0.0..size.x) - size.x / 2.0;
            let y = rng.gen_range(0.0..size.y) - size.y / 2.0;
            let dx = rng.gen_range(-5.0..5.0);
            let dy = rng.gen_range(-5.0..5.0);
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: circle.clone(),
                    material: color.clone(),
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                Velocity(Vec2 { x: dx, y: dy }),
                Force(Vec2 { x: 0.0, y: 0.0 }),
                Type(i),
            ));
        }
    }
}

fn calculate_force(dist: f32, min_dist: f32, max_dist: f32, strength: f32) -> f32 {
    if dist <= min_dist {
        return dist / min_dist - 1.0;
    } else if dist <= max_dist {
        let x = (2.0 * dist - max_dist - min_dist).abs() / (max_dist - min_dist);
        return strength * (1.0 - x);
    } else {
        return 0.0;
    }
}

fn calculate_forces(
    time: Res<Time>,
    fm: Res<ForceMap>,
    mut query: Query<(&Transform, &mut Force, &Type)>,
) {
    let max_distance = 60.0;
    let min_distance = 20.0;
    let force_strength = 2.0;
    let dt = time.delta_seconds();
    let mut iter = query.iter_combinations_mut();
    while let Some([(t1, mut f1, Type(type1)), (t2, mut f2, Type(type2))]) = iter.fetch_next() {
        let distance = t1.translation.distance(t2.translation);
        if distance < max_distance {
            let s1 = fm.0[*type1][*type2];
            let s2 = fm.0[*type2][*type1];
            let cf1 = calculate_force(distance, min_distance, max_distance, s1);
            let cf2 = calculate_force(distance, min_distance, max_distance, s2);
            let cfv1 = (t2.translation - t1.translation).normalize() * force_strength * cf1 * dt;
            let cfv2 = (t1.translation - t2.translation).normalize() * force_strength * cf2 * dt;
            f1.0.x += cfv1.x;
            f1.0.y += cfv1.y;
            f2.0.x += cfv2.x;
            f2.0.y += cfv2.y;
        }
    }
}

fn update(
    time: Res<Time>,
    windows: Query<&Window>,
    mut query: Query<(&mut Velocity, &mut Force, &mut Transform)>,
) {
    let window = windows.single();
    let max_extent = 0.5 * window.size();
    let dt = time.delta_seconds();
    for (mut vel, mut force, mut trans) in &mut query {
        // friction
        vel.0.x *= 1.0 - 0.3 * dt;
        vel.0.y *= 1.0 - 0.3 * dt;
        vel.0.x += force.0.x;
        vel.0.y += force.0.y;
        force.0.x = 0.0;
        force.0.y = 0.0;
        trans.translation.x += vel.0.x * dt;
        trans.translation.y += vel.0.y * dt;
        if trans.translation.x.abs() > max_extent.x {
            vel.0.x *= -1.0;
            let delta = (trans.translation.x.abs() - max_extent.x).copysign(trans.translation.x);
            trans.translation.x -= delta;
        }
        if trans.translation.y.abs() > max_extent.y {
            vel.0.y *= -1.0;
            let delta = (trans.translation.y.abs() - max_extent.y).copysign(trans.translation.y);
            trans.translation.y -= delta;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LifePlugin)
        .run();
}
