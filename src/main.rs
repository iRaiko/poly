#[macro_use]
extern crate clap;
use clap::{Arg, App};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;


mod cbt_json;

fn main(){
    let example_loc = r"C:\Users\Raiko\Desktop\Temporary_test_files\example.txt";
    let new_loc = r"C:\Users\Raiko\Desktop\Temporary_test_files\result.txt";
    let arg_vec = vec!["fake", example_loc, new_loc, "json", "json", "-a"];

    let matches = App::new("poly")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(Arg::with_name("input_path")
            .help("The file from which to convert from")
            .index(1)
            .required(true)
            .validator(file_validator))
        .arg(Arg::with_name("output_path")
            .help("The file location of the converted input file")
            .index(2)
            .required(true))
        .arg(Arg::with_name("from_format")
            .help("What format the input file is converted from")
                .required(true)
        .possible_values(&["yaml", "toml", "json", "ini"]))
        .arg(Arg::with_name("to_format")
            .help("What format the input file is converted too")
            .required(true)
            .possible_values(&["yaml", "toml", "json", "ini"]))
        .arg(Arg::with_name("append")
            .help("Append to end of file instead of overwriting")
            .short("a")
            .long("append"))
        //.get_matches();
        .get_matches_from(arg_vec);

    //read input file
    let input_path = matches.value_of("input_path").unwrap();

    let formatted_content = match fs::read_to_string(input_path){
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    //format to rust
    let rust_content = match matches.value_of("from_format").unwrap(){
        "json" => cbt_json::json_to_rust(formatted_content),
        "yaml" => unimplemented!(),
        "ini" => unimplemented!(),
        "toml" => unimplemented!(),
        _ => unimplemented!(),
        };

    //create/open output file
    let output_path = matches.value_of("output_path").unwrap();

    let mut buffer = match OpenOptions::new()
        .write(true)
        .append(matches.is_present("append"))
        .truncate(!matches.is_present("append"))
        .create(true)
        .open(output_path){
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

    //map rust to format
    let formatted_content = match matches.value_of("to_format").unwrap(){
        "json" => cbt_json::rust_to_json(rust_content),
        "yaml" => unimplemented!(),
        "ini" => unimplemented!(),
        "toml" => unimplemented!(),
        _ => unimplemented!(),
        };

    //write to output file
    buffer.write_all(formatted_content.as_bytes()).expect("aaaaaaaaaa");
}



fn file_validator(v: String) -> Result<(), String>{
    let metadata = fs::metadata(v);
    match metadata {
        Ok(file) =>  if file.is_file(){ return Ok(())} else { Err(String::from("The value is not a valid file")) } 
        Err(error) => Err(String::from(format!("{}", error)))
    }
}

// Ask what a good way to test file validation would be
