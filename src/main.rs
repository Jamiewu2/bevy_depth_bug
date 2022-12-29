use bevy::{prelude::*, render::{render_resource::{SamplerDescriptor, FilterMode}, camera::ScalingMode}};
use std::f32::consts::TAU;
use bevy_inspector_egui::WorldInspectorPlugin;

//viewport dimensions in pixels
const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53))) 
    .add_plugins(DefaultPlugins
        .set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                //fit_canvas_to_parent: true,
                ..default()
            },
        ..default()
        })
        .set({ImagePlugin {default_sampler: SamplerDescriptor {
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest, //pixel effect
            // address_mode_u: AddressMode::Repeat,
            // address_mode_v: AddressMode::Repeat,
            // address_mode_w: AddressMode::Repeat, //repeat instead of stretch textures
            ..Default::default()
        }}})
        .set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        })
    )
    .add_plugin(WorldInspectorPlugin::new())
    .add_startup_system(setup)
    .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut t: Transform = Transform::from_translation(Vec3::ZERO);
    t.rotate_x(TAU/4.0);

    let texture_handle = asset_server.load("materials/checker3.png");
    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: false,
        ..default()
    });

    // plane
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 600.0 })),
        material: material_handle.clone(),
        transform: t,
        ..default()
    }).insert(Name::new("Ground"));

    //cube 2, standardMaterial
    // let cube_color = Color::rgb_linear(35./256., 121./256., 207./256.);
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 60.0 })),
        material: material_handle.clone(),
        //material: materials.add(cube_color.into()),
        transform: t.with_translation(Vec3::new(0.0,0.0,30.0)),
        ..default()
    }).insert(Name::new("Cube2"));

    // directional 'sun' light
    const HALF_SIZE: f32 = 1200.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::ZERO,
            //inits to -Z axis, this rotates about x axis by 40 degrees (into positive y),
            // then rotates -100 degrees about z (from positive y to positive x, and a bit more)
            rotation: Quat::from_euler(EulerRot::ZXY, -100f32.to_radians(), 40f32.to_radians(), 0.),
            ..default()
        },
        ..default()
    });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.50,
    });

    // camera
    setup_camera(commands);
}

fn setup_camera(mut commands: Commands) {
    const FRUSTUM_SIZE: f32 = 1200.;
    let camera_coords = Vec3::new(-306.186, -306.186, 250.0);

    // camera
    commands.spawn(
    Camera3dBundle {
        transform: Transform::from_translation(camera_coords).looking_at(Vec3::ZERO, Vec3::Z),
        projection: OrthographicProjection {
            scale: 1.0,
            left: -FRUSTUM_SIZE,
            right: FRUSTUM_SIZE,
            top: FRUSTUM_SIZE,
            bottom: FRUSTUM_SIZE,
            near: -10.0 * FRUSTUM_SIZE,
            far: 10.0 * FRUSTUM_SIZE,
            scaling_mode: ScalingMode::WindowSize,
            ..default()
        }.into(),
        ..default()
    });
}