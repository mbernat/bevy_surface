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
    println!("Delta: {}", delta);
    for (_camera, mut pos, mut trans) in &mut query.iter() {
        let q = delta.extend(0.0);
        pos.0 += q / 100.;
        println!("New pos: {}", pos.0);
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let _planar_wave = |x, y| planar(x, y, wave);
    let torus = |x, y| torus(1.0, 0.4, x , y);
    let surface = parametric_surface(100, torus);
    let solid_mesh = surface_to_solid(&surface);
    let wireframe_mesh_handle = meshes.add(surface_to_wireframe(&surface));
    let solid_mesh_handle = meshes.add(solid_mesh);
    let solid_material = materials.add(Color::rgba(0.5, 0.1, 0.7, 0.5).into());
    let wireframe_material = materials.add(Color::BLACK.into());
    let solid = PbrComponents {
        mesh: solid_mesh_handle,
        material: solid_material,
        translation: Translation::new(0.5, 0.5, 0.0),
        ..Default::default()
    };
    let _wireframe = PbrComponents {
        mesh: wireframe_mesh_handle,
        material: wireframe_material,
        translation: Translation::new(0.5, 0.5, 0.0),
        ..Default::default()
    };
    let camera_pos = Vec3::new(0., 0., 4.);

    commands
        .spawn(solid)
        //.spawn(wireframe)
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
