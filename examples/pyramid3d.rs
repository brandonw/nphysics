#![crate_type = "bin"]
#![warn(non_camel_case_types)]

extern crate std;
extern crate native;
extern crate kiss3d;
extern crate graphics3d;
extern crate nphysics = "nphysics3df32";
extern crate ncollide = "ncollide3df32";
extern crate nalgebra;

use std::rc::Rc;
use std::cell::RefCell;
use kiss3d::window::Window;
use nalgebra::na::{Vec3, Translation};
use ncollide::geom::{Plane, Cuboid};
use nphysics::world::World;
use nphysics::object::{RigidBody, Static, Dynamic};
use graphics3d::engine::GraphicsManager;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    GraphicsManager::simulate(pyramid3d)
}

pub fn pyramid3d(window: &mut Window, graphics: &mut GraphicsManager) -> World {
    /*
     * World
     */
    let mut world = World::new();
    world.set_gravity(Vec3::new(0.0f32, -9.81, 0.0));

    /*
     * Planes
     */
    let rb   = RigidBody::new(Plane::new(Vec3::new(0.0f32, 1.0, 0.0)), 0.0, Static, 0.3, 0.6);
    let body = Rc::new(RefCell::new(rb));


    world.add_body(body.clone());
    graphics.add(window, body);

    /*
     * Create the boxes
     */
    let num     = 30;
    let rad     = 0.5;
    let shift   = rad * 2.0;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift / 2.0;

    for i in range(0u, num) {
        for j in range(i, num) {
            let fi = i as f32;
            let fj = (j - i) as f32;
            let x = (fi * shift / 2.0) + fj * shift - centerx;
            let y = fi * shift + centery;

            let mut rb = RigidBody::new(Cuboid::new(Vec3::new(rad, rad, rad)), 1.0f32, Dynamic, 0.3, 0.6);

            rb.append_translation(&Vec3::new(x, y, 0.0));

            let body = Rc::new(RefCell::new(rb));

            world.add_body(body.clone());
            graphics.add(window, body);
        }
    }

    /*
     * Set up the camera and that is it!
     */
    graphics.look_at(Vec3::new(0.0, 60.0, -60.0), Vec3::new(0.0, 0.0, 0.0));

    world
}
