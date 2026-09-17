use crate::search::Params;

pub struct LmrTable {
    base: [[[i32; 64]; 64]; 256],
}

impl LmrTable {
    pub fn init(&mut self) {
        let base = Params::lmr_base() as f32 / 100.0;
        let divisor = Params::lmr_div() as f32 / 100.0;
        for depth in 1..256 {
            for normal in 1..64 {
                for duck in 1..64 {
                    let reduction = base
                        + (depth as f32).ln() * (normal as f32).ln() * (duck as f32).ln() / divisor;
                    self.base[depth][normal][duck] = reduction as i32;
                }
            }
        }
    }

    #[inline]
    pub fn base(&self, depth: i32, normal: usize, duck: usize) -> i32 {
        self.base[depth.clamp(0, 255) as usize][normal.min(63)][duck.min(63)]
    }
}
