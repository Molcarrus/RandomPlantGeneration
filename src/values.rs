use bevy::prelude::*;

use rand::Rng;

pub const NUM: usize = 7;
pub const BRANCHES_MINMAX: [u8; 2] = [3, 6];
pub const HEIGHT_MINMAX: [u8; 2] = [2, 5];
pub const OFFSET_RATIO_MINMAX: [f32; 2] = [0.0, 1.0];
pub const ANGLE_MINMAX: [f32; 2] = [0.0, 0.5 * std::f32::consts::PI];
pub const SCALE_MINMAX: [f32; 2] = [0.4, 0.8];
pub const TRUNK_RADIUS_MINMAX: [f32; 2] = [0.1, 0.3];
pub const LEAF_RADIUS_MINMAX: [f32; 2] = [0.1, 0.5];
const VELOCITY_MAG: f32 = 0.5;
const ACCELERATION_MAG: f32 = 0.5;

#[derive(Debug, Resource)]
pub struct Values {
    pub branches: u8,
    pub height: u8,
    pub offset_ratio: f32,
    pub angle: f32,
    pub scaling: f32,
    pub trunk_radius: f32,
    pub leaf_radius: f32,
}

impl Default for Values {
    fn default() -> Self {
        Self {
            branches: 3,
            height: 2,
            offset_ratio: 1.0,
            angle: 0.5 * std::f32::consts::PI,
            scaling: 0.7,
            trunk_radius: 0.15,
            leaf_radius: 0.4,
        }
    }
}

#[derive(Resource)]
pub struct ValueVector {
    pub data: [f32; NUM],
    pub magnitude: Option<f32>,
}

impl Default for ValueVector {
    fn default() -> Self {
        Self {
            data: [0.0; NUM],
            magnitude: Some(VELOCITY_MAG),
        }
    }
}

impl ValueVector {
    pub fn to_values(&self) -> Values {
        Values {
            branches: ((BRANCHES_MINMAX[1] - BRANCHES_MINMAX[0]) as f32 * (self.data[0] + 1.0) * 0.5 + BRANCHES_MINMAX[0] as f32).round() as u8,
            height: ((HEIGHT_MINMAX[1] - HEIGHT_MINMAX[0]) as f32 * (self.data[1] + 1.0) * 0.5 + HEIGHT_MINMAX[0] as f32).round() as u8,
            offset_ratio: (OFFSET_RATIO_MINMAX[1] - OFFSET_RATIO_MINMAX[0]) * (self.data[2] + 1.0) * 0.5 + OFFSET_RATIO_MINMAX[0],
            angle: (ANGLE_MINMAX[1] - ANGLE_MINMAX[0]) as f32 * (self.data[3] + 1.0) * 0.5 + ANGLE_MINMAX[0],
            scaling: (SCALE_MINMAX[1] - SCALE_MINMAX[0]) as f32 * (self.data[4] + 1.0) * 0.5 + SCALE_MINMAX[0],
            trunk_radius: (TRUNK_RADIUS_MINMAX[1] - TRUNK_RADIUS_MINMAX[0]) as f32 * (self.data[5] + 1.0) * 0.5 + TRUNK_RADIUS_MINMAX[0],
            leaf_radius: (LEAF_RADIUS_MINMAX[1] - LEAF_RADIUS_MINMAX[0]) as f32 * (self.data[6] + 1.0) * 0.5 + LEAF_RADIUS_MINMAX[0],
        }
    }

    pub fn from_values(values: &Values) -> Self {
        Self {
            data: [
                2.0 * (values.branches - BRANCHES_MINMAX[0]) as f32 / (BRANCHES_MINMAX[1] - BRANCHES_MINMAX[0]) as f32 - 1.0,
                2.0 * (values.height - HEIGHT_MINMAX[0]) as f32 / (HEIGHT_MINMAX[1] - HEIGHT_MINMAX[0]) as f32 - 1.0,
                2.0 * (values.offset_ratio - OFFSET_RATIO_MINMAX[0]) / (OFFSET_RATIO_MINMAX[1] - OFFSET_RATIO_MINMAX[0]) - 1.0,
                2.0 * (values.angle - ANGLE_MINMAX[0]) / (ANGLE_MINMAX[1] - ANGLE_MINMAX[0]) - 1.0,
                2.0 * (values.scaling - SCALE_MINMAX[0]) / (SCALE_MINMAX[1] - SCALE_MINMAX[0]) - 1.0,
                2.0 * (values.trunk_radius - TRUNK_RADIUS_MINMAX[0]) / (TRUNK_RADIUS_MINMAX[1] - TRUNK_RADIUS_MINMAX[0]) - 1.0,
                2.0 * (values.leaf_radius - LEAF_RADIUS_MINMAX[0]) / (LEAF_RADIUS_MINMAX[1] - LEAF_RADIUS_MINMAX[0]) - 1.0,
            ],
            magnitude: None,
        }
    }

    pub fn add(&mut self, other: &mut Self) {
        for (x, y) in self.data.iter_mut().zip(other.data.iter_mut()) {
            *x += *y;

            if self.magnitude.is_none() {
                if *x < -1.0 || *x > 1.0 {
                    *y = -*y;
                    *x = x.clamp(-1.0, 1.0);
                }
            }
        }
    }

    fn normalize(&mut self) {
        if let Some(mag) = self.magnitude {
            let mut len2: f32 = 0.0;
            for d in self.data {
                len2 += d.powf(2.0);
            }

            assert!(len2 > 0.0);

            let len = len2.sqrt();
            for d in &mut self.data {
                *d /= len;
                *d *= mag;
            }
        }
    }

    pub fn nudge(&mut self) {
        let mut rng = rand::rng();
        let mut acceleration = Self {
            data: [0.0; NUM],
            magnitude: Some(ACCELERATION_MAG),
        };

        for d in &mut acceleration.data {
            *d = rng.random_range(-1.0..1.0);
        }

        acceleration.normalize();

        self.add(&mut acceleration);

        self.normalize();
    }
}