use bevy::prelude::*;

#[derive(Component)]
struct Person {
    name: String,
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Person {
        name: "Saul".to_string(),
    });
    commands.spawn(Person {
        name: "Waltuh".to_string(),
    });
    commands.spawn(Person {
        name: "Jesse".to_string(),
    });
    commands.spawn(Camera2d);
    [
        meshes.add(Circle::new(50.0)),
        meshes.add(CircularSector::new(50.0, 1.0)),
        meshes.add(CircularSegment::new(50.0, 1.25)),
        meshes.add(Ellipse::new(25.0, 50.0)),
        meshes.add(Annulus::new(25.0, 50.0)),
        meshes.add(Capsule2d::new(25.0, 50.0)),
        meshes.add(Rhombus::new(75.0, 100.0)),
        meshes.add(Rectangle::new(50.0, 100.0)),
        meshes.add(RegularPolygon::new(50.0, 6)),
        meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        )),
    ]
    .into_iter()
    .enumerate()
    .for_each(|(i, mesh)| {
        commands.spawn((
            Mesh2d(mesh),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
            Transform::from_xyz((i * 100) as f32, 0., 0.),
        ));
    })
}

fn all_people(time: Res<Time>, mut timer: ResMut<TimedResource>, query: Query<&Person>) {
    if timer.0.tick(time.delta()).just_finished() {
        query
            .into_iter()
            .for_each(|Person { name }| println!("{name}"));
    }
}

// fn update_people(mut query: Query<&mut Person>) {
//     for mut person in &mut query {
//         person.name += "hello";
//     }
// }

#[derive(Resource)]
struct TimedResource(Timer);

struct People;

impl Plugin for People {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimedResource(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )));
        // add any function or tuple of functions whose arguments implement `SystemParams`
        app.add_systems(Startup, init)
            .add_systems(Update, all_people);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(People)
        .run();
}
