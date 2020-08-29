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
    triangles: Vec<[u32; 3]>
}

fn triangle_normal(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    (b - a).cross(c - a)
}

pub fn parametric_surface(count: u32, f: fn(f32, f32) -> Vec3) -> Surface {
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
            let norm1 = triangle_normal(o, a, b);
            let norm2 = triangle_normal(o, b, c);
            let norm3 = triangle_normal(o, c, d);
            let norm4 = triangle_normal(o, d, a);
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

pub fn surface_to_solid(surface: &Surface) -> Mesh {
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

pub fn surface_to_wireframe(surface: &Surface) -> Mesh {
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

// Concrete surfaces
pub fn wave(x: f32, y: f32) -> f32 {
    0.5 * f32::sin(TAU * x) * f32::sin(TAU * y)
}

pub fn planar(x: f32, y: f32, f: fn(f32, f32) -> f32) -> Vec3 {
    [x, y, f(x, y)].into()
}

pub fn torus(a: f32, b: f32, x: f32, y: f32) -> Vec3 {
    let pos = Vec3::new(a + b * f32::cos(TAU * y), 0.0, b * f32::sin(TAU * y));
    // rotate pos around the z-axis by TAU * x
    Mat3::from_rotation_z(TAU * x) * pos
}
