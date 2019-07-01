use std::ops;
use std::iter::Sum;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
  // C++ style implementation from book
  pub e: [f32; 3],
}

impl Vec3 {
  pub fn new() -> Self {
    Vec3 {
      e: [0.0; 3]
    }
  }

  // Traditional C++-style x,y.z access
  pub fn x(&self) -> f32 {
    self.e[0]
  }
  pub fn y(&self) -> f32 {
    self.e[1]
  }
  pub fn z(&self) -> f32 {
    self.e[2]
  }

  // Vec3 can also be used as a color. C++-style
  pub fn r(&self) -> f32 {
    self.e[0]
  }
  pub fn g(&self) -> f32 {
    self.e[1]
  }
  pub fn b(&self) -> f32 {
    self.e[2]
  }

  pub fn dot(&self, other: Vec3) -> f32 {
    // Algebraic definition of dot product of two vectors a = [a1, a2, a3] and b = [b1, b2, b3] is (a1b1 + a2b2+ a3b3)
    (self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2])
  }

  pub fn cross(&self, rhs: Vec3) -> Vec3 {
    // Right-hand rule for Cross product of two vectors in three-dimensional space
    Vec3 {
      e: [(self[1] * rhs[2]) - (self[2] * rhs[1]),
          (self[2] * rhs[0]) - (self[0] * rhs[2]),
          (self[0] * rhs[1]) - (self[1] * rhs[0])]
    }
  }

  // rhs of reflect is the surface normal result will be reflected off of
  pub fn reflect(self, rhs: Vec3) -> Vec3 {
    self - 2.0 * self.dot(rhs) * rhs
  }

  // Snell's law for refraction
  pub fn refract(self, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = self.unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
      Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
      None
    }
  }

  pub fn length(&self) -> f32 {
    // Length/Magnitude of a Vec3 is SQRT of Dot product with itself
    self.dot(*self).sqrt()
  }

  pub fn squared_length(&self) -> f32 {
    self.dot(*self)
  }

  /// Normalizes vector in-place
  pub fn make_unit_vector(&mut self) {
    *self = self.unit_vector();
  }

  pub fn unit_vector(self) -> Self {
    self / self.length()
  }
}

// from a tuple of f32s
impl From<(f32, f32, f32)> for Vec3 {
  fn from(tuple: (f32, f32, f32)) -> Self {
    Vec3 {
      e: [tuple.0, tuple.1, tuple.2]
    }
  }
}

// from a slice/array of f32s
impl From<[f32; 3]> for Vec3 {
  fn from(e: [f32; 3]) -> Self {
    Vec3 {
      e
    }
  }
}

// operator[]
impl ops::Index<usize> for Vec3 {
  type Output = f32;
  fn index(&self, index: usize) -> &Self::Output {
    &self.e[index]
  }
}

impl ops::Neg for Vec3 {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Self {
      e: [-self[0], -self[1], -self[2]],
    }
  }
}

impl ops::Add for Vec3 {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    Self {
      e: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]]
    }
  }
}

impl ops::AddAssign for Vec3 {
  fn add_assign(&mut self, rhs: Self) {
    *self = Self {
      e: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]]
    };
  }
}

impl ops::Sub for Vec3 {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    Self {
      e: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]]
    }
  }
}

impl ops::SubAssign for Vec3 {
  fn sub_assign(&mut self, rhs: Self) {
    *self = Self {
      e: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]]
    };
  }
}

impl ops::Mul for Vec3 {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self {
    Self {
      e: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]]
    }
  }
}

impl ops::MulAssign for Vec3 {
  fn mul_assign(&mut self, rhs: Self) {
    *self = Self {
      e: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]]
    };
  }
}

impl ops::Div for Vec3 {
  type Output = Self;
  fn div(self, rhs: Self) -> Self {
    Self {
      e: [self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2]]
    }
  }
}

impl ops::DivAssign for Vec3 {
  fn div_assign(&mut self, rhs: Self) {
    *self = Self {
      e: [self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2]]
    };
  }
}

impl ops::Mul<f32> for Vec3 {
  type Output = Self;
  fn mul(self, rhs: f32) -> Self {
    Self {
      e: [self[0] * rhs, self[1] * rhs, self[2] * rhs]
    }
  }
}

impl ops::MulAssign<f32> for Vec3 {
  fn mul_assign(&mut self, rhs: f32) {
    *self = Self {
      e: [self[0] * rhs, self[1] * rhs, self[2] * rhs]
    };
  }
}

impl ops::Mul<Vec3> for f32 {
  type Output = Vec3;
  fn mul(self, rhs: Vec3) -> Vec3 {
    Vec3 {
      e: [self * rhs[0], self * rhs[1], self * rhs[2]]
    }
  }
}

impl ops::Div<f32> for Vec3 {
  type Output = Self;
  fn div(self, rhs: f32) -> Self {
    Self {
      e: [self[0] / rhs, self[1] / rhs, self[2] / rhs]
    }
  }
}

impl ops::DivAssign<f32> for Vec3 {
  fn div_assign(&mut self, rhs: f32) {
    *self = Self {
      e: [self[0] / rhs, self[1] / rhs, self[2] / rhs]
    }
  }
}

// This allows us to parallelize adding up a collection of Vec3s
impl Sum<Vec3> for Vec3 {
  fn sum<I>(iter: I) -> Vec3
    where
      I: Iterator<Item = Vec3>,
  {
    iter.fold(Vec3::new(), |a, b| {
      a + b
    })
  }
}