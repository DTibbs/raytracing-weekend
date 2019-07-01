//! Hitable module contains the Hitable trait, HitRecord, and HitableList
use crate::ray::*;
use crate::vec3::*;
use std::sync::Arc;
use crate::material::*;

pub trait Hitable {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

//#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
  pub t: f32,
  pub p: Vec3,
  pub normal: Vec3,
  pub material: MaterialType,
}

// impl HitRecord {
//   pub fn new() -> Self {
//     HitRecord {
//       t: 0.0,
//       p: Vec3::from([0.0; 3]),
//       normal: Vec3::from([0.0; 3]),
//       material: Arc::new()
//     }
//   }
// }

pub struct HitableList {
  pub list: Vec<Arc<Hitable + Send + Sync>>,
}

impl HitableList {
  pub fn new(list: Vec<Arc<Hitable + Send + Sync>>) -> Self {
    HitableList {
      list
    }
  }
}

impl Hitable for HitableList {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let mut rec = None;
    let mut closest_so_far = t_max;
    for h in &self.list {
      let temp = h.hit(&r, t_min, closest_so_far);
      if let Some(hit_anything) = temp {
        closest_so_far = hit_anything.t;
        rec = Some(hit_anything);
      }
    }

    rec
  }
}