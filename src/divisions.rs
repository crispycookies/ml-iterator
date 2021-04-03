use crate::run_epochs;

pub(crate) trait Divisions {
    fn run_division(scale: f64, args: Vec<String>, base: Self, name: String);
}

impl Divisions for u8 {
    fn run_division(scale: f64, args: Vec<String>, base: Self, name: String) {
        let offset: Vec<Self> = vec![1, 2, 7, 15, 31, 63];
        for i in offset {
            let upper = base + 2 * i;
            let zero = base + i;
            let r_zero = 0;

            run_epochs(args.clone(), base.clone(), base.clone(), zero, upper, r_zero, scale.clone(), name.clone());
        }
    }
}

impl Divisions for u16 {
    fn run_division(scale: f64, args: Vec<String>, base: Self, name: String) {
        let offset: Vec<Self> = vec![1, 2, 7, 15, 31, 63, 163];
        for i in offset {
            let upper = base + 2 * i;
            let zero = base + i;
            let r_zero = 0;

            run_epochs(args.clone(), base.clone(), base.clone(), zero, upper, r_zero, scale.clone(), name.clone());
        }
    }
}

impl Divisions for u32 {
    fn run_division(scale: f64, args: Vec<String>, base: Self, name: String) {
        let offset: Vec<Self> = vec![1, 2, 7, 15, 31, 63, 163];
        for i in offset {
            let upper = base + 2 * i;
            let zero = base + i;
            let r_zero = 0;

            run_epochs(args.clone(), base.clone(), base.clone(), zero, upper, r_zero, scale.clone(), name.clone());
        }
    }
}

impl Divisions for u64 {
    fn run_division(scale: f64, args: Vec<String>, base: Self, name: String) {
        let offset: Vec<Self> = vec![1, 2, 7, 15, 31, 63, 163];
        for i in offset {
            let upper = base + 2 * i;
            let zero = base + i;
            let r_zero = 0;

            run_epochs(args.clone(), base.clone(), base.clone(), zero, upper, r_zero, scale.clone(), name.clone());
        }
    }
}

impl Divisions for u128 {
    fn run_division(scale: f64, args: Vec<String>, base: Self, name: String) {
        let offset: Vec<Self> = vec![1, 2, 7, 15, 31, 63, 163];
        for i in offset {
            let upper = base + 2 * i;
            let zero = base + i;
            let r_zero = 0;

            run_epochs(args.clone(), base.clone(), base.clone(), zero, upper, r_zero, scale.clone(), name.clone());
        }
    }
}

impl Divisions for f32 {
    fn run_division(scale: f64, args: Vec<String>, base: Self, name: String) {
        let offset: Vec<Self> = vec![0.00000001, 0.00001, 0.001, 0.25, 0.5, 1., 2., 5., 10., f32::MAX - 1., f32::MAX - 1.];
        for i in offset {
            let upper = base + 2. * i;
            let zero = base + i;
            let r_zero = 0.;

            run_epochs(args.clone(), base.clone(), base.clone(), zero, upper, r_zero, scale.clone(), name.clone());
        }
    }
}

impl Divisions for f64 {
    fn run_division(scale: f64, args: Vec<String>, base: Self, name: String) {
        let offset: Vec<Self> = vec![0.00000001, 0.00001, 0.001, 0.25, 0.5, 1., 2., 5., 10., f64::MAX - 1., f64::MAX - 1.];
        for i in offset {
            let upper = base + 2. * i;
            let zero = base + i;
            let r_zero = 0.;

            run_epochs(args.clone(), base.clone(), base.clone(), zero, upper, r_zero, scale.clone(), name.clone());
        }
    }
}