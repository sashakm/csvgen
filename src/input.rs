use clap::{Arg,App,Error};


#[derive(Debug)]
///This holds parsed commandline params.
pub struct Parameters {
    ///desired size of output
    pub size: usize,
    ///toggle header printing
    pub header: bool,
    ///custom column types
    pub column_types: Vec<String>,
    ///write output to this file
    pub file_path: String,
}

impl Parameters {
    /// Returns a Result containing a Parameters-struct.
    /// Fields are populated by parsing commandline parameters.
    /// ```
    /// use csvgen::input::Parameters;
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
                             .takes_value(false)
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
                         .parse::<usize>()
                         .unwrap(),

            header: matches.is_present("header"),

            column_types: matches.value_of("column-types")
                                .unwrap_or("string,float,int")
                                .split(",")
                                .map(|v| String::from(v))
                                .collect(),

            file_path: matches.value_of("filepath")
                                 .unwrap_or("stdout")
                                 .to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parameters() {
        use input::Parameters;
        let params = Parameters::new().expect("Could not parse parameters!");
        assert_eq!(params.file_path,String::from("stdout"));
        assert_eq!(params.column_types.len(),3);
        assert_eq!(params.header,false);
        assert_eq!(params.size,2);
    }
}