use bevy::prelude::*;
use rand::Rng;

#[derive(Resource, Default)]
struct RectFactors {
    max_x: f32,
    min_x: f32,
    max_y: f32,
    min_y: f32,
}

#[derive(Resource, Default)]
struct SpawnFlags {
    spawn_rects: i32,
    color: Color,
}

#[derive(Component)]
struct MoveAbleRect {
    angle: f32,
    speed: f32,
    just_spawned: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(RectFactors { ..default() })
        .insert_resource(SpawnFlags {
            color: Color::rgb(1., 1., 0.),
            ..default()
        })
        .add_startup_system(setup)
        .add_system(move_rectangle)
        .add_system(spawn_rectangles)
        .run();
}

fn spawn_rectangles(mut commands: Commands, mut spawn_flags: ResMut<SpawnFlags>) {
    let mut rng = rand::thread_rng();

    for _i in 1..=spawn_flags.spawn_rects {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: spawn_flags.color,
                    custom_size: Some(Vec2::new(50., 50.)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            })
            .insert(MoveAbleRect {
                angle: rng.gen_range(0..=360) as f32,
                speed: rng.gen_range(1..=5) as f32,
                just_spawned: false,
            });
    }

    spawn_flags.spawn_rects = 0;
}

fn setup(mut commands: Commands, mut rect_factors: ResMut<RectFactors>) {
    commands.spawn(Camera2dBundle::default());

    let bounds_x = 400.;
    let bounds_y = 400.;

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            rect: Some(Rect {
                min: Vec2::new(-bounds_x, bounds_y),
                max: Vec2::new(bounds_x, -bounds_y),
            }),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });

    let mut rng = rand::thread_rng();
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 1., 0.),
                custom_size: Some(Vec2::new(50., 50.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        })
        .insert(MoveAbleRect {
            angle: rng.gen_range(0..=360) as f32,
            speed: rng.gen_range(1..=5) as f32,
            just_spawned: false,
        });

    let corner1 = Vec2::new(-bounds_x, bounds_x);
    let corner2 = Vec2::new(bounds_x, -bounds_y);
    rect_factors.max_x = corner2.x;
    rect_factors.min_x = corner1.x;
    rect_factors.max_y = corner1.y;
    rect_factors.min_y = corner2.y;
}

fn move_rectangle(
    mut rect_query: Query<(Entity, &mut Transform, &mut Sprite, &mut MoveAbleRect)>,
    rect_factors: Res<RectFactors>,
    mut spawn_flags: ResMut<SpawnFlags>,
) {
    if spawn_flags.color.r() == 1. && 0. < spawn_flags.color.g() {
        spawn_flags.color += Color::rgb(0., -0.01, 0.01)
    } else if spawn_flags.color.b() == 1. && 0. < spawn_flags.color.r() {
        spawn_flags.color += Color::rgb(-0.01, 0.01, 0.)
    } else if spawn_flags.color.g() == 1. && 0. < spawn_flags.color.b() {
        spawn_flags.color += Color::rgb(0.01, 0., -0.01)
    }

    if spawn_flags.color.r() > 1. {
        spawn_flags.color = Color::rgb(1., spawn_flags.color.g(), spawn_flags.color.b());
    }

    if spawn_flags.color.g() > 1. {
        spawn_flags.color = Color::rgb(spawn_flags.color.r(), 1., spawn_flags.color.b());
    }
    if spawn_flags.color.b() > 1. {
        spawn_flags.color = Color::rgb(spawn_flags.color.r(), spawn_flags.color.g(), 1.);
    }

    for (_entity, mut transform, mut sprite, mut moveable_rect) in rect_query.iter_mut() {
        let bound_factor = 25. + moveable_rect.speed;

        if transform.translation.x + bound_factor >= rect_factors.max_x
            || transform.translation.x - bound_factor <= rect_factors.min_x
            || transform.translation.y + bound_factor >= rect_factors.max_y
            || transform.translation.y - bound_factor <= rect_factors.min_y
        {
            moveable_rect.angle = 180. + moveable_rect.angle;

            if !moveable_rect.just_spawned {
                spawn_flags.spawn_rects += 1;
                moveable_rect.just_spawned = true;
            }
        } else {
            moveable_rect.just_spawned = false;
        }

        transform.translation += Vec3::new(
            moveable_rect.speed * moveable_rect.angle.cos(),
            moveable_rect.speed * moveable_rect.angle.sin(),
            0.,
        );

        sprite.color = spawn_flags.color;
    }
}
