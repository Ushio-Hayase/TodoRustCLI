#![windows_subsystem = "windows"]

use std::{error::Error, fmt::{self, write, Display}, fs::File, io::{Read, Write}};
use serde_derive::{Serialize, Deserialize};
use serde_json::{to_string, Value};
use chrono::Utc;

#[derive(Serialize, Deserialize, Clone, Copy)]
enum Status {
    Todo,
    InProgress,
    Done
}

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Done => write!(f, "Done"),
            Self::InProgress => write!(f, "InProgress"),
            Self::Todo => write!(f, "Todo"),
        }
    }
    
}


#[derive(Serialize, Deserialize)]
struct Task {
    id : u32,
    desciption : String,
    status: Status,
    create_at: String,
    updated_at: String
}

fn add(data: &mut Vec<Task>, content: String) -> Result<(), Box<dyn Error>>{
    let add_content: Task = serde_json::from_str(&content)?;

    data.push(add_content);

    Ok(())
}

fn update(data: &mut Vec<Task>, id: u32, content: String) -> Result<(), Box<dyn Error>> {
    for x in data {
        if x.id == id {
            x.desciption = content.clone();
            let time = &Utc::now();
            x.updated_at = time.to_string();
        }
    }

    Ok(())
}

fn delete(data: &mut Vec<Task>, id: u32) -> Result<(), Box<dyn Error>> {
    for i in 0..data.len() {
        if data[i].id == id {
            data.remove(i);
        }
    }
    Ok(())
}

fn list(data: &Vec<Task>, task: Option<Status>) -> Result<(), Box<dyn Error>> {
    for x in data {
        println!("--------");
        match task {
            Some(ref op) => match op {
                Status::Done => {
                    if let Status::Done = x.status {
                        println!("{}\n{}\n{}\n{}", x.id, x.desciption, x.create_at, x.updated_at);
                    }
                },
                Status::Todo => {
                    if let Status::Todo = x.status {
                        println!("{}\n{}\n{}\n{}", x.id, x.desciption, x.create_at, x.updated_at);
                    }
                }, 
                Status::InProgress => {
                    if let Status::InProgress = x.status {
                        println!("{}\n{}\n{}\n{}", x.id, x.desciption, x.create_at, x.updated_at);
                    }
                }, 
            },
            _ => {
                println!("{}\n{}\n{}\n{}\n{}", x.id, x.desciption, x.status, x.create_at, x.updated_at);
            }
        }
    }
    Ok(())
}

fn mark(data: &mut Vec<Task>, id: u32, task: Status) -> Result<(), Box<dyn Error>>{
    for x in data {
        if x.id == id {
            x.status = task;
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
    let mut command: String = String::default();
    std::io::stdin().read_line(&mut command).unwrap();

    while command != "exit" {
        match command {
            
        }
    }
}


