use bevy::{
    app::AppExit,
    input::{
        keyboard::*,
        mouse::*
    },
    math::*,
    prelude::*,
    render::{
        camera::*,
        pass::ClearColor
    }
};

mod sphere;
mod surface;

use surface::*;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Bevy Surface".to_string(),
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.15, 0.1, 0.5)))
        .add_default_plugins()
        .add_startup_system(setup.system())
        .init_resource::<CursorState>()
        .init_resource::<MouseClickState>()
        .init_resource::<MouseMotionState>()
        .init_resource::<KeyboardState>()
        .add_system(mouse_motion_system.system())
        .add_system(keyboard_system.system())
        .add_system(cursor_system.system())
        .add_system(mouse_click_system.system())
        .add_system(mesh_system.system())
        .run();
}

#[derive(Default)]
struct MouseClickState {
    reader: EventReader<MouseButtonInput>
}

fn mouse_click_system(
    mut state: ResMut<MouseClickState>,
    events: Res<Events<MouseButtonInput>>,
    cursor: Res<CursorState>,
    mut query: Query<&mut Mero>
) {
    let real_pos = cursor.position / 500. - Vec2::new(1.0, 1.0);
    for event in state.reader.iter(&events) {
        for mut mero in &mut query.iter() {
            if event.state == ElementState::Released {
                match event.button {
                    MouseButton::Left => {
                        println!("Adding a zero at position: {}", real_pos);
                        let z = to_complex(real_pos);
                        mero.zeros.push(z);
                    }
                    MouseButton::Right => {
                        println!("Adding a pole at position: {}", real_pos);
                        let z = to_complex(real_pos);
                        mero.poles.push(z);
                    }
                    _ => {}
                }
            }
        }
    }
}

#[derive(Default)]
struct CursorState {
    reader: EventReader<CursorMoved>,
    position: Vec2
}

fn cursor_system(
    mut state: ResMut<CursorState>,
    events: Res<Events<CursorMoved>>
) {
    for event in state.reader.iter(&events) {
        state.position = event.position
    }
}

#[derive(Default)]
struct MouseMotionState {
    reader: EventReader<MouseMotion>
}

fn mouse_motion_system(
    mut state: ResMut<MouseMotionState>,
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
    mut query: Query<(Changed<Mero>, &Surface, &Handle<Mesh>)>
) {
    for (mero, surface, handle) in &mut query.iter() {
        if let Some(mesh) = meshes.get_mut(handle) {
            println!("Updating mesh");
            let pol = |z| poly(&mero, z);
            // TODO: do not recreate the mesh, only update the uvs
            let solid_mesh = surface_to_solid(&surface, pol);
            *mesh = solid_mesh;
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
        .load_sync(&mut textures, "assets/periodic.png")
        .unwrap();

    let pol = |z| poly(&Mero::new(), z);
    let bound = Vec2::new(2., 2.);
    let plane_surface = parametric_surface(-bound, bound, [200, 200], plane);

    let solid_mesh = meshes.add(surface_to_solid(&plane_surface, pol));
    let solid_material = materials.add(texture_handle.into());
    let solid_plane = PbrComponents {
        mesh: solid_mesh,
        material: solid_material,
        translation: Translation::new(-2.0, 0.5, 0.0),
        ..Default::default()
    };

    let sphere_surface = parametric_surface(-bound, bound, [200, 200], sphere::north_chart);

    let solid_mesh = meshes.add(surface_to_solid(&sphere_surface, pol));
    let solid_material = materials.add(texture_handle.into());
    let solid_sphere = PbrComponents {
        mesh: solid_mesh,
        material: solid_material,
        translation: Translation::new(4.0, 0.5, 0.0),
        ..Default::default()
    };

    let camera_pos = Vec3::new(0., 0., 10.);

    commands
        .spawn(solid_plane)
        .with(plane_surface)
        .with(Mero::new())
        .spawn(solid_sphere)
        .with(sphere_surface)
        .with(Mero::new())
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
            translation: Translation::new(2.0, 2.0, 4.0),
            ..Default::default()
        })
        ;
}
