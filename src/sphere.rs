use crate::hitable::*;
use crate::material::*;
use crate::ray::*;
use crate::vec3::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
  pub material: MaterialType,
}

impl Sphere {
  /// Utility function to get a random point in a sphere w/ radius of 1.0
  pub fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    loop {
      p = 2.0 * Vec3::from((rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>())) - Vec3::from([1.0; 3]);
      if p.squared_length() < 1.0 {
        return p;
      }
    }
  }
}

impl From<(Vec3, f32, MaterialType)> for Sphere {
  fn from(tuple: (Vec3, f32, MaterialType)) -> Self {
    Sphere {
      center: tuple.0,
      radius: tuple.1,
      material: tuple.2,
    }
  }
}

impl Hitable for Sphere {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
      let mut record = None;
      let oc = r.origin - self.center;
      let a = r.direction.dot(r.direction);
      let b = oc.dot(r.direction);
      let c = oc.dot(oc) - self.radius * self.radius;
      let discriminant = b * b - a * c;
      if discriminant > 0.0 {
        let mut temp = (-b - discriminant.sqrt()) / a;
        if temp < t_max && temp > t_min {
          let p = r.point_at_parameter(temp);
          record = Some(
            HitRecord {
              t: temp,
              p,
              normal: (p - self.center) / self.radius,
              material: self.material,
            });
        } else {
          temp = (-b + discriminant.sqrt()) / a;
          if temp < t_max && temp > t_min {
            let p = r.point_at_parameter(temp);
            record = Some(
              HitRecord {
                t: temp,
                p,
                normal: (p - self.center) / self.radius,
                material: self.material,
              });
          }
        }
      }

      // If something was hit, record will not be None
      record
  }
}