use bevy::prelude::*;

#[derive(Component)]
struct RectFactors {
    entity: Entity,
    angle: f32,
    speed: f32,
    max_x: f32,
    min_x: f32,
    max_y: f32,
    min_y: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_rectangle)
        .run();
}

fn setup(mut commands: Commands) {
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

    let id = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 1., 0.),
                custom_size: Some(Vec2::new(50., 50.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        })
        .id();

    let corner1 = Vec2::new(-bounds_x, bounds_x);
    let corner2 = Vec2::new(bounds_x, -bounds_y);

    commands.spawn(RectFactors {
        entity: id,
        angle: 0.,
        speed: 5.,
        max_x: corner2.x,
        min_x: corner1.x,
        max_y: corner1.y,
        min_y: corner2.y,
    });
}

fn move_rectangle(
    mut rect_query: Query<(Entity, &mut Transform, &mut Sprite)>,
    mut factors_query: Query<(Entity, &mut RectFactors)>,
) {
    let mut factors = factors_query.single_mut().1;

    let bound_factor = 25. + factors.speed;

    for (entity, mut transform, mut sprite) in rect_query.iter_mut() {
        if entity == factors.entity {
            if transform.translation.x + bound_factor >= factors.max_x
                || transform.translation.x - bound_factor <= factors.min_x
                || transform.translation.y + bound_factor >= factors.max_y
                || transform.translation.y - bound_factor <= factors.min_y
            {
                factors.angle = 180. + factors.angle;
            }

            transform.translation += Vec3::new(
                factors.speed * factors.angle.cos(),
                factors.speed * factors.angle.sin(),
                0.,
            );

            factors.angle += factors.speed / 100000.;

            if sprite.color.r() == 1. && 0. < sprite.color.g() {
                sprite.color += Color::rgb(0., -0.01, 0.01)
            } else if sprite.color.b() == 1. && 0. < sprite.color.r() {
                sprite.color += Color::rgb(-0.01, 0.01, 0.)
            } else if sprite.color.g() == 1. && 0. < sprite.color.b() {
                sprite.color += Color::rgb(0.01, 0., -0.01)
            }

            if sprite.color.r() > 1. {
                sprite.color = Color::rgb(1., sprite.color.g(), sprite.color.b());
            }

            if sprite.color.g() > 1. {
                sprite.color = Color::rgb(sprite.color.r(), 1., sprite.color.b());
            }
            if sprite.color.b() > 1. {
                sprite.color = Color::rgb(sprite.color.r(), sprite.color.g(), 1.);
            }

            break;
        }
    }
}
