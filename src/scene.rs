use prelude::*;

pub struct Collision<'a> {
  pub object   : &'a Object,
  pub toi      : f32,
  pub location : Point,
  pub normal   : Vector,
}

pub struct Object {
  pub center        : Point,
  pub radius        : f32,
  pub shininess     : f32,
  pub emittance     : f32,
  pub reflectance   : f32,
  pub transmittance : f32,
  pub texture       : Texture,
}

fn either_or_join<T, F: FnOnce(T, T) -> T>(f: F, x: Option<T>, y: Option<T>) -> Option<T> {
  match (x, y) {
    (None    , None)    => None,
    (x       , None)    => x,
    (None    , y)       => y,
    (Some(x) , Some(y)) => Some(f(x, y)),
  }
}

impl Object {
  pub fn intersect_ray<'a>(&'a self, ray: &Ray) -> Option<Collision<'a>> {
    // quadratic coefficients
    let a = dot(ray.direction, ray.direction);
    let to_center = ray.origin - self.center;
    let b = 2.0 * dot(to_center, ray.direction);
    let c = dot(to_center, to_center) - self.radius*self.radius;

    // discriminant
    let d = b*b - 4.0*a*c;

    if d < 0.0 {
      return None;
    }

    let d = d.sqrt();
    let a = 2.0 * a;

    let s1 = (d - b) / a;
    let s1 = if s1 >= 0.0 { Some(s1) } else { None };
    let s2 = (-d - b) / a;
    let s2 = if s2 >= 0.0 { Some(s2) } else { None };

    either_or_join(f32::min, s1, s2)
      .map(|toi| {
        let location = ray.origin + toi*ray.direction;
        Collision {
          object   : self,
          toi      : toi,
          location : location,
          normal   : normalize(location - self.center),
        }
      })
  }
}

pub enum Texture {
  SolidColor(RGB),
}

pub struct T {
  pub objects       : Vec<Object>,
  pub fovy          : f32,
  pub eye           : Point,
  pub look          : Vector,
  pub up            : Vector,
}

impl T {
  pub fn move_camera(&mut self, v: &Vector) {
    self.eye = self.eye + v;
  }

  pub fn x(&self) -> Vector {
    self.look.cross(self.up)
  }

  pub fn y(&self) -> Vector {
    self.up
  }

  pub fn z(&self) -> Vector {
    self.look
  }
}
