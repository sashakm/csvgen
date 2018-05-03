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
///Max worker threads
const THREAD_NUM: usize = 2;
///Max size of output-file
const MAX_SIZE: usize = 100;

fn main() {
    let params: Parameters = Parameters::new().unwrap();
    //Limit possible outputsize until checks for target filesystem are in place
    if &params.size > &MAX_SIZE {
        panic!("Requested output is too large.")
    }

    fn call(params: Parameters)-> thread::Result<usize> {
        thread::spawn(move || {
            println!("thread spawned");
            let mut output_counter: usize = 0;
            let size_in_byte = (&params.size*&MB) / &THREAD_NUM;
            while &output_counter < &size_in_byte {
                    let line = CsvLine::new(&params.column_types)
                                    .line_value;
                output_counter += write_stdout(&line).unwrap();
            }
        });
        Ok(0)
    }

    for num in 1..THREAD_NUM+1 {
        println!("Thread init");
        let p = params.clone();
        call(p);
    }
}