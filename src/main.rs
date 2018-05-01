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
use std::sync::Arc;

///MB to Byte
const MB: usize = 1024*1024;
///Threading threshold
const THREAD_NUM: usize = 5;
///Max size of output-file
const MAX_SIZE: usize = 100;

fn main() {
    let params: Arc<Parameters> = Arc::new(Parameters::new().unwrap());
    //Limit possible outputsize until checks for target filesystem are in place
    if &params.size > &MAX_SIZE {
        panic!("Requested output is too large.")
    }

    fn call(col_types: &'static Vec<String>, out_limit: &'static usize) 
        -> Result<usize, std::fmt::Error> {
        thread::spawn(move || {
            let mut output_counter: usize = 0;
            while &output_counter < &out_limit {
                    let line = CsvLine::new(&col_types)
                                    .line_value;
                output_counter += write_stdout(&line).unwrap();
            }
        });
        Ok(0)
    }

    for num in 1..THREAD_NUM {
        call(&params.column_types,&params.size);
    }
}