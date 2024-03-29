#![allow(dead_code, unused_imports)]

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use std::convert::{TryInto};
use std::ops::{Add, Sub, AddAssign, SubAssign};
pub use std::time::*;
use nalgebra::{geometry, Matrix};
use nalgebra::base;

pub use self::objects::Light;
mod objects;
pub use self::objects::Material;
pub use self::objects::Box;
pub use self::objects::Sphere;

pub use self::instant::Instant;
mod instant;

const LIGHT_COUNT: usize = 1;
const SPHERE_COUNT: usize = 10;
const BOX_COUNT: usize = 5;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Model {
    current_time: f32,
    sphere_count: i32,
    padding2: f32,
    padding3: f32,
    spheres: [Sphere; SPHERE_COUNT],
    lights: [Light; LIGHT_COUNT],
    boxes: [Box; BOX_COUNT],
}

impl Model {
    pub fn new<>(
    ) -> Self {
        let mut s: [Sphere; SPHERE_COUNT] = std::iter::repeat_with(|| Sphere::new())
            .take(SPHERE_COUNT).collect::<Vec<_>>()
            .try_into().unwrap();
        s[0].material = Material::get_metal();
        s[0].material.color = [0.0, 0.0, 0.5];
        s[1].material = Material::get_metal();
        s[1].material.color = [0.0, 0.5, 0.0];
        s[2].material = Material::get_metal();
        s[2].material.color = [0.5, 0.0, 0.0];
        s[3].material = Material::get_metal();
        s[3].material.color = [0.5, 0.5, 0.0];
        s[4].material.color = [0.0, 0.5, 0.5];
        s[5].material.color = [0.5, 0.0, 0.5];
        s[6].material.color = [0.5, 0.5, 0.5];
        s[7].material.color = [0.8, 0.5, 0.5];
        s[8].material.color = [0.5, 0.8, 0.5];
        s[9].material.color = [0.5, 0.5, 0.8];
        let mut b: [Box; BOX_COUNT] = std::iter::repeat_with(|| Box::new()).take(BOX_COUNT).collect::<Vec<_>>().try_into().unwrap();
        b[0].bounds = [[-20.0, 0.0, 0.0, 0.0], [-5.0, 10.0, 5.0, 0.0]];
        b[0].material = Material::get_metal();
        b[1].bounds = [[-70.0, -11.0, -100.0, 0.0], [70.0, -10.0, 50.0, 0.0]];
        b[1].material.color = [0.2, 0.7, 0.5];
        b[2].bounds = [[-70.0, -11.0, -100.0, 0.0], [-69.0, 60.0, 50.0, 0.0]];
        b[2].material = Material::get_metal();
        b[2].material.color = [0.2, 0.1, 0.9];
        b[3].bounds = [[69.0, -11.0, -100.0, 0.0], [70.0, 60., 50.0, 0.0]];
        b[3].material = Material::get_metal();
        b[3].material.color = [0.9, 0.2, 0.9];
        Self {
            current_time: 0.0,
            sphere_count: SPHERE_COUNT as i32,
            padding2: 0.0,
            padding3: 0.0,
            spheres: s,
            lights: std::iter::repeat_with(|| Light::new()).take(LIGHT_COUNT).collect::<Vec<_>>().try_into().unwrap(),
            boxes: b,
        }
    }

    pub fn update_current_time(&mut self, earlier: instant::Instant) {
        let temp = instant::Instant::now()
        .duration_since(earlier)
        .as_nanos();
        let temp1 = (temp % (std::u32::MAX-1) as u128) as u32;
        self.current_time = (temp1 as f32 / 4300000000.00) * 6.3;
    }

    pub fn update_model(&mut self) {
        for i in 0..self.sphere_count { 
            let i2 = i as f32;
            let offset = [(3.0*i2+self.current_time).sin() * 5.0, (2.0*i2+self.current_time).sin() * 5.0, (4.0*i2+self.current_time).sin() * 5.0];
            self.spheres[i as usize].center = [5.0 + offset[0], 10.0 + offset[1], -20.0 + offset[2]];
        }
    }

}