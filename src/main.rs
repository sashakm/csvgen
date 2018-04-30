#![crate_name = "csvgen"]
#![feature(rand)]

extern crate clap;
extern crate rand;
extern crate time;

mod input;
use input::Parameters;

mod generator;
use generator::CsvLine;

///MB to Byte
pub const MB: usize = 1024*1024;


fn main() {
    let params: Box<Parameters> = Box::new(Parameters::new().unwrap());
    let line = CsvLine::new(&params.column_types)
                        .line_value;
    println!("{}",line);
}