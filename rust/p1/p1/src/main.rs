extern crate shellexpand;

use serde::Deserialize;

use std::time::{Duration, SystemTime};
use std::env;
use std::fs::OpenOptions;
//use std::io::Result;
use std::io::Write;
use std::fs::File;
use std::error::Error;

use rand::Rng;

fn error(msg: &str) {
    usage();
    println!("\tError: {}", msg);
    std::process::exit(1);
}


fn usage() {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Task {
	id: String,
	description: String,
	status: String,
	timestamp: SystemTime
}

fn main() {
    //println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
//    println!("{}", args);

//    dbg!(&args);

    if args.len() == 1 {
       error("must specify a command");
    }

    let mut file = OpenOptions::new()
	.read(true)
	.write(true)
	.create(true)
	.open(shellexpand::tilde("~/.tasks.json").into_owned())
	.unwrap();
//    file.write_all(args[1].as_bytes());

//    Ok(());

    let tasks: Vec<Task>; 

    let readResults = readTasks(file);
    match readResults {
        Ok(file) => tasks = file,
        Err(_err) => tasks = Vec::new()
    }
    
    
    // let action = args[1];
    match args[1].as_str() {
        "create" => createTask(args, &tasks),
        "status" => status(args, &tasks) ,
        "update" => updateTask(args, &tasks),
        "delete" => deleteTask(args, &tasks),
        _ => error("invalid argument")

    }

    writeToFile(&tasks);


}

fn readTasks(file: File) -> Result<Vec<Task>, Box<dyn Error>> {
    let u = serde_json::from_reader(file)?;
    return Ok(u);
}


fn status(args: Vec<String>, tasks: &Vec<Task>) {}

fn createTask(args: Vec<String>, tasks: &Vec<Task>) {
    if args.len() != 3 {
        error("needs 3 arguments (third argument must be the description)");
    }

    let mut id: String;

    while true {
        id = String::from("");

        for i in 0..6 {
            let n: u8 = rand::thread_rng().gen_range(65..90);
            let c = n as char;
            id.push(c);

        }

        println!("{}", id);
        break;


    }

}

fn updateTask(args: Vec<String>, tasks: &Vec<Task>) {

}

fn deleteTask(args: Vec<String>, tasks: &Vec<Task>) {

}


fn writeToFile(tasks: &Vec<Task>) {

}

