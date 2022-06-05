use glam::Vec2;

const MAX_IK_STEPS: i32 = 50;

pub struct Arms {
    count: usize,
    angles: Vec<f32>,
    lengths: Vec<f32>,
}

impl Arms {
    pub fn new(count: usize, arm_len: f32) -> Self {
        Self {
            count,
            angles: vec![0.0; count],
            lengths: vec![arm_len; count],
        }
    }

    pub fn set_arm_length(&mut self, arm_len: f32) {
        self.lengths = vec![arm_len; self.count]
    }

    fn positions(&self) -> Vec<Vec2> {
        let mut current = Vec2::ZERO;
        let mut current_angle = 0.0;
        let mut positions = vec![current];

        for (&angle, &length) in self.angles.iter().zip(&self.lengths) {
            current_angle += angle;
            let offset = Vec2::new(current_angle.cos(), current_angle.sin()) * length;
            current += offset;
            positions.push(current);
        }

        positions
    }

    pub fn update_angles(&mut self, target: Vec2) {
        for _step in 0..MAX_IK_STEPS {
            for arm in 0..self.count {
                let positions = self.positions();
                let pivot_pos = positions[arm];
                let end_pos = positions[self.count];
                if end_pos.distance(target) < 0.1 {
                    return;
                }

                let to_end = end_pos - pivot_pos;
                let to_target = target - pivot_pos;
                self.angles[arm] += to_end.angle_between(to_target);
            }
        }
    }

    pub fn arms(&self) -> Vec<(Vec2, Vec2)> {
        let mut arms = Vec::new();
        for arm in self.positions().windows(2) {
            arms.push((arm[0], arm[1]));
        }

        arms
    }
}
