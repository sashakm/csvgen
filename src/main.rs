#![crate_name = "csvgen"]
#![feature(rand)]

extern crate clap;
extern crate rand;
extern crate time;

mod input;
use input::Parameters;

mod generator;
use generator::CsvLine;

mod output;
use output::write_stdout;

use std::thread;

///MB to Byte
const MB: usize = 1024*1024;
///Threading threshold
const THREAD_NUM: usize = 5;
///Max size of output-file
const MAX_SIZE: usize = 100;

fn main() {
    let params: Box<Parameters> = Box::new(Parameters::new().unwrap());
    //Limit possible outputsize until checks for target filesystem are in place
    if &params.size > &MAX_SIZE {
        panic!("Requested output is too large.")
    }
    for num in 1..THREAD_NUM {
        thread::spawn(move || {
            let mut output_counter: usize = 0;
            let output_limit: usize = &params.size * &MB;
            while &output_counter < &output_limit {
                    let line = CsvLine::new(&params.column_types)
                                    .line_value;
                output_counter += write_stdout(&line).unwrap();
            }
        });
    }
}