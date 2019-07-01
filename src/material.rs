use crate::hitable::*;
use crate::ray::*;
use crate::sphere::*;
use crate::vec3::*;
use rand::Rng;
use rand::distributions::{Distribution, Standard};

// One trait to rule them all
pub trait Material {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3) -> Option<Ray>;
}

// Use an enum as type of material
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MaterialType {
  Dielectric(Dielectric),
  Lambertian(Lambertian),
  Metal(Metal),
}

// So we can use Material with rand::random::<Material>()
impl Distribution<MaterialType> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MaterialType {
    // Weighted random material. 80% Diffuse, 15% Metal, 5% Glass
    let mat_type: f32 = rng.gen();
    match mat_type {
      x if x < 0.8 => {
        MaterialType::Lambertian(Lambertian::from((rng.gen(), rng.gen(), rng.gen())))
      },
      x if x < 0.95 => {
        MaterialType::Metal(Metal::from(((rng.gen(), rng.gen(), rng.gen()), rng.gen())))
      },
      _ => {
        let rnd: f32 = rng.gen();
        MaterialType::Dielectric(Dielectric::from(rnd))
      },
    }
  }
}

impl Material for MaterialType {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3) -> Option<Ray> {
    use MaterialType::*;
    match self {
      Dielectric(d) => d.scatter(r_in, rec, attenuation),
      Lambertian(l) => l.scatter(r_in, rec, attenuation),
      Metal(m) => m.scatter(r_in, rec, attenuation),
    }
  }
}

// =================================================================================
/// DIELECTRIC MATERIAL
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dielectric {
  pub ref_idx: f32,
}

impl Dielectric {
  pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
  }
}

impl From<f32> for Dielectric {
  fn from(ref_idx: f32) -> Self {
    Self {
      ref_idx
    }
  }
}

impl Material for Dielectric {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3) -> Option<Ray> {
    let mut outward_normal: Vec3;
    let reflected = r_in.direction.reflect(rec.normal);
    let ni_over_nt: f32;
    let reflect_prob: f32;
    let cosine: f32;
    *attenuation = Vec3::from((1.0, 1.0, 1.0));
    if r_in.direction.dot(rec.normal) > 0.0 {
      outward_normal = -rec.normal;
      ni_over_nt = self.ref_idx;
      cosine = self.ref_idx * r_in.direction.dot(rec.normal) / r_in.direction.length();
    } else {
      outward_normal = rec.normal;
      ni_over_nt = 1.0 / self.ref_idx;
      cosine = -r_in.direction.dot(rec.normal) / r_in.direction.length();
    }

    let refracted = r_in.direction.refract(outward_normal, ni_over_nt);
    if refracted.is_some() {
      reflect_prob = Dielectric::schlick(cosine, self.ref_idx);
    } else {
      reflect_prob = 1.0;
    }

    // Result randomly chosen between reflected and refracted
    if rand::random::<f32>() < reflect_prob {
      Some(Ray::from((rec.p, reflected)))
    } else {
      Some(Ray::from((rec.p, refracted.unwrap())))
    }
  }
}
// =================================================================================

// =================================================================================
/// LAMBERTIAN MATERIAL
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Lambertian {
  pub albedo: Vec3,
}

impl From<(f32, f32, f32)> for Lambertian {
  fn from(tuple: (f32, f32, f32)) -> Self {
    Self {
      albedo: Vec3::from(tuple)
    }
  }
}

impl From<Vec3> for Lambertian {
  fn from(albedo: Vec3) -> Self {
    Self {
      albedo
    }
  }
}

impl Material for Lambertian {
  fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3) -> Option<Ray> {
    let target = rec.p + rec.normal + Sphere::random_in_unit_sphere();
    *attenuation = self.albedo;
    Some(Ray::from((rec.p, target - rec.p)))
  }
}
// =================================================================================

// =================================================================================
/// METAL MATERIAL
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Metal {
  pub albedo: Vec3,
  pub fuzz: f32,
}

// This from is for Vec3 from 3 f32s tupled with a f32 for fuzz
impl From<((f32, f32, f32), f32)> for Metal {
  fn from(tuple: ((f32, f32, f32), f32)) -> Self {
    Self {
      albedo: Vec3::from(tuple.0),
      fuzz: tuple.1,
    }
  }
}

impl From<(Vec3, f32)> for Metal {
  fn from(tuple: (Vec3, f32)) -> Self {
    Self {
      albedo: Vec3::from(tuple.0),
      fuzz: tuple.1,
    }
  }
}

impl Material for Metal {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3) -> Option<Ray> {
    let reflected = r_in.direction.unit_vector().reflect(rec.normal);
    let scattered = Ray::from((rec.p, reflected + self.fuzz * Sphere::random_in_unit_sphere()));
    *attenuation = self.albedo;
    // Result:
    if scattered.direction.dot(rec.normal) > 0.0 {
      Some(scattered)
    } else {
      None
    }
  }
}
// =================================================================================