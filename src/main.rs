#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};
use std::fs;


mod cbt_json;
mod cbt_ini;
mod cbt_yaml;
mod cbt_toml;

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



    //steps
    //
    //1. clap
    //2. get values from clap
    //3. load file
    //4. convert format
    //5. save file
    //6. end

    if let Some(submatches) = matches.subcommand_matches("morph")
    {

        let format = submatches.value_of("from_type").expect("No value");
        println!("The format we are converting from is {}", format);
        let path = submatches.value_of("from_file").expect("No value");
        println!("The file name is {}", path);
        let content = load_file(path);

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
            println!("The format we are converting to is {}", submatches.value_of("to_type").expect("No value"));
            let path = submatches.value_of("to_file").expect("No value");
            println!("The file we are saving to is {}", path);

            let serialized_content = match format{
                "json" => cbt_json::serialize_json(deserialized_content),
                "yaml" => cbt_yaml::serialize_yaml(deserialized_content),
                "toml" => cbt_toml::serialize_toml(deserialized_content),
                "ini" => cbt_ini::serialize_ini(deserialized_content),
                _ => panic!("You are not supposed to come here my friend"),
            };

            println!("{}", serialized_content);

            let result = save_file(path, serialized_content);
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

fn file_validator(v: String) -> Result<(), String>{
    let metadata = fs::metadata(v);
    match metadata {
        Ok(file) =>  if file.is_file(){ return Ok(())} else { Err(String::from("The value is not a valid file")) } 
        Err(error) => Err(String::from(format!("{}", error)))
    }
}

// Ask what a good way to test file validation would be
