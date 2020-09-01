#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};
use std::fs;


mod cbt_json;
mod cbt_ini;
mod cbt_yaml;

fn main(){
    let arg_vec = vec!["poly", "morph", "json", "test.txt", "to", "yaml", "bla.txt"];

    let possible_formats = vec!["json", "toml", "yaml", "ini"];

    let matches = App::new("poly")
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(SubCommand::with_name("morph")
            .arg(Arg::with_name("from_type")
                .help("")
                .required(true)
                .possible_values(&possible_formats))
            .arg(Arg::with_name("from_file")
                .help("")
                .required(true))
            .subcommand(SubCommand::with_name("to")
                    .arg(Arg::with_name("to_type")
                        .help("")
                        .required(true)
                        .possible_values(&possible_formats))
                    .arg(Arg::with_name("to_file")
                        .help("")
                        .required(true))))
        //.get_matches();
        .get_matches_from(arg_vec);

    if let Some(submatches) = matches.subcommand_matches("morph")
    {
        if submatches.is_present("from_type")
        {
            println!("{}", submatches.value_of("from_type").expect("No value"));
        }
        if submatches.is_present("from_file")
        {
            let path = submatches.value_of("from_file").expect("No value");
            println!("{}", path);
            load_file(path);
        }
        if let Some(submatches) = submatches.subcommand_matches("to")
        {
            if submatches.is_present("to_type")
            {
                println!("{}", submatches.value_of("to_type").expect("No value"));
            }
            if submatches.is_present("to_file")
            {
                let path = submatches.value_of("to_file").expect("No value");
                println!("{}", path);
                save_file(path);
            }
        }
    }
    
}


fn load_file(path: &str){
    println!("Loading file : {}", path);
}

fn save_file(path: &str){
    println!("Savign file : {}", path);
}

fn file_validator(v: String) -> Result<(), String>{
    let metadata = fs::metadata(v);
    match metadata {
        Ok(file) =>  if file.is_file(){ return Ok(())} else { Err(String::from("The value is not a valid file")) } 
        Err(error) => Err(String::from(format!("{}", error)))
    }
}

// Ask what a good way to test file validation would be
