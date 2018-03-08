extern crate csv;
extern crate clap;
    

use std::io;
use std::process;
use std::error::Error;

use clap::{Arg, App};

fn main() {
    let matches = App::new("csvgen")
                        .version("0.1.0")
                        .author("jkm <jkm@bricknet.de>")
                        .about("Generate mock csv-files of arbitrary size.")
                        .arg(Arg::with_name("size")
                             .short("s")
                             .long("size")
                             .value_name("Size")
                             .help("Specify size of output file in MB.")
                             .required(true)
                             .takes_value(true)
                             )
                        .arg(Arg::with_name("columns")
                             .short("c")
                             .long("cols")
                             .value_name("Columns")
                             .help("Specify a comma-separated list of column-headers.")
                             )
                        .get_matches();
    
    let size = matches.value_of("size").unwrap().parse::<u32>().unwrap()*1024*1024;
    let cols: Vec<&str> = matches.value_of("columns")
                                 .unwrap_or("foo,bar,baz")
                                 .split(",")
                                 .collect();

    //let row_size: u32 = size / cols.iter().count();
    println!("{:?},{:?}", size,cols);
}
