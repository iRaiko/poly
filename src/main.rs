#[macro_use]
extern crate clap;
use clap::{Arg, App};
use std::fs;

mod cbt_json;

fn main(){
    let matches = App::new("poly")
    .author(crate_authors!())
    .version(crate_version!())
    .arg(Arg::with_name("input")
        .help("The file from which to convert from")
        .index(1)
        .required(true)
        .validator(file_validator))
    .arg(Arg::with_name("output")
        .help("The file location of the converted input file")
        .index(2)
        .required(true))
    .arg(Arg::with_name("from")
        .help("What format the input file is converted from")
        .required(true)
        .possible_values(&["yaml", "toml", "json", "ini"]))
    .arg(Arg::with_name("to")
        .help("What format the input file is converted too")
        .required(true)
        .possible_values(&["yaml", "toml", "json", "ini"]))
    .arg(Arg::with_name("append")
        .help("Append to end of file instead of overwriting")
        .short("a")
        .long("append"))
    .get_matches();
}


fn file_validator(v: String) -> Result<(), String>{
    let metadata = fs::metadata(v);
    match metadata {
        Ok(file) =>  if file.is_file(){ return Ok(())} else { Err(String::from("The value is not a valid file")) } 
        Err(error) => Err(String::from(format!("{}", error)))
    }
}

// Ask what a good way to test file validation would be
