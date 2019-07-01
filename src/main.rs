use rayon::prelude::*;
use std::sync::Arc;
use rand::{thread_rng, Rng};

pub mod vec3;
pub mod ray;
pub mod sphere;
pub mod hitable;
pub mod camera;
pub mod material;

use vec3::*;
use ray::*;
use sphere::*;
use hitable::*;
use camera::*;
use material::*;

// 2560 x 1440 on 6 cores, in release, is ~14minutes
//const RES_X: u32 = 2560;
//const RES_Y: u32 = 1440;

// 1000 x 500 on 6 cores, in release, is ~1min 10s
const RES_X: u32 = 1000;
const RES_Y: u32 = 500;

const NUM_SAMPLES: u32 = 10;
// Original color from book:
const BG_COLOR: Vec3 = Vec3 {
  e: [0.5, 0.7, 1.0]
};

// const BG_COLOR: Vec3 = Vec3 {
//   e: [0.8, 0.6, 0.7]
// };

fn color<T: Hitable + Send + Sync>(r: &Ray, world: &Arc<T>, depth: i32) -> Vec3 {
  // Do ray cast in world, and calculate color at location it hits based on what & where it hits. 
  if let Some(rec) = world.hit(r, 0.001, std::f32::MAX) {
    let mut attenuation = Vec3::new();
    if let Some(scatter) = rec.material.scatter(&r, &rec, &mut attenuation) {
      if depth < 50 {
        return attenuation * color(&scatter, world, depth + 1);
      }
    }
    // else
    Vec3::new()
  } else {
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    // This is an oddly crafted port of C++ code that returns the same result.
    (1.0 - t) * Vec3::from([1.0; 3]) + t * Vec3::from(BG_COLOR)
  }
}

fn generate_ppm(world: &Arc<HitableList>, cam: &Camera) {
  // O(n^2) for looping
  for j in (0..RES_Y).rev() {
    for i in 0..RES_X {
      let mut col = Vec3::from([0.0; 3]);
      
      // Anti-aliasing. ns is number of samples to take around each u,v point
      for _ in 0..NUM_SAMPLES {
        let u: f32 = ((i as f32) + rand::random::<f32>()) / (RES_X as f32);
        let v: f32 = ((j as f32) + rand::random::<f32>()) / (RES_Y as f32);
        let r = cam.get_ray(u, v);
        
        // WTF is this?????
        //let p = r.point_at_parameter(2.0);

        col += color(&r, &world, 0);
      }
      col /= NUM_SAMPLES as f32;

      // halve the color (gamma 2) by sqrt'ing each value
      col = Vec3::from((col.r().sqrt(), col.g().sqrt(), col.b().sqrt()));

      let ir = (255.99 * col[0]) as i32;
      let ig = (255.99 * col[1]) as i32;
      let ib = (255.99 * col[2]) as i32;
      
      // TODO: For this to work in parallel, we can't print from inside this loop:
      println!("{} {} {}", ir, ig, ib);
    }
  }
}

fn generate_ppm_parallel(world: &Arc<HitableList>, cam: &Camera) {
  // Use a parallel iterator to pre-calculate each pixel.
  
  // Allocate all the pixels on heap w/ Vec
  let pixel_count = RES_X * RES_Y;
  let mut pixels = Vec::with_capacity(pixel_count as usize);
  // Fill it so we can mutate it w/ par_iter
  for _ in 0..pixel_count {
    pixels.push((0, 0, 0));
  }

  // Enumerate the par_iter so we get an index w/ each iteration
  pixels.par_iter_mut().enumerate().for_each(|(index, pxl)| {
    let row = RES_Y - (index as u32 / RES_X);
    let col = index as u32 % RES_X;

    //let mut clr = Vec3::from([0.0; 3]);
      
    // Anti-aliasing by generating a bunch of random points and averaging (sum / NUM_SAMPLES)
    let mut pixel: Vec3 = (0..NUM_SAMPLES).into_par_iter().map(|_| {
      let u: f32 = ((col as f32) + rand::random::<f32>()) / (RES_X as f32);
      let v: f32 = ((row as f32) + rand::random::<f32>()) / (RES_Y as f32);
      let r = cam.get_ray(u, v);
      color(&r, &world, 0)
    }).sum();
    pixel /= NUM_SAMPLES as f32;

    // halve the color (gamma 2) by sqrt'ing each value
    pixel = Vec3::from((pixel.r().sqrt(), pixel.g().sqrt(), pixel.b().sqrt()));

    // Set the pixel color on the 
    *pxl = ((255.99 * pixel[0]) as i32,
          (255.99 * pixel[1]) as i32,
          (255.99 * pixel[2]) as i32);
  });

  for p in pixels.iter() {
    println!("{} {} {}", p.0, p.1, p.2);
  }
}

fn main() {
  // PPM header
  println!("P3\n{} {}\n255", RES_X, RES_Y);

  // List of Hitable things
  let mut list: Vec<Arc<Hitable + Send + Sync>> = Vec::new();
  // Large sphere everything is sitting on (like EARF)
  list.push(Arc::new(
    Sphere::from((Vec3::from((0.0, -1000.0, 0.0)),
                  1000.0,
                  MaterialType::Lambertian(Lambertian::from((0.5, 0.5, 0.5)))))
  ));

  // Throw a ton more spheres in the world randomly
  for a in -11..11 {
    for b in -11..11 {
      let material = rand::random::<MaterialType>();
      let center = Vec3::from(((a as f32) + 0.9 * rand::random::<f32>(), 0.2, (b as f32) + 0.9 * rand::random::<f32>()));
      if (center - Vec3::from((4.0, 0.2, 0.0))).length() > 0.9 {
        list.push(Arc::new(
          Sphere::from((center,
                        0.2,
                        material))
        ));
      }
    }
  }

  list.push(Arc::new(
    Sphere::from((Vec3::from((0.0, 1.0, 0.0)),
                  1.0,
                  MaterialType::Dielectric(Dielectric::from(1.5))))
  ));
  list.push(Arc::new(
    Sphere::from((Vec3::from((-4.0, 1.0, 0.0)),
                  1.0,
                  MaterialType::Lambertian(Lambertian::from((0.4, 0.2, 0.1)))))
  ));
  list.push(Arc::new(
    Sphere::from((Vec3::from((4.0, 1.0, 0.0)),
                  1.0,
                  MaterialType::Metal(Metal::from(((0.7, 0.6, 0.5), 0.0)))))
  ));

  let world = Arc::new(HitableList::new(list));
  let cam: Camera = Camera::new(
        Vec3::from((7.0, 1.5, 1.95)),
        Vec3::from((0.0, 0.0, -1.0)),
        Vec3::from((0.0, 1.0, 0.0)),
        50.0,
        (RES_X as f32) / (RES_Y as f32));

  // TODO: Compare run times of these
  //generate_ppm(&world, &cam);
  generate_ppm_parallel(&world, &cam);
}
                                                                                                                    