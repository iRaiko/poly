#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand, AppSettings};
use std::fs;
use std::fs::OpenOptions;
use std::io::stdin;

mod cbt_json;
mod cbt_ini;
mod cbt_yaml;
mod cbt_toml;
mod error;

fn main(){
    let _arg_vec = vec!["poly", "morph", "json", "test.txt", "to", "yaml", "bla.txt"];

    let possible_formats = vec!["json", "toml", "yaml", "ini"];

    let matches = App::new("poly")
        .author(crate_authors!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequired)
        .help(
"
Polymorh

Poly v1

Usage:
    poly.exe morph <from_format> <from_file> to <to_format> <to_file>
Example: 
    poly.exe morph json test.txt to yaml converted.txt

Subcommands:
    morph
    to

Arguments:
    <from_format>   Set the format from which to convert the file from with possible values:
                    [json, toml, yaml, ini]
    <from_file>     Set the file from which to convert from.
                    Example: C:\\Users\\Harry Potter\\example.txt
    <to_format>     Set the format from which to convert the file to with possible values:
                    [json, toml, yaml, ini]
    <to_file>       The location for the file the conversion will be saved to.
                    Example: C:\\Users\\Harry Potter\\converted_example.txt

"
    )
        .usage("poly.exe morph <from_format> <from_file> to <to_format> <to_file>\n    Example: poly.exe morph json test.txt to yaml converted.txt")
        .subcommand(SubCommand::with_name("morph")
            .setting(AppSettings::SubcommandRequired)
            .usage("poly.exe morph <from_format> <from_file> to <to_format> <to_file>")
            .arg(Arg::with_name("from_type")
                .help("Set the format from which to convert the file from.\nPossible values:")
                .required(true)
                .possible_values(&possible_formats))
            .arg(Arg::with_name("from_file")
                .help("Set the file from which to convert from.\nExample: C:\\Users\\Harry Potter\\example.txt")
                .required(true)
                .validator(from_file_validator))
            .subcommand(SubCommand::with_name("to")
                .usage("poly.exe morph <from_format> <from_file> to <to_format> <to_file>")
                .arg(Arg::with_name("to_type")
                    .help("Set the format from which to convert the file to.\nPossible values:")
                    .required(true)
                    .possible_values(&possible_formats))
                .arg(Arg::with_name("to_file")
                    .help("The location for the file the conversion will be saved to.\nExample: C:\\Users\\Harry Potter\\converted_example.txt")
                    .required(true)
                    .validator(to_file_validator))))
        .get_matches();
        //.get_matches_from(arg_vec);

    if let Some(submatches) = matches.subcommand_matches("morph")
    {

        let format = submatches.value_of("from_type").expect("No value");
        println!("The format we are converting from is {}", format);
        let path = submatches.value_of("from_file").expect("No value");
        println!("The file name is {}", path);
        let content = load_file(path);

        println!("{}", content);

        let deserialized_content = match format{
            "json" => cbt_json::deserialize_json(content),
            "yaml" => cbt_yaml::deserialize_yaml(content),
            "toml" => cbt_toml::deserialize_toml(content),
            "ini" => cbt_ini::deserialize_ini(content),
            _ => panic!("You are not supposed to come here my friend"),
        };

        println!("{}", deserialized_content);

        if let Some(submatches) = submatches.subcommand_matches("to")
        {
            let new_format = submatches.value_of("to_type").expect("No value");
            println!("The format we are converting to is {}", new_format);
            let new_path = submatches.value_of("to_file").expect("No value");
            println!("The file we are saving to is {}", new_path);

            let serialized_content = match new_format{
                "json" => cbt_json::serialize_json(deserialized_content),
                "yaml" => cbt_yaml::serialize_yaml(deserialized_content),
                "toml" => cbt_toml::serialize_toml(deserialized_content),
                "ini" => cbt_ini::serialize_ini(deserialized_content),
                _ => panic!("You are not supposed to come here my friend"),
            };

            println!("{}", serialized_content);

            let result = save_file(new_path, serialized_content);
            println!("{}", result);
        }
    }

}


fn load_file(path: &str) -> String{
    println!("Loading file : {}", path);
    String::from("Loaded file")
}

fn save_file(path: &str, content: String) -> String{
    println!("Saving Content: {} to file : {}", content,  path);
    String::from("Saved file")
}

fn to_file_validator(file_path: String) -> Result<(), String>
{
    match file_validator(&file_path){
        Ok(()) => Ok(()),
        Err(e) => match e
        {
            error::CustomError::File => Err(String::from(format!("{}", error::CustomError::File))),
            error::CustomError::Io(_) => 
            {
                check_for_file_creation(&file_path)
            }
        }
    }
}

fn check_for_file_creation(file_path: &str) -> Result<(), String>
{
    println!("File not found, do you want to create the file: (Y/N)");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    if input.trim().to_ascii_lowercase() == "y"
    {
        match OpenOptions::new().write(true).create(true).open(file_path)
        {
            Ok(_) => Ok(()),
            Err(e) => return Err(String::from(format!("{}", e))),
        }
    }
    else{
        Err(String::from("Will not create the file and exit"))
    }


}

fn from_file_validator(file_path: String) -> Result<(), String>
{
    match file_validator(&file_path)
    {
        Ok(()) => Ok(()),
        Err(e) => Err(String::from(format!("{}", e)))
    }
}

fn file_validator(file_path: &str) -> Result<(), error::CustomError>{
    let metadata = fs::metadata(file_path);
    match metadata {
        Ok(file) =>  
            if file.is_file()
            { 
                return Ok(())
            } else { 
                Err(error::CustomError::File)
            } 
        Err(error) => Err(error::CustomError::Io(error))
    }
}

// Ask what a good way to test file validation would be
