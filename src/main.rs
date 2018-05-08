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
use output::{setup_outfile,write_stdout,append_work_file};

use std::thread;
use std::path::Path;
use std::sync::{mpsc,Arc};
use std::sync::atomic::{AtomicBool,Ordering};

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

    let (sender, receiver) = mpsc::channel();
    let work_done = Arc::new(AtomicBool::new(false));

    let mut workers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(THREAD_NUM);
    let size_in_byte = (&params.size*&MB) / &THREAD_NUM;
    for _num in 1..THREAD_NUM+1 {
        let column_types = params.column_types.clone();
        let tx = sender.clone();
        let stop = work_done.clone();
        let worker = thread::spawn(move || {
            while ! stop.load(Ordering::Relaxed) {
                tx.send(CsvLine::new(&column_types)
                                .line_value).unwrap();
            }
        });
        workers.push(worker);
    };

    let mut output_counter: usize = 0;
    if &params.file_path == "stdout" {
        if params.header {
            let header_line: &str = &format!("{}\n",&params.column_types.join(","));
            write_stdout(&header_line).unwrap();
        }
        while &output_counter < &size_in_byte {
            let line: &str = &receiver.recv().unwrap();
            output_counter += write_stdout(&line).unwrap();
        }
    } else {
        let filepath: &Path = setup_outfile(&params.file_path).unwrap();
        if params.header {
            let header_line: &str = &format!("{}\n",&params.column_types.join(","));
            append_work_file(&header_line, &filepath).unwrap();
        }
        while &output_counter < &size_in_byte {
            let line: &str = &receiver.recv().unwrap();
            output_counter += append_work_file(&line,&filepath).unwrap();
        }
    }
    work_done.store(true, Ordering::Relaxed);
    for w in workers {
        w.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        use main;

        main();
    }
}