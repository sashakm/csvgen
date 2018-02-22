extern crate csv;
extern crate clap;
    

use std::io;
use std::process;
use std::error::Error;

use clap::{Arg, App, ArgMatches};

fn main() {
    let matches = App::new("csvgen")
                        .version("0.1.0")
                        .author("jkm <jkm@bricknet.de>")
                        .about("Generate mock csv-files of arbitrary size.")
                        .arg(Arg::with_name("size")
                             .short("s")
                             .long("size")
                             .value_name("Size")
                             .help("Specify size of file.")
                             .required(true)
                             .takes_value(true)
                             )
                        .arg(Arg::with_name("columns")
                             .short("c")
                             .long("cols")
                             .value_name("Columns")
                             .help("Specify number of columns.")
                             )
                        .get_matches();
    println!("{:?}",matches);
}
