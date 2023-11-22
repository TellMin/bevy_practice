use bevy::{
    app::{App, PluginGroup, Startup},
    asset::Assets,
    core_pipeline::core_3d::Camera3dBundle,
    ecs::system::{Commands, ResMut},
    math::Vec3,
    pbr::{PbrBundle, PointLight, PointLightBundle, StandardMaterial},
    render::{
        color::Color,
        mesh::{shape, Mesh},
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::{Image, ImagePlugin},
    },
    transform::components::Transform,
    utils::default,
    DefaultPlugins,
};

use bevy_xpbd_3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // カメラを追加
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    let sphere = meshes.add(shape::UVSphere::default().into());

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(1.0), // 1.0はColliderの半径
        PbrBundle {
            mesh: sphere,
            material: debug_material.clone(),
            // このxyzはカメラの向きと同じ
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
    ));

    let side_len = 10.0;

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(side_len, 0.002, side_len),
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(side_len).into()),
            material: materials.add(Color::SILVER.into()),
            transform: Transform::from_xyz(0., -1.5, 0.),
            ..default()
        },
    ));
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}
