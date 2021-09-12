use crate::report::Report;
use crate::run_epochs;
use crate::unchecked_ops;


pub(crate) trait Divisions {
    fn run_division(writer: &mut Report, scale: f64, args: Vec<String>, base: Self, name: String);
}

impl Divisions for i8 {
    fn run_division(writer: &mut Report, scale: f64, args: Vec<String>, base: Self, name: String) {
        let offset: Vec<Self> = vec![1, 2, 7, 15, 31, 63];
        for i in offset {
            let upper = unchecked_ops::UncheckedOps::add(base, unchecked_ops::UncheckedOps::mul(i, 2));
            //let upper = base + 2 * i;
            let zero = unchecked_ops::UncheckedOps::add(base, i);
            //let zero = base + i;
            let r_zero = 0;

            run_epochs(writer, args.clone(), base.clone(), base.clone(), upper, zero, r_zero, scale.clone(), name.clone());
        }
    }
}

impl Divisions for i16 {
    fn run_division(writer: &mut Report, scale: f64, args: Vec<String>, base: Self, name: String) {
        let offset: Vec<Self> = vec![1, 2, 7, 15, 31, 63, 163, 255, 2047, 8191, 16384];
        for i in offset {
            let upper = unchecked_ops::UncheckedOps::add(base, unchecked_ops::UncheckedOps::mul(i, 2));
            //let upper = base + 2 * i;
            let zero = unchecked_ops::UncheckedOps::add(base, i);
            //let zero = base + i;
            let r_zero = 0;

            run_epochs(writer, args.clone(), base.clone(), base.clone(), zero, upper, r_zero, scale.clone(), name.clone());
        }
    }
}

impl Divisions for i32 {
    fn run_division(writer: &mut Report, scale: f64, args: Vec<String>, base: Self, name: String) {
        let offset: Vec<Self> = vec![1, 2, 7, 15, 31, 63, 163, 255, 2047, 8191, 16384, 32768, 2147483647];
        for i in offset {
            let upper = unchecked_ops::UncheckedOps::add(base, unchecked_ops::UncheckedOps::mul(i, 2));
            //let upper = base + 2 * i;
            let zero = unchecked_ops::UncheckedOps::add(base, i);
            //let zero = base + i;
            let r_zero = 0;

            run_epochs(writer, args.clone(), base.clone(), base.clone(), zero, upper, r_zero, scale.clone(), name.clone());
        }
    }
}

impl Divisions for i64 {
    fn run_division(writer: &mut Report, scale: f64, args: Vec<String>, base: Self, name: String) {
        let offset: Vec<Self> = vec![1, 2, 7, 15, 31, 63, 163, 255, 2047, 8191, 16384, 32768, 2147483647];
        for i in offset {
            let upper = unchecked_ops::UncheckedOps::add(base, unchecked_ops::UncheckedOps::mul(i, 2));
            //let upper = base + 2 * i;
            let zero = unchecked_ops::UncheckedOps::add(base, i);
            //let zero = base + i;
            let r_zero = 0;

            run_epochs(writer, args.clone(), base.clone(), base.clone(), zero, upper, r_zero, scale.clone(), name.clone());
        }
    }
}

impl Divisions for i128 {
    fn run_division(writer: &mut Report, scale: f64, args: Vec<String>, base: Self, name: String) {
        let offset: Vec<Self> = vec![1, 2, 7, 15, 31, 63, 163];
        for i in offset {
            let upper = unchecked_ops::UncheckedOps::add(base, unchecked_ops::UncheckedOps::mul(i, 2));
            //let upper = base + 2 * i;
            let zero = unchecked_ops::UncheckedOps::add(base, i);
            //let zero = base + i;
            let r_zero = 0;

            run_epochs(writer, args.clone(), base.clone(), base.clone(), zero, upper, r_zero, scale.clone(), name.clone());
        }
    }
}

impl Divisions for f32 {
    fn run_division(writer: &mut Report, scale: f64, args: Vec<String>, base: Self, name: String) {
        let offset: Vec<Self> = vec![0.00000001, 0.00001, 0.001, 0.25, 0.5, 1., 2., 5., 10.];
        for i in offset {
            let upper = unchecked_ops::UncheckedOps::add(base, unchecked_ops::UncheckedOps::mul(i, 2.));
            //let upper = base + 2 * i;
            let zero = unchecked_ops::UncheckedOps::add(base, i);
            //let zero = base + i;
            let r_zero = 0.;

            run_epochs(writer, args.clone(), base.clone(), base.clone(), zero, upper, r_zero, scale.clone(), name.clone());
        }
    }
}

impl Divisions for f64 {
    fn run_division(writer: &mut Report, scale: f64, args: Vec<String>, base: Self, name: String) {
        let offset: Vec<Self> = vec![0.00000001, 0.00001, 0.001, 0.25, 0.5, 1., 2., 5., 10.];
        for i in offset {
            let upper = unchecked_ops::UncheckedOps::add(base, unchecked_ops::UncheckedOps::mul(i, 2.));
            //let upper = base + 2 * i;
            let zero = unchecked_ops::UncheckedOps::add(base, i);
            //let zero = base + i;
            let r_zero = 0.;

            run_epochs(writer, args.clone(), base.clone(), base.clone(), zero, upper, r_zero, scale.clone(), name.clone());
        }
    }
}