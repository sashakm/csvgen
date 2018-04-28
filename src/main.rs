extern crate clap;

mod input;
use input::Parameters;

fn main() {
    let params: Box<Parameters> = Box::new(Parameters::new().unwrap());

    println!("{:?}",params)
}