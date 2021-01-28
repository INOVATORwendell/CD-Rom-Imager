extern crate chrono;


use std::process::Command;


use chrono::Local;
use std::io::{Read, BufReader};



pub fn dmesg_last() -> String {
    let output = Command::new("dmesg")
        .output()
        .expect("failed to execute process");

    let dmesg=String::from_utf8_lossy(&output.stdout);
    let result : Vec<&str> = dmesg.split_terminator("\n").collect();
    return  result[result.len()-1].to_string();
}


pub fn cd_info() -> String {

    let output = Command::new("cd-drive")
        .output()
        .expect("failed to execute process");


    return  String::from_utf8_lossy(&output.stdout).to_string();
}

pub fn write_from_drive(file_:&str) -> String {
    let mut command ="cdrecord -v speed=8 dev=0,0,0  ".to_string();
    command.push(file_.parse().unwrap());
    let output = Command::new(command)
        .output()
        .expect("failed to execute process");
    return  String::from_utf8_lossy(&output.stdout).to_string();


}

pub fn make_iso() -> String {
    let date = Local::now();
    let mut command ="sudo cat /dev/sr0 > ".to_string();
    command.push(date.format("%Y%m%d%H%M%S").to_string().as_str().parse().unwrap());

    let output = Command::new(command)
        .output()
        .expect("failed to execute process");


    //let result : Vec<&str> = dmesg.split_terminator(("\n")).collect();


    return  String::from_utf8_lossy(&output.stdout).to_string();
}