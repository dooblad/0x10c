pub mod collide;
pub mod math;

pub mod f32 {
    pub fn abs(a: f32) -> f32 {
        if a < 0.0 {
            -a
        } else {
            a
        }
    }

    pub fn clamp(v: f32, low: f32, high: f32) -> f32 {
        if v < low {
            low
        } else if v > high {
            high
        } else {
            v
        }
    }
}
