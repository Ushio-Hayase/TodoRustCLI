use std::{error::Error, fmt::{self, Display}, fs::File, io::{Read, Write}, path::Path, str::FromStr};
use serde_derive::{Serialize, Deserialize};
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
    if !Path::exists(Path::new(path)) {File::create(path);}
    let mut file = File::open(path)?;

    let data_str = serde_json::to_string(data)?;

    file.write(data_str.as_bytes())?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut command: String = String::default();
    
    let mut data: Vec<Task> = vec![];
    let path = Path::new("./data.json");

    if !std::path::Path::exists(path) {
        let _ =File::create(path);
    }
    let mut file = File::open(path)?;

    let _ = save(&data, "./data.json");

    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    if buf.len() == 0 {
        buf = String::from_str("[]")?;
    }

    data = serde_json::from_str(&buf)?;

    drop(file);

    loop {
        std::io::stdin().read_line(&mut command).unwrap();    
        let command_list = command.split(" ").collect::<Vec<&str>>();
        
        if let "task-cli" = command_list[0] {
            match command_list[1] {
                "add" => {
                    add(&mut data, String::from(command_list[2]))?;
                },
                "update" => {
                    update(&mut data, command_list[2].to_string().parse::<u32>().expect("id argument is not integer"), String::from(command_list[3]))?;
                },
                "delete" => {
                    delete(&mut data, command_list[2].to_string().parse::<u32>().expect("id argument is not integer"))?;
                },
                "mark-in-progress" => {
                    mark(&mut data, command_list[2].to_string().parse::<u32>().expect("id argument is not integer"), Status::InProgress)?;
                },
                "mark-done" => {
                    mark(&mut data, command_list[2].to_string().parse::<u32>().expect("id argument is not integer"), Status::Done)?;
                },
                "mark-todo" => {
                    mark(&mut data, command_list[2].to_string().parse::<u32>().expect("id argument is not integer"), Status::Todo)?;
                },
                "list" => {
                    if command_list.len() < 3 {
                        list(&data, None)?;
                    }
                    else {
                        match command_list[2] {
                            "done" => {
                                list(&data, Some(Status::Done))?;
                            },
                            "todo" => {
                                list(&data, Some(Status::Todo))?;
                            },
                            "in-progress" => {
                                list(&data, Some(Status::InProgress))?;
                            }
                            _ => {
                                println!("third command argument error");
                            }
                        }
                    }
                },
                "exit" => {
                    break;
                }
                _ => {
                    println!("Unknown Command Arguments")
                }
            }
            save(&data, "./data.json")?;
        }

    }

    Ok(())
}


