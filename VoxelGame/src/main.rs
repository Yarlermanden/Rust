use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::f32::consts::PI;

mod voxel;
use crate::voxel::GrowingVoxel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "VoxelGame".into(),
                    resolution: (1280.0, 720.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }))
        .add_plugins(WorldInspectorPlugin::new())
        
        //systems
        //.add_systems(Startup, setup)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_basic_scene)
        .add_systems(Update, character_movement)
        .add_systems(Update, grow_voxel)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    //commands.spawn(Camera3dBundle::default());
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0, ..default() })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    }).insert(Name::new("Ground"));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0, ..default() })),
        material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    .insert(GrowingVoxel { ..default()})
    .insert(Name::new("Cube"));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    }).insert(Name::new("Light"));
}

fn grow_voxel(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut growing_voxels: Query<&mut GrowingVoxel>,
    time: Res<Time>,
) {
    for mut voxel in &mut growing_voxels  {
        voxel.timer.tick(time.delta());
        if voxel.timer.just_finished() {
            let spawn_transform = 
                Transform::from_xyz(0.0, 0.7, 0.6)
                .with_rotation(Quat::from_rotation_y(-PI / 2.0));

            commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1, ..default() })),
                    material: materials.add(Color::rgb(0.87, 0.44, 0.42).into()),
                    transform: spawn_transform,
                    ..default()
            })
            .insert(GrowingVoxel { ..default() })
            .insert(Name::new("Voxel"));
        }
    }
}







//fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//    commands.spawn(Camera2dBundle::default());
//
//    let texture = asset_server.load("character.png");
//
//    commands.spawn(SpriteBundle {
//        sprite: Sprite {
//            custom_size: Some(Vec2::new(100.0, 100.0)),
//            ..default()
//        },
//        texture,
//        ..default()
//    });
//}

fn character_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut characters{
        if input.pressed(KeyCode::W) {
            transform.translation.y += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= 100.0 * time.delta_seconds();
        }
    }
}