extern crate shellexpand;

use serde::Deserialize;

use uuid::{uuid, Uuid};

use std::time::{Duration, SystemTime};
use std::env;
use std::fs::OpenOptions;
//use std::io::Result;
use std::io::Write;

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

    let tasks: Vec<Task> = serde_json::from_reader(file).expect("JSON not formatted properly");

}
