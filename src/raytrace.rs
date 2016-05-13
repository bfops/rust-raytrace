use std;

use prelude::*;
use scene;

pub struct Output {
  data : Vec<RGB>,
  w    : u32,
  h    : u32,
}

impl Output {
  pub fn new(w: u32, h: u32) -> Self {
    Output {
      data : std::iter::repeat(RGB { r: 0.0, g: 0.0, b: 0.0 }).take((w * h) as usize).collect(),
      w    : w,
      h    : h,
    }
  }

  pub fn pixel_mut(&mut self, x: u32, y: u32) -> &mut RGB {
    self.data.get_mut((y * self.w + x) as usize).unwrap()
  }

  pub fn to_vec(self) -> Vec<RGB> {
    self.data
  }
}

/// A unit of work to be done.
/// Consists of a ray to trace, an attenuation, and a pixel location to draw to.
struct Work {
  pub ray         : Ray,
  pub pixel_x     : u32,
  pub pixel_y     : u32,
  pub attenuation : RGB,
}

fn cast<'a>(s: &'a scene::T, ray: &Ray) -> Option<scene::Collision<'a>> {
  let mut first_collision: Option<scene::Collision<'a>> = None;

  for object in &s.objects {
    if let Some(collision) = object.intersect_ray(ray) {
      if let Some(first_collision) = first_collision.as_ref() {
        if first_collision.toi < collision.toi {
          continue
        }
      }
      first_collision = Some(collision);
    }
  }

  first_collision
}

fn do_work<AddWork: FnMut(Work)> (
  s: &scene::T,
  work: &Work,
  add_work: &mut AddWork,
  output: &mut Output,
) {
  let min_attenuation = 0.01;
  if work.attenuation.r < min_attenuation &&
     work.attenuation.g < min_attenuation &&
     work.attenuation.b < min_attenuation {
    return
  }

  let collision =
    match cast(s, &work.ray) {
      None => return,
      Some(c) => c,
    };
  let color =
    match collision.object.texture {
      scene::Texture::SolidColor(color) => color,
    };

  let color = work.attenuation * color;

  *output.pixel_mut(work.pixel_x, work.pixel_y) += color * collision.object.emittance;

  let make_ray = {
    let location = collision.location;
    move |direction| {
      Ray {
        direction : direction,
        origin    : location + 0.01 * direction,
      }
    }
  };

  let make_work = {
    let pixel_x = work.pixel_x;
    let pixel_y = work.pixel_y;
    move |ray, attenuation| {
      Work {
        ray         : ray,
        attenuation : attenuation,
        pixel_x     : pixel_x,
        pixel_y     : pixel_y,
      }
    }
  };

  let reflected = work.ray.direction - 2.0 * dot(work.ray.direction, collision.normal) * collision.normal;
  add_work(make_work(make_ray(reflected), color * collision.object.reflectance));

  let transmitted = work.ray.direction;
  add_work(make_work(make_ray(transmitted), color * collision.object.transmittance));
}

pub fn scene(s: &scene::T, width: u32, height: u32) -> Output {
  let mut output = Output::new(width, height);
  let mut work_items = std::collections::VecDeque::new();

  let aspect = width as f32 / height as f32;
  let max_y = (s.fovy / 2.0).tan();
  let scale = 2.0 * max_y / height as f32;
  let shift = -max_y;

  let view_to_world = Matrix::from_cols(s.x(), s.y(), s.z());

  for y in 0 .. height {
    for x in 0 .. width {
      // in view coordinates
      let ray =
        Vector::new(
          scale * x as f32 + shift * aspect,
          scale * y as f32 + shift,
          1.0,
        );

      work_items.push_back(
        Work {
          ray         :
            Ray {
              origin    : s.eye,
              direction : normalize(view_to_world * ray),
            },
          pixel_x     : x,
          pixel_y     : y,
          attenuation : RGB { r: 1.0, g: 1.0, b: 1.0 },
        }
      );
    }
  }

  while let Some(work) = work_items.pop_front() {
    let mut add_work = |work| work_items.push_back(work);
    do_work(s, &work, &mut add_work, &mut output);
  }

  output
}
