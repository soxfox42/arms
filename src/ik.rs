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

    fn end_pos(&self) -> Vec2 {
        Vec2::ZERO
    }

    pub fn update_angles(&mut self, target: Vec2) {
        for _step in 0..MAX_IK_STEPS {
            for arm in 0..self.count {
                self.angles[arm] += arm as f32 / 1000.0 + 0.001;
            }
        }
    }

    pub fn arms(&self) -> Vec<(Vec2, Vec2)> {
        let mut current = Vec2::ZERO;
        let mut positions = vec![current];
        
        for (&angle, &length) in self.angles.iter().zip(&self.lengths) {
            let offset = Vec2::new(angle.cos(), angle.sin()) * length;
            current += offset;
            positions.push(current);
        }

        let mut arms = Vec::new();
        for arm in positions.windows(2) {
            arms.push((arm[0], arm[1]));
        }

        arms
    }
}


