use std::fs::File;
use std::io::Write;
use std::borrow::BorrowMut;

pub struct Report {
    pub(crate) data: Vec<String>,
    pub(crate) name: String,
}

impl Report {
    #[warn(unused_must_use)]
    pub fn new() -> Self {
        let r = Report {
            data: vec![],
            name: "".to_string(),
        };
        return r;
    }
    fn split_lines(&self, data: String) -> Vec<String> {
        let lines: Vec<&str> = data.lines().collect();
        let mut lines_s: Vec<String> = vec![];

        for i in lines {
            if !i.is_empty() {
                lines_s.push(i.to_string());
            }
        }

        return lines_s;
    }
    fn split_by_whitespace_drop_first(&self, data: String) -> String {
        let mut split: Vec<&str> = data.split_whitespace().collect();
        let mut split_s: Vec<String> = vec![];
        split.remove(0);
        for i in split {
            if !i.is_empty() {
                split_s.push(i.to_string());
            }
        }
        return split_s.get(0).unwrap().clone();
    }
    fn add_data_to_vec(&self, d_type: String, epochs: String, eta: String, training_size: String, scale: String, min: String, zero: String, max: String, data: Vec<String>) -> Vec<String> {
        let mut line: Vec<String> = vec![d_type, epochs, eta, training_size, scale, min, zero, max];
        let mut data_c = data.clone();
        line.append(data_c.borrow_mut());
        return line;
    }
    fn make_string(&self, data: Vec<String>) -> String {
        let mut s = data.get(0).unwrap().clone();
        for i in 1..data.len() {
            s = s + ", " + &*data.get(i).unwrap().clone();
        }
        s = s + "\n";
        return s;
    }
    pub fn add(&mut self, d_type: String, epochs: String, eta: String, training_size: String, scale: String, min: String, zero: String, max: String, output: String) {
        println!("{}", output.clone());
        let lines = self.split_lines(output);
        let mut vec = vec![];
        for i in lines {
            vec.push(self.split_by_whitespace_drop_first(i.clone()));
        }
        let vectored_data = self.add_data_to_vec(d_type.clone(), epochs.clone(), eta.clone(), training_size.clone(), scale.clone(), min.clone(), zero.clone(), max.clone(), vec);
        self.data.push(self.make_string(vectored_data));
    }
    pub fn store(&self) {
        let mut file = File::create(self.name.clone() + ".report.csv").unwrap();
        for i in &self.data {
            file.write(i.as_bytes()).unwrap();
        }
    }
}