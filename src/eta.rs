use crate::run;
use crate::report::Report;

pub(crate) trait ETA {
    fn eta(writer: &mut Report, args: Vec<String>, lower: Self, offset: Self, upper: Self, zero: Self, r_zero: Self, scale: f64, epochs: usize, name: String);
}

impl ETA for i8 {
    fn eta(writer: &mut Report, args: Vec<String>, lower: Self, offset: Self, upper: Self, zero: Self, r_zero: Self, scale: f64, epochs: usize, name: String) {
        let eta: Vec<Self> = vec![1, 2, 4, 8, 16, 32, 64];
        for i in eta {
            run(writer, args.clone(), lower.clone(), offset, upper.clone(), zero.clone(), r_zero.clone(), scale.clone(), epochs.clone(), i, name.clone());
        }
    }
}

impl ETA for i16 {
    fn eta(writer: &mut Report, args: Vec<String>, lower: Self, offset: Self, upper: Self, zero: Self, r_zero: Self, scale: f64, epochs: usize, name: String) {
        let eta: Vec<Self> = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
        for i in eta {
            run(writer, args.clone(), lower.clone(), offset, upper.clone(), zero.clone(), r_zero.clone(), scale.clone(), epochs.clone(), i, name.clone());
        }
    }
}

impl ETA for i32 {
    fn eta(writer: &mut Report, args: Vec<String>, lower: Self, offset: Self, upper: Self, zero: Self, r_zero: Self, scale: f64, epochs: usize, name: String) {
        let eta: Vec<Self> = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
        for i in eta {
            run(writer, args.clone(), lower.clone(), offset, upper.clone(), zero.clone(), r_zero.clone(), scale.clone(), epochs.clone(), i, name.clone());
        }
    }
}

impl ETA for i64 {
    fn eta(writer: &mut Report, args: Vec<String>, lower: Self, offset: Self, upper: Self, zero: Self, r_zero: Self, scale: f64, epochs: usize, name: String) {
        let eta: Vec<Self> = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
        for i in eta {
            run(writer, args.clone(), lower.clone(), offset, upper.clone(), zero.clone(), r_zero.clone(), scale.clone(), epochs.clone(), i, name.clone());
        }
    }
}

impl ETA for i128 {
    fn eta(writer: &mut Report, args: Vec<String>, lower: Self, offset: Self, upper: Self, zero: Self, r_zero: Self, scale: f64, epochs: usize, name: String) {
        let eta: Vec<Self> = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
        for i in eta {
            run(writer, args.clone(), lower.clone(), offset, upper.clone(), zero.clone(), r_zero.clone(), scale.clone(), epochs.clone(), i, name.clone());
        }
    }
}

impl ETA for f32 {
    fn eta(writer: &mut Report, args: Vec<String>, lower: Self, offset: Self, upper: Self, zero: Self, r_zero: Self, scale: f64, epochs: usize, name: String) {
        let eta: Vec<Self> = vec![0.1, 0.5, 1., 10., 1000., 10000., 100000.];
        for i in eta {
            run(writer, args.clone(), lower.clone(), offset, upper.clone(), zero.clone(), r_zero.clone(), scale.clone(), epochs.clone(), i, name.clone());
        }
    }
}

impl ETA for f64 {
    fn eta(writer: &mut Report, args: Vec<String>, lower: Self, offset: Self, upper: Self, zero: Self, r_zero: Self, scale: f64, epochs: usize, name: String) {
        let eta: Vec<Self> = vec![0.1, 0.5, 1., 10., 1000., 10000., 100000., 1000000.];
        for i in eta {
            run(writer, args.clone(), lower.clone(), offset, upper.clone(), zero.clone(), r_zero.clone(), scale.clone(), epochs.clone(), i, name.clone());
        }
    }
}