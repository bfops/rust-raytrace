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

fn cast<'a>(s: &'a scene::T, ray: &Ray) -> Option<&'a scene::Object> {
  let mut collision = None;

  for object in &s.objects {
    let toi = object.toi(ray);
    if let Some(toi) = toi {
      if let Some((best_toi, _)) = collision {
        if toi >= best_toi {
          continue
        }
      }
      collision = Some((toi, object));
    }
  }

  collision.map(|(_, obj)| obj)
}

fn trace(s: &scene::T, ray: &Ray) -> RGB {
  if cast(s, ray).is_some() {
    RGB { r: 1.0, g: 0.0, b: 0.0 }
  } else {
    RGB { r: 0.0, g: 0.0, b: 0.0 }
  }
}

fn do_work(s: &scene::T, work: &Work, output: &mut Output) {
  *output.pixel_mut(work.pixel_x, work.pixel_y) = work.attenuation * trace(s, &work.ray);
}

pub fn scene(s: &scene::T, width: u32, height: u32, random_seed: u64) -> Output {
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
              direction : normalize(&(view_to_world * ray)),
            },
          pixel_x     : x,
          pixel_y     : y,
          attenuation : RGB { r: 1.0, g: 1.0, b: 1.0 },
        }
      );
    }
  }

  while let Some(work) = work_items.pop_front() {
    do_work(s, &work, &mut output);
  }

  output
}
