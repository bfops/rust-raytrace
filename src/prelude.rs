use cgmath;
use glium;
use std;

pub use cgmath::EuclideanSpace;

pub type Vector = cgmath::Vector3<f32>;
pub type Point = cgmath::Point3<f32>;
pub type Matrix = cgmath::Matrix3<f32>;

mod dumb_submodule {
  use cgmath::{InnerSpace};

  pub fn normalize(v: super::Vector) -> super::Vector {
    v.normalize()
  }

  pub fn dot(v1: super::Vector, v2: super::Vector) -> f32 {
    v1.dot(v2)
  }

  pub fn cross(v1: super::Vector, v2: super::Vector) -> super::Vector {
    v1.cross(v2)
  }
}

pub use self::dumb_submodule::*;

#[derive(Debug ,Clone)]
pub struct Ray {
  pub origin    : Point,
  pub direction : Vector,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RGB {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

unsafe impl Send for RGB {}

unsafe impl glium::texture::PixelValue for RGB {
  fn get_format() -> glium::texture::ClientFormat {
    glium::texture::ClientFormat::F32F32F32
  }
}

impl std::ops::Add for RGB {
  type Output = RGB;
  fn add(self, rhs: RGB) -> Self::Output {
    RGB {
      r: self.r + rhs.r,
      g: self.g + rhs.g,
      b: self.b + rhs.b,
    }
  }
}

impl std::ops::AddAssign for RGB {
  fn add_assign(&mut self, rhs: RGB) {
    self.r += rhs.r;
    self.g += rhs.g;
    self.b += rhs.b;
  }
}

impl std::ops::Mul for RGB {
  type Output = RGB;
  fn mul(self, rhs: RGB) -> Self::Output {
    RGB {
      r: self.r * rhs.r,
      g: self.g * rhs.g,
      b: self.b * rhs.b,
    }
  }
}

impl std::ops::Mul<f32> for RGB {
  type Output = RGB;
  fn mul(self, rhs: f32) -> Self::Output {
    RGB {
      r: self.r * rhs,
      g: self.g * rhs,
      b: self.b * rhs,
    }
  }
}

// http://http.developer.nvidia.com/Cg/asin.html
pub fn asin(x: f32) -> f32 {
  let negate = (x < 0.0) as u32 as f32;
  let x = x.abs();
  let mut ret = -0.0187293;
  ret *= x;
  ret += 0.0742610;
  ret *= x;
  ret -= 0.2121144;
  ret *= x;
  ret += 1.5707288;
  ret = 3.14159265358979*0.5 - (1.0 - x).sqrt()*ret;
  ret - 2.0 * negate * ret
}
