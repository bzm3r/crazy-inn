use bevy::prelude::*;
use rand::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Name(u32);
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
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(decay_score)
        .run();
}

fn decay_score(
    q_diner: Query<(&mut Score, &Children)>,
    q_txt: Query<&mut Text>,
) {
    for (diner, children) in q_diner.iter() {
        let current = diner.score;
        let new = current.0 * 0.05;
        diner.score = Score(new);
        for &child in children.iter() {
            let mut text = q_txt.get(child).unwrap();
            text.sections[0].value = new.into_string();
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("fonts/Inconsolata-Regular.ttf");
    let text_style = TextStyle {
        font,
        font_size: 30.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //let server_icon = asset_server.load("shield-powerup.png");
    let diner_icon = asset_server.load("gun-powerup.png");

    // for (n, name) in
    //     <_>::into_iter(["Erin", "Lyon", "Ishkr"]).enumerate()
    // {
    //     let server_pos = Vec2::new(
    //         0.5 * 640.0 + (n as f32) * 24.0,
    //         240.0,
    //     );
    //     commands
    //         .spawn_bundle(ServerBundle {
    //             name: Name(name.to_string()),
    //             pos: Pos(server_pos),
    //             vel: Vel(Vec2::default()),
    //             priorities: Priorities(BinaryHeap::new()),
    //             serving: Serving(false),
    //         })
    //         .insert_bundle(SpriteBundle {
    //             material: materials.add(server_icon.clone().into()),
    //             transform: {
    //                 let mut t = Transform::from_translation(
    //                     Vec3::new(server_pos.x, server_pos.y, 0.0),
    //                 );
    //                 // t.apply_non_uniform_scale(Vec3::new(
    //                 //     2.5, 2.5, 1.0,
    //                 // ));
    //                 t
    //             },
    //             ..Default::default()
    //         });
    // }

    let mut rng = rand::thread_rng();
    let init_score: f32 = 100.0;
    for x in 0..6 {
        let diner_pos = Vec2::new(
            -(320.0 + 24.0) + (640.0 - 24.0) * rng.gen::<f32>(),
            -(240.0 + 24.0) + (480.0 - 24.0) * rng.gen::<f32>(),
        );
        info!("diner_pos: {}, {}", diner_pos.x, diner_pos.y);
        let diner_id = commands
            .spawn_bundle(DinerBundle {
                name: Name(x),
                pos: Pos(diner_pos),
                score: Score(init_score),
            })
            .insert_bundle(SpriteBundle {
                material: materials.add(diner_icon.clone().into()),
                transform: {
                    let mut t = Transform::from_translation(
                        Vec3::new(diner_pos.x, diner_pos.y, 0.0),
                    );
                    // t.apply_non_uniform_scale(Vec3::new(
                    //     2.5, 2.5, 1.0,
                    // ));
                    t
                },
                ..Default::default()
            })
            .id();

        let text_id = commands
            .spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    &init_score.to_string(),
                    text_style.clone(),
                    text_alignment.clone(),
                ),
                transform: {
                    let mut t =
                        Transform::from_translation(Vec3::new(
                            diner_pos.x,
                            diner_pos.y - 24.0,
                            0.0,
                        ));
                    // t.apply_non_uniform_scale(Vec3::new(
                    //     2.5, 2.5, 1.0,
                    // ));
                    t
                },
                ..Default::default()
            })
            .id();

        commands.entity(diner_id).push_children(&[text_id]);
    }
}
