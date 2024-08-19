use std::{error::Error, fs::File, io::{Read, Write}, time::Instant};
use serde_derive::{Serialize, Deserialize};
use serde_json::{to_string, Value};
use chrono::Utc;

#[derive(Serialize, Deserialize)]
enum Status {
    Todo,
    InProgress,
    Done
}

#[derive(Serialize, Deserialize)]
struct Task<'a> {
    id : u32,
    desciption : &'a str,
    status: Status,
    create_at: &'a str,
    updated_at: &'a str
}

fn add<'a>(data: &mut Vec<Task<'a>>, content: &'a str) -> Result<(), Box<dyn Error>>{
    let add_content: Task = serde_json::from_str(content)?;

    data.push(add_content);

    Ok(())
}

fn update<'a>(data: &mut Vec<Task<'a>>, id: u32, content: &'a str) -> Result<(), Box<dyn Error>> {
    for x in data {
        if x.id == id {
            x.desciption = content;
            let time = Utc::now();
            x.updated_at = time.to_string().as_ref();
        }
    }

    Ok(())
}

fn save(data: &Vec<Task>, path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(path)?;

    let data_str = serde_json::to_string(data)?;

    file.write(data_str.as_bytes())?;

    Ok(())
}

fn main() {
    println!("Hello, world!");
}


