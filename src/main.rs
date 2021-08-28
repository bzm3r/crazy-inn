use bevy::prelude::*;
use rand::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Name(String);
struct Pos(Vec2);
struct Vel(Vec2);

struct Task {
    priority: u32,
    diner: u32,
}
struct Priorities(BinaryHeap<Task>);

struct Serving(bool);
struct Diner;
struct Score(f32);

#[derive(Bundle)]
struct ServerBundle {
    name: Name,
    pos: Pos,
    vel: Vel,
    priorities: Priorities,
    serving: Serving,
}

#[derive(Bundle)]
struct DinerBundle {
    name: Name,
    pos: Pos,
    score: Score,
}

impl Eq for Task {}

impl PartialEq<Self> for Task {
    fn eq(&self, other: &Self) -> bool {
        if self.priority == other.priority {
            true
        } else {
            false
        }
    }
}

impl PartialOrd<Self> for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Crazy Inn".to_string(),
            width: 640.0,
            height: 480.0,
            vsync: true,
            ..Default::default()
        })
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let server_icon = asset_server.load("shield-powerup.png");
    // let diner_icon = asset_server.load("shield-powerup.png");

    let server_pos =
        Vec2::new(0.5 * 640.0 + (0 as f32) * 24.0, 240.0);
    commands
        .spawn_bundle(ServerBundle {
            name: Name("Test".to_string()),
            pos: Pos(server_pos),
            vel: Vel(Vec2::default()),
            priorities: Priorities(BinaryHeap::new()),
            serving: Serving(false),
        })
        .insert_bundle(SpriteBundle {
            material: materials.add(server_icon.into()),
            transform: {
                let mut t = Transform::from_translation(Vec3::new(
                    server_pos.x,
                    server_pos.y,
                    0.0,
                ));
                t.apply_non_uniform_scale(Vec3::new(2.5, 2.5, 1.0));
                t
            },
            ..Default::default()
        });

    // , "Lyon", "Ishkr"
    // for (n, name) in ["Erin"].into_iter().enumerate() {
    //     let server_pos =
    //         Vec2::new(0.5 * 640.0 + (n as f32) * 24.0, 240.0);
    //     commands
    //         .spawn_bundle(ServerBundle {
    //             name: Name(name.to_string()),
    //             pos: Pos(server_pos),
    //             vel: Vel(Vec2::default()),
    //             priorities: Priorities(BinaryHeap::new()),
    //             serving: Serving(false),
    //         })
    //         .insert_bundle(SpriteBundle {
    //             material: materials.add(server_icon.into()),
    //             transform: {
    //                 let mut t = Transform::from_translation(
    //                     Vec3::new(server_pos.x, server_pos.y, 0.0),
    //                 );
    //                 t.apply_non_uniform_scale(Vec3::new(
    //                     2.5, 2.5, 1.0,
    //                 ));
    //                 t
    //             },
    //             ..Default::default()
    //         });
    // }

    // let mut rng = rand::thread_rng();
    // for x in 0..1 {
    //     let diner_pos = Vec2::new(
    //         640.0_f32 * rng.gen::<f32>(),
    //         480.0_f32 * rng.gen::<f32>(),
    //     );
    //     commands
    //         .spawn_bundle(DinerBundle {
    //             name: Name(x.to_string()),
    //             pos: Pos(diner_pos),
    //             score: Score(100.0),
    //         })
    //         .insert_bundle(SpriteBundle {
    //             material: materials.add(diner_icon.clone().into()),
    //             transform: {
    //                 let mut t = Transform::from_translation(
    //                     Vec3::new(diner_pos.x, diner_pos.y, 0.0),
    //                 );
    //                 t.apply_non_uniform_scale(Vec3::new(
    //                     2.5, 2.5, 1.0,
    //                 ));
    //                 t
    //             },
    //             ..Default::default()
    //         });
    // }
}
