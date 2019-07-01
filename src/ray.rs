use crate::vec3::*;

pub struct Ray {
  pub origin: Vec3,
  pub direction: Vec3,
}

impl Ray {
  pub fn new() -> Self {
    Ray {
      origin: Vec3::new(),
      direction: Vec3::new()
    }
  }

  pub fn point_at_parameter(&self, t: f32) -> Vec3 {
    // LERP from origin
    self.origin + t * self.direction
  }
}

// Construct a Ray from 2 Vec3s
impl From<(Vec3, Vec3)> for Ray {
  fn from(tuple: (Vec3, Vec3)) -> Self {
    Ray{
      origin: tuple.0,
      direction: tuple.1
    }
  }
}
