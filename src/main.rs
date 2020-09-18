#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand, AppSettings};
use std::fs;
use std::fs::OpenOptions;
use std::io::stdin;

mod error;

fn main(){
    let _arg_vec = vec!["poly", "morph", "doml", r"C:\Users\Raiko\Desktop\Temporary_test_files\result.txt", "to", "json", r"C:\Users\Raiko\Desktop\Temporary_test_files\bla.txt"];

    let possible_formats = vec!["json", "doml", "yaml", "ini"];

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
        let path = submatches.value_of("from_file").expect("No value");
        let content = load_file(path);

        let deserialized_content = match format{
            "json" => cbt::cbt_json::from_json(&content),
            "yaml" => cbt::cbt_yaml::from_yaml(&content),
            "doml" => cbt::cbt_toml::from_toml(&content),
            _ => panic!("You are not supposed to come here my friend"),
        };

        if let Some(submatches) = submatches.subcommand_matches("to")
        {
            let new_format = submatches.value_of("to_type").expect("No value");
            let new_path = submatches.value_of("to_file").expect("No value");

            let serialized_content = match new_format{
                "json" => cbt::cbt_json::to_json(deserialized_content),
                "yaml" => cbt::cbt_yaml::to_yaml(deserialized_content),
                "doml" => cbt::cbt_toml::to_toml(deserialized_content),
                _ => panic!("You are not supposed to come here my friend"),
            };

            let result = save_file(new_path, serialized_content);
            println!("{}", result);
        }
    }
}


fn load_file(path: &str) -> String{
    match fs::read_to_string(path)
    {
        Ok(content) => content,
        Err(e) => panic!("{}", e)
    }
}

fn save_file(path: &str, content: String) -> String{
    match fs::write(path, content)
    {
        Ok(_) => String::from("Succesfull save"),
        Err(e) => panic!("{}", e)
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

fn to_file_validator(file_path: String) -> Result<(), String>
{
    match file_validator(&file_path)
    {
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
    match metadata 
    {
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


mod cbt
{
    use std::collections::HashMap;

    pub enum RustValue
    {
        Null,
        Bool(bool),
        Integer(i64),
        Float(f64),
        String(String),
        Array(Vec<RustValue>),
        Object(HashMap<String, RustValue>)
    }


    pub mod cbt_json
    {
        pub fn from_json(json_content: &str) -> super::RustValue
        {
            let json = serde_json::from_str(json_content).unwrap();
            json_to_rust(json)
        }

        pub fn to_json(rust: super::RustValue) -> String
        {
            let json = rust_to_json(rust);
            serde_json::to_string_pretty(&json).unwrap()
        }

        fn json_to_rust(val: serde_json::Value) -> super::RustValue
        {
            match val
            {
                serde_json::Value::Null => super::RustValue::Null,
                serde_json::Value::Number(number) => 
                    {if let Some(int) = number.as_i64()
                        {
                            super::RustValue::Integer(int)
                        }
                    else{
                        if let Some(float) = number.as_f64()
                        {
                            super::RustValue::Float(float)
                        }
                        else{
                            super::RustValue::Null
                        }
                    }},
                serde_json::Value::String(string) => super::RustValue::String(string),
                serde_json::Value::Bool(bool) => super::RustValue::Bool(bool),
                serde_json::Value::Array(arr) => super::RustValue::Array(arr.into_iter().map(|val| json_to_rust(val)).collect()),
                serde_json::Value::Object(obj) => super::RustValue::Object(obj.into_iter().map(|(key, value)| return (String::from(key), json_to_rust(value))).collect())
            }
        }

        fn rust_to_json(val: super::RustValue) -> serde_json::Value
        {
            match val
            {
                super::RustValue::Null => serde_json::Value::Null,
                super::RustValue::Integer(number) => serde_json::Value::Number(serde_json::Number::from(number)),
                super::RustValue::Float(number) => serde_json::Value::Number(serde_json::Number::from_f64(number).unwrap()),
                super::RustValue::String(string) => serde_json::Value::String(string),
                super::RustValue::Bool(bool) => serde_json::Value::Bool(bool),
                super::RustValue::Array(arr) => serde_json::Value::Array(arr.into_iter().map(|val| rust_to_json(val)).collect()),
                super::RustValue::Object(obj) => serde_json::Value::Object(obj.into_iter().map(|(key, value)| return (String::from(key), rust_to_json(value))).collect())
            }
        }

    }

    pub mod cbt_yaml
    {
        pub fn from_yaml(yaml_content: &str) -> super::RustValue
        {
            let yaml = serde_yaml::from_str(yaml_content).unwrap();
            yaml_to_rust(yaml)
        }

        pub fn to_yaml(rust: super::RustValue) -> String
        {
            let yaml = rust_to_yaml(rust);
            serde_yaml::to_string(&yaml).unwrap()
        }

        fn rust_to_yaml(val: super::RustValue) -> serde_yaml::Value
        {
            match val
            {
                super::RustValue::Null => serde_yaml::Value::Null,
                super::RustValue::Integer(number) => serde_yaml::Value::Number(serde_yaml::Number::from(number)),
                super::RustValue::Float(number) => serde_yaml::Value::Number(serde_yaml::Number::from(number)),
                super::RustValue::String(string) => serde_yaml::Value::String(string),
                super::RustValue::Bool(bool) => serde_yaml::Value::Bool(bool),
                super::RustValue::Array(arr) => serde_yaml::Value::Sequence(arr.into_iter().map(|x| rust_to_yaml(x)).collect()),
                super::RustValue::Object(obj) => serde_yaml::Value::Mapping(obj.into_iter().map(|(key, value)| return (serde_yaml::Value::String(key), rust_to_yaml(value))).collect())
            }
        }
        
        fn yaml_to_rust(val: serde_yaml::Value) -> super::RustValue
        {
            match val
            {
                serde_yaml::Value::Null => super::RustValue::Null,
                serde_yaml::Value::Bool(bool) => super::RustValue::Bool(bool),
                serde_yaml::Value::Number(number) =>             
                {
                    if let Some(int) = number.as_i64()
                    {
                        super::RustValue::Integer(int)
                    }
                    else{
                        if let Some(float) = number.as_f64()
                        {
                            super::RustValue::Float(float)
                        }
                        else{
                            super::RustValue::Null
                        }
                    }},
                serde_yaml::Value::String(string) => super::RustValue::String(string),
                serde_yaml::Value::Sequence(arr) => super::RustValue::Array(arr.into_iter().map(|x| yaml_to_rust(x)).collect()),
                serde_yaml::Value::Mapping(obj) => super::RustValue::Object(obj.into_iter().map(|(key, value)| return ({if let serde_yaml::Value::String(v) = key{v}else{String::new()}}, yaml_to_rust(value))).collect()),
            }
        }

    }

    pub mod cbt_toml
    {
        pub fn from_toml(toml_content: &str) -> super::RustValue
        {
            let toml = toml::from_str(toml_content).unwrap();
            toml_to_rust(toml)
        }

        pub fn to_toml(rust: super::RustValue) -> String
        {
            let rust = rust_to_toml(rust);
            toml::to_string_pretty(&rust).unwrap()
        }

        fn rust_to_toml(val: super::RustValue) -> toml::Value
        {
            match val
            {
                super::RustValue::Null => toml::Value::String(String::from("Null")),
                super::RustValue::Integer(number) => toml::Value::Integer(number),
                super::RustValue::Float(number) => toml::Value::Float(number),
                super::RustValue::String(string) => toml::Value::String(string),
                super::RustValue::Bool(bool) => toml::Value::Boolean(bool),
                super::RustValue::Array(arr) => toml::Value::Array(arr.into_iter().map(|x| rust_to_toml(x)).collect()),
                super::RustValue::Object(obj) => toml::Value::Table(obj.into_iter().map(|(key, value)| return (key, rust_to_toml(value))).collect()),
            }
        }

        fn toml_to_rust(val: toml::Value) -> super::RustValue
        {
            match val
            {
                toml::Value::Integer(number) => super::RustValue::Integer(number),
                toml::Value::Float(number) => super::RustValue::Float(number),
                toml::Value::Boolean(bool) => super::RustValue::Bool(bool),
                toml::Value::String(string) => super::RustValue::String(string),
                toml::Value::Datetime(date) => super::RustValue::String(date.to_string()),
                toml::Value::Array(arr) => super::RustValue::Array(arr.into_iter().map(|x| toml_to_rust(x)).collect()),
                toml::Value::Table(obj) => super::RustValue::Object(obj.into_iter().map(|(key, value)| return (String::from(key), toml_to_rust(value))).collect()),
            }
        }
    }
}