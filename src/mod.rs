extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate glutin;
#[macro_use]
extern crate log;
extern crate rand;
extern crate time;

mod main;
mod prelude;
mod raytrace;
mod scene;

fn main() {
  main::main();
}
