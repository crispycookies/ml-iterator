use std::path::Path;
use std::process::Command;
use std::str::from_utf8;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::report::Report;

mod eta;
mod divisions;
mod report;
pub(crate) mod unchecked_ops;

#[derive(PartialEq, EnumIter)]
enum Types {
    U8T,
    U16T,
    U32T,
    U64T,
    U128T,
    F32T,
    F64T,
}

pub fn separate<T>(s: T) -> String
    where T: std::fmt::Display
{
    s.to_string()
}

fn start<T>(writer: &mut Report, program: String, file: String, def_weight: T, offset: T, lower: T, upper: T, zero: T, scale: f64, size: usize, t_size: usize, epochs: usize, eta: T, d_type: String)
    where T: std::fmt::Display + Clone
{
    // https://doc.rust-lang.org/std/process/struct.Command.html
    // Accessed 02.04.2021(dd.mm.yyyy) @ 00:01

    let m_def_weight = separate(def_weight);
    let m_upper = separate(upper.clone());
    let m_lower = separate(lower.clone());
    let m_zero = separate(zero);
    let m_offset = separate(offset);
    let m_size = separate(size);
    let m_data = separate(file);
    let m_o_file = d_type.clone() + &*Path::new(m_data.as_str()).file_stem().unwrap().to_str().unwrap().to_string() + "-type" + &*d_type + "-size" + size.to_string().as_str() +
        "-lower" + lower.clone().to_string().as_str() + "-upper" + upper.clone().to_string().as_str() + "-t_size" + t_size.to_string().as_str() + "-scaler" + scale.to_string().as_str();
    let m_training_size = separate(t_size);
    let m_epochs = separate(epochs);
    let m_eta = separate(eta);
    let m_scale = separate(scale);

    let all = d_type.clone() + " " + &*m_def_weight + " " + &*m_upper + " " + &*m_lower + " " + &*m_zero + " " + &*m_offset + " " + &*m_size + " " + &*m_data + " " + &*m_o_file + " " + &*m_training_size + " " + &*m_epochs + " " + &*m_eta + " " + &*m_scale;
    println!("{}", all);

    let output = {
        Command::new(program)
            .arg(d_type.clone())
            .arg(m_def_weight)
            .arg(m_upper.clone())
            .arg(m_lower.clone())
            .arg(m_zero.clone())
            .arg(m_offset)
            .arg(m_size)
            .arg(m_data)
            .arg(m_o_file)
            .arg(m_training_size.clone())
            .arg(m_epochs.clone())
            .arg(m_eta.clone())
            .arg(m_scale.clone())
            .output()
            .expect("failed to execute process")
    };

    match from_utf8(output.stderr.as_slice()) {
        Ok(e) => {
            println!("{}", e.to_string());
        }
        Err(_) => {
            println!("No Viable Output")
        }
    }
    match from_utf8(output.stdout.as_slice()) {
        Ok(e) => {
            println!("{}", e.to_string());
            writer.add(d_type.clone(), m_epochs.clone(), m_eta.clone(), m_training_size.clone(), m_scale.clone(), m_lower.clone(), m_zero.clone(), m_upper.clone(), e.to_string());
        }
        Err(_) => {
            println!("No Viable Output")
        }
    }
}

fn run<T>(writer: &mut Report, args: Vec<String>, lower: T, offset: T, upper: T, zero: T, r_zero: T, scale: f64, epochs: usize, eta: T, name: String)
    where T: std::fmt::Display + Clone
{
    let size = args.get(1).unwrap().parse::<usize>().unwrap();
    let t_size = args.get(2).unwrap().parse::<usize>().unwrap();
    start(writer, args.get(3).unwrap().clone(), args.get(4).unwrap().clone(), r_zero, offset, lower, upper, zero, scale, size, t_size, epochs, eta, name);
}


fn run_epochs<T>(writer: &mut Report, args: Vec<String>, lower: T, _base: T, upper: T, zero: T, r_zero: T, scale: f64, name: String)
    where T: std::fmt::Display + Clone + eta::ETA + std::ops::Sub<Output=T>
{
    let epoch_max = args.get(0).unwrap().parse::<usize>().unwrap();
    for i in 0..epoch_max {
        let mut offset = upper.clone();
        offset = offset - lower.clone();
        eta::ETA::eta(writer, args.clone(), lower.clone(), offset, upper.clone(), zero.clone(), r_zero.clone(), scale.clone(), i, name.clone())
    }
}

fn run_scale<T>(writer: &mut Report, args: Vec<String>, name: String, base: T)
    where T: std::fmt::Display + Clone + divisions::Divisions
{
    let divisions = vec![1., 2., 4., 8., 16., 32., 64., 128., 256., 512., 1024., 2048., 4096.];
    for i in divisions {
        divisions::Divisions::run_division(writer, i, args.clone(), base.clone(), name.clone());
    }
}

fn run_types(args: Vec<String>) {

    let mut writer = Report::new();
    const FOLDER: &str = "report";

    if !Path::exists(Path::new(FOLDER)) {
        std::fs::create_dir(FOLDER).expect("Failed to create required directory");
    }

    writer.name = FOLDER.to_string().clone();

    for i in Types::iter() {
        match i {
            Types::U8T => {
                run_scale::<u8>(&mut writer, args.clone(), "u8".to_string(), 0);
            }
            Types::U16T => {
                run_scale::<u16>(&mut writer, args.clone(), "u16".to_string(), 0);
            }
            Types::U32T => {
                run_scale::<u32>(&mut writer, args.clone(), "u32".to_string(), 0);
            }
            Types::U64T => {
                run_scale::<u64>(&mut writer, args.clone(), "u64".to_string(), 0);
            }
            Types::U128T => {
                run_scale::<u128>(&mut writer, args.clone(), "u128".to_string(), 0);
            }
            Types::F32T => {
                run_scale::<f32>(&mut writer, args.clone(), "f32".to_string(), -1.);
            }
            Types::F64T => {
                run_scale::<f64>(&mut writer, args.clone(), "f64".to_string(), -1.);
            }
        }
    }
    writer.store();
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    run_types(args);
}
