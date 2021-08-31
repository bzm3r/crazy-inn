use bevy::prelude::*;
use rand::prelude::*;
use std::collections::VecDeque;
use std::iter::FromIterator;
use wasm_bindgen::prelude::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Id(u32);

struct WaitQ(VecDeque<Id>);
struct Priority(Option<Id>);
struct IsServing(bool);
struct Score(f32);
struct ScoreText;

const NUM_DINERS: u32 = 20;
const WIN_HEIGHT: f32 = 800.0;
const WIN_WIDTH: f32 = 800.0;
const BDRY_OFFSET: f32 = 48.0;

#[derive(Bundle)]
struct ServerBundle {
    priorities: Priority,
    is_serving: IsServing,
    serve_timer: Timer,
}

#[derive(Bundle)]
struct DinerBundle {
    id: Id,
    score: Score,
    decay_timer: Timer,
}

struct Scoreboard {
    score: u32,
    min: u32,
}

#[wasm_bindgen(start)]
pub fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Crazy Inn".to_string(),
            width: WIN_WIDTH,
            height: WIN_HEIGHT,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WaitQ(VecDeque::from_iter(
            (0..NUM_DINERS).map(|x| Id(x)),
        )))
        .insert_resource(Scoreboard { score: 0, min: 0 })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_webgl2::WebGL2Plugin)
        .add_startup_system(setup)
        .add_system(decay_score)
        .add_system(serve_diners)
        .add_system(scoreboard_system)
        .run();
}

fn decay_score(
    time: Res<Time>,
    mut scoreboard: ResMut<Scoreboard>,
    mut q_diner: Query<(&mut Score, &mut Timer, &Children)>,
    mut q_txt: Query<&mut Text>,
) {
    let mut updated = false;
    let mut total = 0;
    let mut min = u32::MAX;
    for (mut score, mut timer, children) in q_diner.iter_mut() {
        if min > score.0.round() as u32 {
            min = score.0.round() as u32;
        }
        if timer.tick(time.delta()).just_finished() {
            updated = true;
            let new = score.0 * 0.95;
            score.0 = new;
            if min > score.0.round() as u32 {
                min = score.0.round() as u32;
            }
            total += new.round() as u32;
            for child in children.iter() {
                let mut text = q_txt.get_mut(*child).unwrap();
                text.sections[0].value =
                    (new.round() as u32).to_string();
            }
        }
    }
    if updated {
        scoreboard.score = total;
        scoreboard.min = min;
    }
}

fn scoreboard_system(
    scoreboard: Res<Scoreboard>,
    mut q_score: Query<&mut Text, With<ScoreText>>,
) {
    let mut text = q_score.single_mut().unwrap();
    text.sections[0].value = format!(
        "Score: {}, Min: {}",
        (scoreboard.score as f32 / NUM_DINERS as f32).round() as u32,
        scoreboard.min
    );
}

fn serve_diners(
    time: Res<Time>,
    mut wait_q: ResMut<WaitQ>,
    mut q_servers: Query<(
        &mut Priority,
        &mut IsServing,
        &mut Timer,
        &mut GlobalTransform,
    )>,
    mut q_diners: Query<(
        &Id,
        &mut Score,
        &GlobalTransform,
        Without<Priority>,
    )>,
) {
    for (mut priority, mut is_serving, mut timer, mut s_trans) in
        q_servers.iter_mut()
    {
        match (priority.0, is_serving.0) {
            (Some(p_id), false) => {
                let s_pos = s_trans.translation;
                for (d_id, _, d_trans, without_priority) in
                    q_diners.iter_mut()
                {
                    if p_id == *d_id && without_priority {
                        let d_pos = d_trans.translation;
                        let dist_sqd = d_pos.distance(s_pos);
                        if dist_sqd < 1e-1 {
                            is_serving.0 = true;
                            timer.tick(time.delta());
                        } else {
                            let dir = (d_pos - s_pos).normalize();
                            let mag = (5.0_f32 * 5.0).min(dist_sqd);
                            s_trans.translation =
                                s_pos + 0.1 * mag * dir;
                        }
                        break;
                    }
                }
            }
            (Some(p_id), true) => {
                is_serving.0 =
                    !timer.tick(time.delta()).just_finished();
                if !is_serving.0 {
                    for (d_id, mut score, _, without_priority) in
                        q_diners.iter_mut()
                    {
                        if p_id == *d_id && without_priority {
                            score.0 += 25.0;
                            wait_q.0.push_back(p_id);
                            priority.0 = None;
                        }
                    }
                }
            }
            (None, _) => match wait_q.0.pop_front() {
                Some(id) => {
                    priority.0 = Some(id);
                    info!("getting diner to serve: {:?}", id);
                }
                None => {
                    info!("no diner to serve...");
                }
            },
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
    commands.spawn_bundle(UiCameraBundle::default());

    let server_icon = asset_server.load("shield-powerup.png");
    let diner_icon = asset_server.load("gun-powerup.png");

    let mut rng = rand::thread_rng();
    let init_score: f32 = 100.0;
    for x in 0..NUM_DINERS {
        let diner_pos = Vec2::new(
            -1.0 * WIN_WIDTH * 0.5
                + BDRY_OFFSET
                + (WIN_WIDTH - BDRY_OFFSET * 2.0) * rng.gen::<f32>(),
            -1.0 * WIN_HEIGHT * 0.5
                + BDRY_OFFSET
                + (WIN_HEIGHT - BDRY_OFFSET * 2.0) * rng.gen::<f32>(),
        );
        info!("diner_pos: {}, {}", diner_pos.x, diner_pos.y);
        let diner_id = commands
            .spawn_bundle(DinerBundle {
                id: Id(x),
                score: Score(init_score),
                decay_timer: Timer::from_seconds(2.0, true),
            })
            .insert_bundle(SpriteBundle {
                material: materials.add(diner_icon.clone().into()),
                transform: Transform::from_translation(Vec3::new(
                    diner_pos.x,
                    diner_pos.y,
                    0.0,
                )),
                ..Default::default()
            })
            .id();

        let text_id =
            commands
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        (init_score.round() as u32).to_string(),
                        text_style.clone(),
                        text_alignment.clone(),
                    ),
                    transform: Transform::from_translation(
                        Vec3::new(0.0, 0.0 - 24.0, 0.0),
                    ),
                    ..Default::default()
                })
                .id();

        commands.entity(diner_id).push_children(&[text_id]);
    }

    for n in 0..3 {
        let server_pos =
            Vec2::new(0.0 + (n as f32) * 24.0, 0.0 * WIN_HEIGHT);
        commands
            .spawn_bundle(ServerBundle {
                priorities: Priority(None),
                is_serving: IsServing(false),
                serve_timer: Timer::from_seconds(2.0, true),
            })
            .insert_bundle(SpriteBundle {
                material: materials.add(server_icon.clone().into()),
                transform: Transform::from_translation(Vec3::new(
                    server_pos.x,
                    server_pos.y,
                    0.0,
                )),
                ..Default::default()
            });
    }

    // scoreboard
    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load(
                                "fonts/Inconsolata-Regular.ttf",
                            ),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load(
                                "fonts/Inconsolata-Regular.ttf",
                            ),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreText);
}
