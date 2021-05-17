use bevy::{prelude::*, gltf::Gltf};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(FlyCamera::default());
}

struct LoadingState {
    pub handle: Handle<Gltf>,
}

fn load_model(asset_server: Res<AssetServer>, mut commands: Commands) {
    let state = LoadingState  { handle: asset_server.load("sinsack.glb") };
    commands.insert_resource(state);
}

fn check_loaded_model(state: Res<LoadingState>, gltfs: Res<Assets<Gltf>>) {
    if let Some(gltf) = gltfs.get(state.handle.clone()) {
        info!("{:?}", gltf);
    } else {
        error!("Not loaded");
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(load_model.system())
        .add_system(check_loaded_model.system())
        .add_plugin(FlyCameraPlugin)
        .run();
}
