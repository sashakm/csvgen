use clap::{Arg,App};

fn parse_args()
{
    let matches = App::new("csvgen")
                        .version("0.1.0")
                        .author("jkm <jkm@bricknet.de>")
                        .about("Generate mock csv-files of arbitrary size.")
                        .arg(Arg::with_name("size")
                             .short("s")
                             .long("size")
                             .value_name("Size")
                             .help("Specify size of output file in MB.")
                             .takes_value(true)
                             )
                        .arg(Arg::with_name("header-values")
                             .short("v")
                             .long("header-values")
                             .value_name("Header Values")
                             .help("Specify a string of comma-separated values to set header-names.")
                             .takes_value(true)
                             )
                        .arg(Arg::with_name("column-types")
                             .short("c")
                             .long("column-types")
                             .value_name("Column Types")
                             .help("Specify types for columns.")
                             .takes_value(true)
                            )
                        .arg(Arg::with_name("filepath")
                             .short("f")
                             .long("filepath")
                             .value_name("File path")
                             .help("Write to file path instead of stdout.")
                             .takes_value(true)
                            )
                        .get_matches();
    
    let size = matches.value_of("size")
                      .unwrap()
                      .parse::<u64>()
                      .expect("Value of size is not a number")
                      *MiB;

    let cols: Vec<&str> = matches.value_of("columns")
                                 .unwrap_or("foo,bar,baz")
                                 .split(",")
                                 .collect();
}