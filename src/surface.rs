use bevy::{
  math::*,
  prelude::*,
  render::{
    mesh::VertexAttribute,
    pipeline::PrimitiveTopology
  }
};

const TAU: f32 = 2.0 * std::f32::consts::PI;

pub struct Surface {
    positions: Vec<Vec3>,
    normals: Vec<Vec3>,
    uvs: Vec<[f32; 2]>,
    triangles: Vec<[u32; 3]>
}

fn triangle_normal(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    (b - a).cross(c - a)
}

pub fn parametric_surface<F, G>(count: u32, f: F, g: G) -> Surface
where
    F: Fn(Vec2) -> Vec3,
    G: Fn(Vec2) -> Vec2
{
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut triangles = Vec::new();

    let delta = 1. / count as f32;

    for i in 0..=count {
        for j in 0..=count {
            let y = i as f32 * delta;
            let x = j as f32 * delta;
            let z = Vec2::new(x, y);
            let x_delta = Vec2::new(delta, 0.0);
            let y_delta = Vec2::new(0.0, delta);
            let o = f(z);
            positions.push(o);
            let a = f(z + x_delta);
            let b = f(z + y_delta);
            let c = f(z - x_delta);
            let d = f(z - y_delta);
            let norm1 = triangle_normal(o, a, b);
            let norm2 = triangle_normal(o, b, c);
            let norm3 = triangle_normal(o, c, d);
            let norm4 = triangle_normal(o, d, a);
            let norm = (norm1 + norm2 + norm3 + norm4).normalize();
            normals.push(norm.into());
            uvs.push(g(z).into());
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
        uvs,
        triangles
    }
}

pub fn surface_to_solid(surface: &Surface) -> Mesh {
    let positions = surface.positions.iter().map(|p| (*p).into()).collect();
    let normals = surface.normals.iter().map(|n| (*n).into()).collect();
    let mut indices = Vec::new();
    for t in &surface.triangles {
        indices.extend_from_slice(t);
    }
    Mesh {
        primitive_topology: PrimitiveTopology::TriangleList,
        attributes: vec![
            VertexAttribute::position(positions),
            VertexAttribute::normal(normals),
            VertexAttribute::uv(surface.uvs.clone()),
        ],
        indices: Some(indices)
    }
}

pub fn surface_to_wireframe(surface: &Surface) -> Mesh {
    let positions = surface.positions.iter().map(|p| (*p).into()).collect();
    let normals = surface.normals.iter().map(|n| (*n).into()).collect();
    let mut indices = Vec::new();
    for t in &surface.triangles {
        indices.extend_from_slice(&[t[0], t[1], t[1], t[2], t[2], t[0]]);
    }
    Mesh {
        primitive_topology: PrimitiveTopology::LineList,
        attributes: vec![
            VertexAttribute::position(positions),
            VertexAttribute::normal(normals),
            VertexAttribute::uv(surface.uvs.clone()),
        ],
        indices: Some(indices)
    }
}

pub fn surface_to_point_cloud(surface: &Surface) -> Mesh {
    let positions = surface.positions.iter().map(|p| (*p).into()).collect();
    let normals = surface.normals.iter().map(|n| (*n).into()).collect();
    let mut indices = Vec::new();
    for t in &surface.triangles {
        indices.extend_from_slice(&[t[0], t[1], t[2]]);
    }
    Mesh {
        primitive_topology: PrimitiveTopology::PointList,
        attributes: vec![
            VertexAttribute::position(positions),
            VertexAttribute::normal(normals),
            VertexAttribute::uv(surface.uvs.clone()),
        ],
        indices: Some(indices)
    }
}

// Concrete surfaces
pub fn wave(z: Vec2) -> f32 {
    0.5 * f32::sin(TAU * z[0]) * f32::sin(TAU * z[1])
}

pub fn planar(z: Vec2, f: fn(Vec2) -> f32) -> Vec3 {
    z.extend(f(z))
}

pub fn torus(a: f32, b: f32, z: Vec2) -> Vec3 {
    let pos = Vec3::new(a + b * f32::cos(TAU * z[1]), 0.0, b * f32::sin(TAU * z[1]));
    Mat3::from_rotation_z(TAU * z[0]) * pos
}

// Functions
pub fn constant(c: Vec2, _z: Vec2) -> Vec2 { c }

pub fn identity(z: Vec2) -> Vec2 { z }

pub fn shift(c: Vec2, z: Vec2) -> Vec2 { Vec2::new((c.x() + z.x()) % 1.0, (c.y() + z.y()) % 1.0) }
