#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};
use std::fs;


mod cbt_json;
mod cbt_ini;
mod cbt_yaml;

fn main(){
    let example_loc = r"C:\Users\Raiko\Desktop\Temporary_test_files\example.txt";
    let new_loc = r"C:\Users\Raiko\Desktop\Temporary_test_files\result.txt";
    let arg_vec = vec!["poly", "morph", "json", "test.txt", "to", "yaml", "bla.txt"];

    let matches = App::new("poly")
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(SubCommand::with_name("morph")
            .arg(Arg::with_name("from_type")
                .possible_values(&["json", "toml", "yaml", "ini"]))
            .arg(Arg::with_name("from_file"))
                .subcommand(SubCommand::with_name("to")
                    .arg(Arg::with_name("to_type")
                        .possible_values(&["json", "toml", "yaml", "ini"]))
                    .arg(Arg::with_name("to_file"))))
        .get_matches();
        //.get_matches_from(arg_vec);

    if let Some(submatches) = matches.subcommand_matches("morph")
    {
        println!("{}", submatches.value_of("from_type").expect("No value"));
        println!("{}", submatches.value_of("from_file").expect("No value"));

        if let Some(submatches) = submatches.subcommand_matches("to")
        {
            println!("{}", submatches.value_of("to_type").expect("No value"));
            println!("{}", submatches.value_of("to_file").expect("No value"));
        }
    }
    
}



fn _file_validator(v: String) -> Result<(), String>{
    let metadata = fs::metadata(v);
    match metadata {
        Ok(file) =>  if file.is_file(){ return Ok(())} else { Err(String::from("The value is not a valid file")) } 
        Err(error) => Err(String::from(format!("{}", error)))
    }
}

// Ask what a good way to test file validation would be
