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
    start: Vec2,
    end: Vec2,
    count: [u32; 2],
    positions: Vec<Vec3>,
    normals: Vec<Vec3>,
    triangles: Vec<[u32; 3]>
}

fn triangle_normal(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    (b - a).cross(c - a)
}

fn compute_uvs<F>(surface: &Surface, f: F) -> Vec<[f32; 2]>
where
    F: Fn(Vec2) -> Vec2
{
    let mut uvs = Vec::new();

    let start = surface.start;
    let end = surface.end;
    let count = surface.count;
    let diff = end - start;
    let delta_x = diff[0] / count[0] as f32;
    let delta_y = diff[1] / count[1] as f32;

    for i in 0..=count[0] {
        for j in 0..=count[1] {
            let x = start[0] + i as f32 * delta_x;
            let y = start[1] + j as f32 * delta_y;
            let z = Vec2::new(x, y);
            uvs.push(f(z).into());
        }
    }
    uvs
}

pub fn parametric_surface<F>(start: Vec2, end: Vec2, count: [u32; 2], f: F) -> Surface
where
    F: Fn(Vec2) -> Vec3,
{
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut triangles = Vec::new();

    let diff = end - start;
    let delta_x = diff[0] / count[0] as f32;
    let delta_y = diff[1] / count[1] as f32;
    let eps = f32::min(delta_x, delta_y) / 1e3;

    for i in 0..=count[0] {
        for j in 0..=count[1] {
            let x = start[0] + i as f32 * delta_x;
            let y = start[1] + j as f32 * delta_y;
            let z = Vec2::new(x, y);
            let x_eps = Vec2::new(eps, 0.0);
            let y_eps = Vec2::new(0.0, eps);
            let o = f(z);
            positions.push(o);
            let a = f(z + x_eps);
            let b = f(z + y_eps);
            let c = f(z - x_eps);
            let d = f(z - y_eps);
            let norm1 = triangle_normal(o, a, b);
            let norm2 = triangle_normal(o, b, c);
            let norm3 = triangle_normal(o, c, d);
            let norm4 = triangle_normal(o, d, a);
            let norm = (norm1 + norm2 + norm3 + norm4).normalize();
            normals.push(norm.into());
        }
    }

    for i in 0..count[0] {
        for j in 0..count[1] {
            let bottom_left = i * (count[1] + 1) + j;
            let top_left = i * (count[1] + 1) + j + 1;
            let bottom_right = (i + 1) * (count[1] + 1) + j;
            let top_right = (i + 1) * (count[1] + 1) + j + 1;
            triangles.push([top_left, bottom_left, top_right]);
            triangles.push([top_right, bottom_left, bottom_right]);
        }
    }

    Surface {
        start,
        end,
        count,
        positions,
        normals,
        triangles
    }
}

pub fn surface_to_solid<F>(surface: &Surface, f: F) -> Mesh
where
    F: Fn(Vec2) -> Vec2
{
    let positions = surface.positions.iter().map(|p| (*p).into()).collect();
    let normals = surface.normals.iter().map(|n| (*n).into()).collect();
    let mut indices = Vec::new();
    let uvs = compute_uvs(&surface, f);
    for t in &surface.triangles {
        indices.extend_from_slice(t);
    }
    Mesh {
        primitive_topology: PrimitiveTopology::TriangleList,
        attributes: vec![
            VertexAttribute::position(positions),
            VertexAttribute::normal(normals),
            VertexAttribute::uv(uvs)
        ],
        indices: Some(indices)
    }
}

pub fn surface_to_wireframe<F>(surface: &Surface, f: F) -> Mesh
where
    F: Fn(Vec2) -> Vec2
{
    let positions = surface.positions.iter().map(|p| (*p).into()).collect();
    let normals = surface.normals.iter().map(|n| (*n).into()).collect();
    let mut indices = Vec::new();
    for t in &surface.triangles {
        indices.extend_from_slice(&[t[0], t[1], t[1], t[2], t[2], t[0]]);
    }
    let uvs = compute_uvs(&surface, f);
    Mesh {
        primitive_topology: PrimitiveTopology::LineList,
        attributes: vec![
            VertexAttribute::position(positions),
            VertexAttribute::normal(normals),
            VertexAttribute::uv(uvs)
        ],
        indices: Some(indices)
    }
}

pub fn surface_to_point_cloud<F>(surface: &Surface, f: F) -> Mesh
where
    F: Fn(Vec2) -> Vec2
{
    let positions = surface.positions.iter().map(|p| (*p).into()).collect();
    let normals = surface.normals.iter().map(|n| (*n).into()).collect();
    let mut indices = Vec::new();
    for t in &surface.triangles {
        indices.extend_from_slice(&[t[0], t[1], t[2]]);
    }
    let uvs = compute_uvs(&surface, f);
    Mesh {
        primitive_topology: PrimitiveTopology::PointList,
        attributes: vec![
            VertexAttribute::position(positions),
            VertexAttribute::normal(normals),
            VertexAttribute::uv(uvs)
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

pub fn plane(z: Vec2) -> Vec3 {
    z.extend(0.0)
}

pub fn torus(a: f32, b: f32, z: Vec2) -> Vec3 {
    let pos = Vec3::new(a + b * f32::cos(TAU * z[1]), 0.0, b * f32::sin(TAU * z[1]));
    Mat3::from_rotation_z(TAU * z[0]) * pos
}

use num_complex::Complex;

pub fn to_complex(z: Vec2) -> Complex<f32> {
    Complex::new(z[0], z[1])
}

pub fn from_complex(z: Complex<f32>) -> Vec2 {
    Vec2::new(z.re, z.im)
}

// Functions
pub fn to_uv(z: Complex<f32>) -> Vec2 {
    let r = z.norm();
    let phi = z.arg();
    let frac = phi / (std::f32::consts::PI * 2.0);
    Vec2::new(0.0, r)
}

pub fn identity(z: Vec2) -> Vec2 {
    to_uv(to_complex(z))
}

pub struct Mero {
    pub factor: Complex<f32>,
    pub zeros: Vec<Complex<f32>>,
    pub poles: Vec<Complex<f32>>
}

impl Mero {
    pub fn new() -> Mero {
        Mero {
            factor: Complex::new(1.0, 0.0),
            zeros: vec![],
            poles: vec![]
        }
    }
}

pub fn poly(ps: &Mero, z: Vec2) -> Vec2 {
    let z = to_complex(z);
    let one = Complex::new(1.0, 0.0);
    let mut val = ps.factor;
    for p in ps.zeros.iter() {
        val *= z - p
    }
    for p in ps.poles.iter() {
        val *= one/(z - p)
    }
    to_uv(val)
}
