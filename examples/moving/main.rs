//! This example illustrates the portal and destination moving, and tests for hierarchy

use bevy::prelude::*;
use bevy_basic_portals::*;

const PORTAL_TRANSLATION_START: Vec3 = Vec3::ZERO;
const PORTAL_TRANSLATION_END: Vec3 = Vec3::new(3., 3., 0.);

const DESTINATION_TRANSLATION_START: Vec3 = Vec3::new(20., 0., 0.);
const DESTINATION_TRANSLATION_END: Vec3 = Vec3::new(17., -3., 0.);

// Not sure what the camera scale is supposed to do? No visible difference
const CAMERA_SCALE_START: Vec3 = Vec3::ONE;
const CAMERA_SCALE_END: Vec3 = Vec3::new(2., 2., 2.);

const SPHERE_TRANSFORM: Transform = Transform::from_xyz(20., 0., -5.);

const TIME0: u128 = 0;
const TIME1: u128 = 1000;
const TIME2: u128 = 2000;
const TIME3: u128 = 3000;
const TIME4: u128 = 4000;
const TIME5: u128 = 5000;
const TIME6: u128 = 6000;
const TIME7: u128 = 7000;
const TIME8: u128 = 8000;
const TIME9: u128 = 8500;
const TIME10: u128 = 9000;
const TIME_STOP: u128 = 9000;

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PortalsPlugin::MINIMAL)
        .add_startup_system(setup)
        .add_system(move_portal_and_destination)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let camera_transform: Transform =
        Transform::from_xyz(-20., 0., 20.).looking_at(Vec3::ZERO, Vec3::Y);

    let main_camera = commands
        .spawn((
            Camera3dBundle {
                transform: camera_transform,
                ..default()
            },
            MainCamera,
        ))
        .id();

    let portal_mesh = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(10., 10.))));
    commands.spawn(CreatePortalBundle {
        mesh: portal_mesh,
        create_portal: CreatePortal {
            main_camera: Some(main_camera),
            debug: Some(DebugPortal {
                show_window: false,
                ..default()
            }),
            ..default()
        },
        ..default()
    });

    let sphere_mesh = meshes.add(Mesh::from(shape::UVSphere {
        radius: 2.,
        ..default()
    }));
    commands.spawn(PbrBundle {
        mesh: sphere_mesh,
        transform: SPHERE_TRANSFORM,
        ..default()
    });
}

fn move_portal_and_destination(
    time: Res<Time>,
    mut portal_query: Query<
        &mut Transform,
        (
            With<Portal>,
            Without<PortalDestination>,
            Without<MainCamera>,
        ),
    >,
    mut destination_query: Query<
        &mut Transform,
        (
            With<PortalDestination>,
            Without<Portal>,
            Without<MainCamera>,
        ),
    >,
    mut camera_query: Query<
        &mut Transform,
        (
            With<MainCamera>,
            Without<PortalDestination>,
            Without<Portal>,
        ),
    >,
) {
    let portal_rotation_start: Quat = Quat::from_axis_angle(Vec3::Y, 0.);
    let portal_rotation_end: Quat = Quat::from_axis_angle(Vec3::Y, 0.5);
    let destination_rotation_start: Quat = Quat::from_axis_angle(Vec3::Y, 0.);
    let destination_rotation_end: Quat = Quat::from_axis_angle(Vec3::Y, 0.5);

    let mut portal_transform = portal_query.get_single_mut().unwrap();
    let mut destination_transform = destination_query.get_single_mut().unwrap();
    let mut camera_transform = camera_query.get_single_mut().unwrap();

    let time = time.elapsed().as_millis() % TIME_STOP;
    let (portal_translation, portal_rotation, destination_translation, destination_rotation, camera_scale) = 
        // Portal translation
        if (TIME0..TIME1).contains(&time) {
            (
                PORTAL_TRANSLATION_START.lerp(
                    PORTAL_TRANSLATION_END, percent_from_to(time, TIME0, TIME1)
                ),
                portal_rotation_start,
                DESTINATION_TRANSLATION_START,
                destination_rotation_start,
                CAMERA_SCALE_START
            )
        }
        // Destination translation
        else if (TIME1..TIME2).contains(&time) {
            (
                PORTAL_TRANSLATION_END,
                portal_rotation_start,
                DESTINATION_TRANSLATION_START.lerp(
                    DESTINATION_TRANSLATION_END, percent_from_to(time, TIME1, TIME2)
                ),
                destination_rotation_start,
                CAMERA_SCALE_START
            )
        }
        // Portal rotation
        else if (TIME2..TIME3).contains(&time) {
            (
                PORTAL_TRANSLATION_END,
                portal_rotation_start.lerp(
                    portal_rotation_end, percent_from_to(time, TIME2, TIME3)
                ),
                DESTINATION_TRANSLATION_END,
                destination_rotation_start,
                CAMERA_SCALE_START
            )
        }
        // Destination rotation
        else if (TIME3..TIME4).contains(&time) {
            (
                PORTAL_TRANSLATION_END,
                portal_rotation_end,
                DESTINATION_TRANSLATION_END,
                destination_rotation_start.lerp(
                    destination_rotation_end, percent_from_to(time, TIME3, TIME4)
                ),
                CAMERA_SCALE_START
            )
        }
        // Portal reverse translation
        else if (TIME4..TIME5).contains(&time) {
            (
                PORTAL_TRANSLATION_END.lerp(
                    PORTAL_TRANSLATION_START, percent_from_to(time, TIME4, TIME5)
                ),
                portal_rotation_end,
                DESTINATION_TRANSLATION_END,
                destination_rotation_end,
                CAMERA_SCALE_START
            )
        }
        // Destination reverse translation
        else if (TIME5..TIME6).contains(&time) {
            (
                PORTAL_TRANSLATION_START,
                portal_rotation_end,
                DESTINATION_TRANSLATION_END.lerp(
                    DESTINATION_TRANSLATION_START, percent_from_to(time, TIME5, TIME6)
                ),
                destination_rotation_end,
                CAMERA_SCALE_START
            )
        }
        // Portal reverse rotation
        else if (TIME6..TIME7).contains(&time) {
            (
                PORTAL_TRANSLATION_START,
                portal_rotation_end.lerp(
                    portal_rotation_start, percent_from_to(time, TIME6, TIME7)
                ),
                DESTINATION_TRANSLATION_START,
                destination_rotation_end,
                CAMERA_SCALE_START
            )
        }
        // Destination reverse rotation
        else if (TIME7..TIME8).contains(&time) {
            (
                PORTAL_TRANSLATION_START,
                portal_rotation_start,
                DESTINATION_TRANSLATION_START,
                destination_rotation_end.lerp(
                    destination_rotation_start, percent_from_to(time, TIME7, TIME8)
                ),
                CAMERA_SCALE_START
            )
        }
        // Camera scale up
        else if (TIME8..TIME9).contains(&time) {
            (
                PORTAL_TRANSLATION_START,
                portal_rotation_start,
                DESTINATION_TRANSLATION_START,
                destination_rotation_start,
                CAMERA_SCALE_START.lerp(
                    CAMERA_SCALE_END, percent_from_to(time, TIME8, TIME9)
                )
            )
        }
        // Camera scale down
        else if (TIME9..TIME10).contains(&time) {
            (
                PORTAL_TRANSLATION_START,
                portal_rotation_start,
                DESTINATION_TRANSLATION_START,
                destination_rotation_start,
                CAMERA_SCALE_END.lerp(
                    CAMERA_SCALE_START, percent_from_to(time, TIME9, TIME10)
                )
            )
        }
        else {
            (portal_transform.translation, portal_transform.rotation, destination_transform.translation, destination_transform.rotation, camera_transform.scale)
        };

    portal_transform.translation = portal_translation;
    destination_transform.translation = destination_translation;
    portal_transform.rotation = portal_rotation;
    destination_transform.rotation = destination_rotation;
    camera_transform.scale = camera_scale;
}

fn percent_from_to(time: u128, begin: u128, stop: u128) -> f32 {
    ((time - begin) as f32) / ((stop - begin) as f32)
}
