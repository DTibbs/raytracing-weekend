use crate::ray::*;
use crate::vec3::*;
use std::f32::consts::PI;

// TODO: Add DOF

pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
}

// Common camera ratios: 16:9, 4:3
const SIXTEEN_NINE: f32 = 1440.0 / 2560.0;
const FOUR_THREE: f32 = 768.0 / 1024.0;
impl Default for Camera {
  fn default() -> Self {
    Camera::new(Vec3::from([0.0; 3]),
                Vec3::from((0.0, 0.0, -1.0)),
                Vec3::from((0.0, 1.0, 0.0)),
                90.0,
                SIXTEEN_NINE)
  }
}

impl Camera {
  pub fn new(origin: Vec3, look_at: Vec3, up: Vec3, fov: f32, aspect: f32) -> Self {
    let theta = fov * PI / 180.0;
    let half_height = (theta / 2.0).tan();
    let half_width = aspect * half_height;
    let w = (origin - look_at).unit_vector();
    let u = up.cross(w).unit_vector();
    let v = w.cross(u);
    Camera {
      origin,
      lower_left_corner: origin - half_width * u - half_height * v - w,
      horizontal: 2.0 * half_width * u,
      vertical: 2.0 * half_height * v,
    }
  }

  pub fn get_ray(&self, u: f32, v: f32) -> Ray {
    Ray {
      origin: self.origin,
      direction: (self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
  }
}