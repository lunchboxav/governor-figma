extern crate reqwest;
extern crate serde_json;
extern crate tokio;

use serde::Deserialize;
use serde_json::Value;

use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
#[serde(rename = "test")]
struct Color {
    a: f64,
    b: f64,
    g: f64,
    r: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let figma_token = "your-figma-token";
    let figma_node_id = "your-targeted-node-id";

    let res = reqwest::Client::new()
        .get("https://api.figma.com/v1/files/PJVO9QfGt35VBw7g44N03Z/nodes?ids=1278%3A1353")
        .header(
            "X-FIGMA-TOKEN",
            figma_token
        )
        .send()
        .await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;
    let f: Value = serde_json::from_str(&body)?;

    let mut file_buffer = File::create("token.js")?;

    let color_list = f["nodes"][figma_node_id]["document"]["children"][1]["children"]
        .as_array()
        .unwrap();
    let color_length = color_list.len();

    for i in 0..color_length {
        let color_name = color_list[i]["name"].as_str().unwrap();
        // let new_name = convert_to_camel_case(color_name);
        let color_detail = color_list[i]["children"][0]["fills"][0]["color"]
            .as_object()
            .unwrap()
            .clone();

        let de = serde_json::Value::Object(color_detail);
        // println!("{:?}", de);
        // let deserialized: Color = serde_json::from_value(de).unwrap();
        let j = serde_json::to_string(&de)?;
        //println!("{:?}", deserialized);
        let mut token = String::from("");
        let token_string = format!("let {} = {};\n", color_name, j.as_str());

        //let token_string = format!("let {} = {:?};\n", color_name, color_detail);
        token.push_str(&token_string);
        file_buffer.write(&token_string.as_bytes())?;
    }

    Ok(())
}

fn convert_to_camel_case(old_string: &str) -> String {
    let split = old_string.split("-");
    let vec: Vec<&str> = split.collect();
    let new_string = format!("{}{}", vec[0], convert_to_uppercase(vec[1]).as_str());
    new_string
}

fn convert_to_uppercase(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}
