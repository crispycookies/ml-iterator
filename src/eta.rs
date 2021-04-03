use crate::run;

pub(crate) trait ETA {
    fn eta(args: Vec<String>, lower: Self, offset: Self, upper: Self, zero: Self, r_zero: Self, scale: f64, epochs: usize, name: String);
}

impl ETA for u8 {
    fn eta(args: Vec<String>, lower: Self, offset: Self, upper: Self, zero: Self, r_zero: Self, scale: f64, epochs: usize, name: String) {
        let eta: Vec<Self> = vec![1, 2, 4, 8, 16, 32, 64];
        for i in eta {
            run(args.clone(), lower.clone(), offset, upper.clone(), zero.clone(), r_zero.clone(), scale.clone(), epochs.clone(), i, name.clone());
        }
    }
}

impl ETA for u16 {
    fn eta(args: Vec<String>, lower: Self, offset: Self, upper: Self, zero: Self, r_zero: Self, scale: f64, epochs: usize, name: String) {
        let eta: Vec<Self> = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
        for i in eta {
            run(args.clone(), lower.clone(), offset, upper.clone(), zero.clone(), r_zero.clone(), scale.clone(), epochs.clone(), i, name.clone());
        }
    }
}

impl ETA for u32 {
    fn eta(args: Vec<String>, lower: Self, offset: Self, upper: Self, zero: Self, r_zero: Self, scale: f64, epochs: usize, name: String) {
        let eta: Vec<Self> = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
        for i in eta {
            run(args.clone(), lower.clone(), offset, upper.clone(), zero.clone(), r_zero.clone(), scale.clone(), epochs.clone(), i, name.clone());
        }
    }
}

impl ETA for u64 {
    fn eta(args: Vec<String>, lower: Self, offset: Self, upper: Self, zero: Self, r_zero: Self, scale: f64, epochs: usize, name: String) {
        let eta: Vec<Self> = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
        for i in eta {
            run(args.clone(), lower.clone(), offset, upper.clone(), zero.clone(), r_zero.clone(), scale.clone(), epochs.clone(), i, name.clone());
        }
    }
}

impl ETA for u128 {
    fn eta(args: Vec<String>, lower: Self, offset: Self, upper: Self, zero: Self, r_zero: Self, scale: f64, epochs: usize, name: String) {
        let eta: Vec<Self> = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
        for i in eta {
            run(args.clone(), lower.clone(), offset, upper.clone(), zero.clone(), r_zero.clone(), scale.clone(), epochs.clone(), i, name.clone());
        }
    }
}

impl ETA for f32 {
    fn eta(args: Vec<String>, lower: Self, offset: Self, upper: Self, zero: Self, r_zero: Self, scale: f64, epochs: usize, name: String) {
        let eta: Vec<Self> = vec![0.1, 0.5, 1., 10., 1000., 10000., 100000.];
        for i in eta {
            run(args.clone(), lower.clone(), offset, upper.clone(), zero.clone(), r_zero.clone(), scale.clone(), epochs.clone(), i, name.clone());
        }
    }
}

impl ETA for f64 {
    fn eta(args: Vec<String>, lower: Self, offset: Self, upper: Self, zero: Self, r_zero: Self, scale: f64, epochs: usize, name: String) {
        let eta: Vec<Self> = vec![0.1, 0.5, 1., 10., 1000., 10000., 100000., 1000000.];
        for i in eta {
            run(args.clone(), lower.clone(), offset, upper.clone(), zero.clone(), r_zero.clone(), scale.clone(), epochs.clone(), i, name.clone());
        }
    }
}