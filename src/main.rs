#![crate_name = "csvgen"]
#![feature(rand)]

extern crate clap;
extern crate rand;
extern crate time;

mod input;
use input::Parameters;

mod generator;
use generator::CsvLine;

fn main() {
    let params: Box<Parameters> = Box::new(Parameters::new().unwrap());
    let line = CsvLine::new(&params.column_types)
                        .line_value;
    println!("{}",line);
}