use clap::{Arg,App,Error};

///MB to Byte
const MB: u64 = 1024*1024;

#[derive(Debug)]
///This holds parsed commandline params.
pub struct Parameters {
    ///desired size of output
    size: u64,
    ///toggle header printing
    header: bool,
    ///custom header values
    header_vals: String,
    ///custom column types
    column_types: Vec<String>,
    ///write output to this file
    file_path: String,
}

impl Parameters {
    /// Returns a Result containing a Parameters-struct.
    /// Fields are populated by parsing commandline parameters.
    /// ```
    /// use csvgen::Parameters
    /// let params = Parameters::new();
    /// ```
    pub fn new() -> Result<Parameters, Error> {
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
                        .arg(Arg::with_name("header")
                             .short("h")
                             .long("header")
                             .value_name("Header")
                             .help("Print column headers.")
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

        Ok(Parameters {
            size: matches.value_of("size")
                         .unwrap_or("2")
                         .parse::<u64>()
                         .unwrap() * MB,

            header: matches.value_of("header")
                           .unwrap_or("false")
                           .parse::<bool>()
                           .unwrap(),

            header_vals: String::from(
                            matches.value_of("header-values")
                                   .unwrap_or("string,float,int")),

            column_types: matches.value_of("column-types")
                                .unwrap_or("string,float,int")
                                .split(",")
                                .map(|v| String::from(v))
                                .collect(),

            file_path: String::from(
                          matches.value_of("filepath")
                                 .unwrap_or("stdout"))
        })
    }
}