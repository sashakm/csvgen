use clap::{Arg,App};

const MB: u64 = 1024*1024;

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
    
    let size: u64 = matches.value_of("size")
                      .unwrap()
                      .parse::<u64>()?
                      *MB;

    let headerVals: &str = matches.value_of("header-values")
                             .unwrap_or("string,float,int")
                             .parse::<&str>()?;

    let colTypes: Vec<&str> = matches.value_of("column-types")
                                 .unwrap_or("string,float,int")
                                 .parse::<&str>()
                                 .split(",")
                                 .collect();
}