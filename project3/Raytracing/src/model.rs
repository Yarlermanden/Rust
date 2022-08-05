use std::time::{SystemTime, UNIX_EPOCH};


#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Model {
    current_time: f32,
}

impl Model {
    pub fn new<>(
    ) -> Self {
        Self {
            current_time: 0.0,
        }
    }

    pub fn update_current_time(&mut self) {
        let temp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
        let temp1 = (temp % (std::u32::MAX-1) as u128) as u32;
        self.current_time = ((temp1 as usize) >> 4) as f32 / 16.0;
        println!("{}", self.current_time);
    }
}