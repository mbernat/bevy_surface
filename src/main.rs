use bevy::{
    app::AppExit,
    input::{
        keyboard::*,
        mouse::*
    },
    math::*,
    prelude::*,
    render::camera::*,
};
use rand::*;

mod surface;
use surface::*;

/// This example illustrates how to create a mesh asset with a custom vertex format and a shader that uses that mesh
fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .init_resource::<MouseState>()
        .init_resource::<KeyboardState>()
        .add_system(mouse_system.system())
        .add_system(keyboard_system.system())
        //.add_system(mesh_system.system())
        .run();
}

#[derive(Default)]
struct MouseState {
    reader: EventReader<MouseMotion>
}

fn mouse_system(
    mut state: ResMut<MouseState>,
    events: Res<Events<MouseMotion>>,
    mut query: Query<(&Camera, &mut Translation, &mut Transform)>
) {
    let mut delta = Vec2::zero();
    for event in state.reader.iter(&events) {
        delta += event.delta;
    }
    delta.set_x(-delta.x());
    for (_camera, mut pos, mut trans) in &mut query.iter() {
        let q = delta.extend(0.0);
        pos.0 += q / 100.;
        *trans = Transform::new_sync_disabled(Mat4::face_toward(
            pos.0,
            Vec3::new(0.5, 0.5, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ));
    }
}

#[derive(Default)]
struct KeyboardState {
    reader: EventReader<KeyboardInput>
}

fn keyboard_system(
    mut state: ResMut<KeyboardState>,
    mut exit_events: ResMut<Events<AppExit>>,
    keyboard_events: Res<Events<KeyboardInput>>
) {
    for event in state.reader.iter(&keyboard_events) {
        if event.state == ElementState::Pressed && event.key_code == Some(KeyCode::Escape) {
            exit_events.send(AppExit)
        }
    }
}

fn mesh_system(
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<&Handle<Mesh>>
) {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(0.99, 1.01);
    let b = rng.gen_range(0.39, 0.41);
    let torus = |x, y| torus(a, b, x , y);
    let surface = parametric_surface(100, torus);
    let solid_mesh = surface_to_solid(&surface);
    for handle in &mut query.iter() {
        if let Some(mesh) = meshes.get_mut(handle) {
            *mesh = solid_mesh;
            break
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/rainbow2.png")
        .unwrap();

    let _planar_wave = |x, y| planar(x, y, wave);
    let torus = |x, y| torus(1.0, 0.4, x , y);
    let surface = parametric_surface(100, torus);

    let solid_mesh = meshes.add(surface_to_solid(&surface));
    let solid_material = materials.add(texture_handle.into());
    let _solid = PbrComponents {
        mesh: solid_mesh,
        material: solid_material,
        translation: Translation::new(0.5, 0.5, 0.0),
        ..Default::default()
    };

    let wireframe_mesh = meshes.add(surface_to_wireframe(&surface));
    let wireframe_material = materials.add(Color::BLACK.into());
    let _wireframe = PbrComponents {
        mesh: wireframe_mesh,
        material: wireframe_material,
        translation: Translation::new(0.5, 0.5, 0.0),
        ..Default::default()
    };

    let point_cloud_mesh = meshes.add(surface_to_point_cloud(&surface));
    let point_cloud_material = materials.add(Color::BLUE.into());
    let point_cloud = PbrComponents {
        mesh: point_cloud_mesh,
        material: point_cloud_material,
        translation: Translation::new(0.5, 0.5, 0.0),
        ..Default::default()
    };

    let camera_pos = Vec3::new(0., 0., 4.);

    commands
        //.spawn(solid)
        //.spawn(wireframe)
        .spawn(point_cloud)
        .spawn(Camera3dComponents {
            translation: Translation(camera_pos),
            transform: Transform::new_sync_disabled(Mat4::face_toward(
                camera_pos,
                Vec3::zero(),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        })
        .spawn(LightComponents {
            translation: Translation::new(2.0, 2.0, 2.0),
            ..Default::default()
        })
        ;
}
