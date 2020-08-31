use serde_json::{Value, json};
use serde::{Deserialize, Serialize};

pub fn json_to_rust(input: String) -> Value{
    match serde_json::from_str(&input){
        Ok(v) => v,
        Err(e) => panic!("Wrong format or {}", e) 
    }
}

pub fn rust_to_json<T: serde::ser::Serialize>(input: T) -> String{
    match serde_json::to_string(&input){
        Ok(v) => v,
        Err(e) => panic!("Wrong format or {}", e)
    }
}


#[derive(Serialize, Deserialize)]
struct Person<'a>{
    name: &'a str,
    age: i8,
    phones: Vec<&'a str>,
}

#[test]
fn json_to_rust_test()
{
    let json = String::from(
    "{
        \"name\": \"John Doe\",
        \"age\": 43,
        \"phones\": [
            \"+44 1234567\",
            \"+44 2345678\"
            ]
    }");

    let values = json_to_rust(json);

    assert!(values["name"] == "John Doe");
    assert!(values["age"] == 43);
    assert!(values["phones"][0] == "+44 1234567");
    assert!(values["phones"][1] == "+44 2345678");
}

#[test]
fn rust_to_json_struct_test()
{
    let json = String::from(
        "{\"name\":\"John Doe\",\"age\":43,\"phones\":[\"+44 1234567\",\"+44 2345678\"]}");


    let john = Person{
        name: "John Doe",
        age: 43,
        phones: vec!["+44 1234567", "+44 2345678"],
    };

    let john = rust_to_json(&john);

    assert_eq!(john, json);
}