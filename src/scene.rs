use cgmath;

use main::RGB;

pub struct Object {
  pub center        : cgmath::Vector3<f32>,
  pub radius        : f32,
  pub diffuseness   : f32,
  pub emittance     : f32,
  pub reflectance   : f32,
  pub transmittance : f32,
  pub texture       : Texture,
}

pub enum Texture {
  SolidColor(RGB),
}

pub struct T {
  pub objects       : Vec<Object>,
  pub fovy          : f32,
  pub eye           : cgmath::Vector3<f32>,
  pub look          : cgmath::Vector3<f32>,
  pub up            : cgmath::Vector3<f32>,
}

impl T {
  pub fn move_camera(&mut self, v: &cgmath::Vector3<f32>) {
    self.eye = self.eye + v;
  }

  pub fn x(&self) -> cgmath::Vector3<f32> {
    self.look.cross(self.up)
  }

  pub fn y(&self) -> cgmath::Vector3<f32> {
    self.up
  }

  pub fn z(&self) -> cgmath::Vector3<f32> {
    self.look
  }

  pub fn render(&self, width: u32, height: u32, random_seed: u64) -> Vec<RGB> {
    let mut r = vec!();
    for y in 0 .. height {
      for x in 0 .. width {
        r.push(RGB {
          r: x as f32 / width as f32,
          g: y as f32 / height as f32,
          b: 0.0,
        });
      }
    }

    r
  }
}
