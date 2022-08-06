#![allow(dead_code, unused_imports)]

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use std::convert::{TryInto};
use std::ops::{Add, Sub, AddAssign, SubAssign};
pub use std::time::*;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Model {
    current_time: f32,
    padding: f32,
    padding2: f32,
    padding3: f32,
}

impl Model {
    pub fn new<>(
    ) -> Self {
        Self {
            current_time: 0.0,
            padding: 0.0,
            padding2: 0.0,
            padding3: 0.0,
        }
    }

    pub fn update_current_time(&mut self, earlier: Instant) {
        let temp = Instant::now()
        .duration_since(earlier)
        //.as_millis();
        //.as_millis();
        .as_nanos();
        let temp1 = (temp % (std::u32::MAX-1) as u128) as u32;
        //self.current_time = ((temp1 as usize) >> 4) as f32 / 1000;
        //self.current_time = temp1 as f32 / 100000.0;
        self.current_time = (temp1 as f32 / 4300000000.00) * 6.3;
        //println!("{:.32}", temp1);
        println!("{} : {}", temp1, self.current_time);
    }

}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(std::time::Instant);

#[cfg(not(target_arch = "wasm32"))]
impl Instant {
    pub fn now() -> Self { Self(std::time::Instant::now()) }
    pub fn duration_since(&self, earlier: Instant) -> Duration { self.0.duration_since(earlier.0) }
    pub fn elapsed(&self) -> Duration { self.0.elapsed() }
    pub fn checked_add(&self, duration: Duration) -> Option<Self> { self.0.checked_add(duration).map(|i| Self(i)) }
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> { self.0.checked_sub(duration).map(|i| Self(i)) }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(inline_js = r#"
export function performance_now() {
  return performance.now();
}"#)]
extern "C" {
    fn performance_now() -> f64;
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(u64);

#[cfg(target_arch = "wasm32")]
impl Instant {
    pub fn now() -> Self { Self((performance_now() * 1000.0) as u64) }
    pub fn duration_since(&self, earlier: Instant) -> Duration { Duration::from_micros(self.0 - earlier.0) }
    pub fn elapsed(&self) -> Duration { Self::now().duration_since(*self) }
    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        match duration.as_micros().try_into() {
            Ok(duration) => self.0.checked_add(duration).map(|i| Self(i)),
            Err(_) => None,
        }
    }
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        match duration.as_micros().try_into() {
            Ok(duration) => self.0.checked_sub(duration).map(|i| Self(i)),
            Err(_) => None,
        }
    }
}