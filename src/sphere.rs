use bevy::{
  math::*
};

pub struct Sphere;

pub struct SphereFunction<V, F>
where F: Fn(Vec2) -> V
{
  north_part: F,
  south_part: F
}

pub fn south_chart(z: Vec2) -> Vec3 {
  let t = 2. / (1. + z.length_squared());
  Vec3::new(t*z[0], t*z[1], 1. - t)
}

pub fn north_chart(z: Vec2) -> Vec3 {
  let t = 2. / (1. + z.length_squared());
  Vec3::new(t*z[0], t*z[1], t - 1.)
}
