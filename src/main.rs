


#![feature(proc_macro_hygiene, decl_macro)]

mod cli;

#[macro_use] extern crate rocket;
extern crate chrono;

use std::io::{self, Write};
use regex::Regex;

use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use rocket::response::content;
use rocket::response::content::Json;
use crate::cli::{dmesg_last, cd_info, write_from_drive, make_iso};
use rocket::http::RawStr;
use std::fs::File;
use std::borrow::Borrow;

#[derive(Serialize, Deserialize)]
struct Json_Info {
    info: String,
    date: String,
}

#[derive(Serialize, Deserialize)]
struct Write_Return {
    filename: String,
    date: String,
    size: String,
    file_type:String,
    speed:String,

}

#[derive(Serialize, Deserialize)]
struct Read_CD {
    filename: String,
    date: String,
    size: String,
    is_shared:String,
    server_alive:String,
    log:String,

}
#[get("/dmesg")]
fn dmesg_json() -> Json<Result<String>> {
    let address = Json_Info {
        info:dmesg_last().to_string(),
        date:Utc::today().to_string(),
    };
    return content::Json(serde_json::to_string(&address))
}
#[get("/write/<iso_name>")]
fn write_file(iso_name:&RawStr) -> Json<Result<String>> {
        println!("{}",iso_name.as_str());
        let log=write_from_drive(iso_name.as_str());
        let file=iso_name.as_str();
        let date=Utc::today().to_string();
        let mut iso_file = File::create(file).unwrap();
        let size=iso_file.metadata().unwrap().len();
        let file_type=    iso_file.metadata().unwrap().file_type().is_file();

        let speed=8;

       let write_json=Write_Return{
            filename: file.to_string(),
            date:date,
            size: size.to_string(),
            file_type: "file : iso".to_string(),
            speed: speed.to_string()
        };


        println!("{}",serde_json::to_string(&write_json).unwrap());

    return content::Json(serde_json::to_string(&write_json))
}
#[get("/cd-info")]
fn cd_info_json() -> Json<Result<String>> {
    let address = Json_Info {
        info:cd_info().to_string(),
        date:Utc::today().to_string(),
    };
    return content::Json(serde_json::to_string(&address))
}
#[get("/read_iso")]
fn made_iso() -> Json<Result<String>> {



    let mut rng = rand::thread_rng();
    rng.range(9999,999999);
    println!("{}",date.to_string());
    let log=make_iso(&*date.to_string());
    let mut iso_file = File::create(date.to_string().as_str()).unwrap();

    let size=iso_file.metadata().unwrap().len();
    let address = Read_CD {
        filename: date.to_string(),
        date:Utc::today().to_string(),
        size: size.to_string(),
        is_shared: "true".to_string(),
        server_alive: "true".to_string(),
        log: log.to_string()
    };
    return content::Json(serde_json::to_string(&address))
}

fn main() {
    rocket::ignite().mount("/cd-rom_server", routes![dmesg_json,cd_info_json,write_file,made_iso]).launch();

}