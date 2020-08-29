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
        mesh::{VertexAttribute},
        pipeline::{
            PrimitiveTopology,
        },
    },
};

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

const TAU: f32 = 2.0 * std::f32::consts::PI;

fn wave(x: f32, y: f32) -> f32 {
    0.5 * f32::sin(TAU * x) * f32::sin(TAU * y)
}

fn planar(x: f32, y: f32, f: fn(f32, f32) -> f32) -> Vec3 {
    [x, y, f(x, y)].into()
}

fn torus(a: f32, b: f32, x: f32, y: f32) -> Vec3 {
    let pos = Vec3::new(a + b * f32::cos(TAU * y), 0.0, b * f32::sin(TAU * y));
    // rotate pos around the z-axis by TAU * x
    Mat3::from_rotation_z(TAU * x) * pos
}

fn cross(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    (b - a).cross(c - a)
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

struct Surface {
    positions: Vec<Vec3>,
    normals: Vec<Vec3>,
    triangles: Vec<[u32; 3]>
}

fn parametric_surface(count: u32, f: fn(f32, f32) -> Vec3) -> Surface {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut triangles = Vec::new();

    let delta = 1. / count as f32;

    for i in 0..=count {
        for j in 0..=count {
            let y = i as f32 * delta;
            let x = j as f32 * delta;
            let o = f(x, y);
            positions.push(o.into());
            let a = f(x + delta, y);
            let b = f(x, y + delta);
            let c = f(x - delta, y);
            let d = f(x, y - delta);
            let norm1 = cross(o, a, b);
            let norm2 = cross(o, b, c);
            let norm3 = cross(o, c, d);
            let norm4 = cross(o, d, a);
            let norm = (norm1 + norm2 + norm3 + norm4).normalize();
            normals.push(norm.into());
            uvs.push([0.0, 0.0]);
        }
    }

    for i in 0..count {
        for j in 0..count {
            let bottom_left = i * (count + 1) + j;
            let bottom_right = i * (count + 1) + j + 1;
            let top_left = (i + 1) * (count + 1) + j;
            let top_right = (i + 1) * (count + 1) + j + 1;
            triangles.push([top_left, bottom_left, top_right]);
            triangles.push([top_right, bottom_left, bottom_right]);
        }
    }

    Surface {
        positions,
        normals,
        triangles
    }
}

fn surface_to_solid(surface: &Surface) -> Mesh {
    let positions = surface.positions.iter().map(|p| (*p).into()).collect();
    let normals = surface.normals.iter().map(|n| (*n).into()).collect();
    let uvs = surface.positions.iter().map(|_| [0.0, 0.0]).collect();
    let mut indices = Vec::new();
    for t in &surface.triangles {
        indices.extend_from_slice(t);
    }
    Mesh {
        primitive_topology: PrimitiveTopology::TriangleList,
        attributes: vec![
            VertexAttribute::position(positions),
            VertexAttribute::normal(normals),
            VertexAttribute::uv(uvs),
        ],
        indices: Some(indices)
    }
}

fn surface_to_wireframe(surface: &Surface) -> Mesh {
    let positions = surface.positions.iter().map(|p| (*p).into()).collect();
    let normals = surface.normals.iter().map(|n| (*n).into()).collect();
    let uvs = surface.positions.iter().map(|_| [0.0, 0.0]).collect();
    let mut indices = Vec::new();
    for t in &surface.triangles {
        indices.extend_from_slice(&[t[0], t[1], t[1], t[2], t[2], t[0]]);
    }
    Mesh {
        primitive_topology: PrimitiveTopology::LineList,
        attributes: vec![
            VertexAttribute::position(positions),
            VertexAttribute::normal(normals),
            VertexAttribute::uv(uvs),
        ],
        indices: Some(indices)
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
