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

