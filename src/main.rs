
/*

use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World!")))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}


mod cli;
extern crate serde_json;
extern crate serde;



use std::io::{self, Write};
use regex::Regex;
use crate::cli::{dmesg_last, cd_info};


use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Json_CD {
    cd_info: String,
    date: String,
}
async fn get_cd_info(_: Request<Body>) -> Result<Response<Body>> {
    let address = Json_CD {
        cd_info:cd_info(),
        date:Utc::today().to_string(),
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&address)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);


    Ok(Response::new(Body::from(j)))
}
#[tokio::main]
pub async fn main() -> Result<()> {
    pretty_env_logger::init();


    let make_service =
        make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(get_cd_info)) });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    server.await.unwrap();

    Ok(())
}


 */


#![feature(proc_macro_hygiene, decl_macro)]

mod cli;

#[macro_use] extern crate rocket;

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
use crate::cli::{dmesg_last, cd_info};

#[derive(Serialize, Deserialize)]
struct Json_Info {
    info: String,
    date: String,
}
#[get("/dmesg")]
fn dmesg_json() -> Json<Result<String>> {
    let address = Json_Info {
        info:dmesg_last().to_string(),
        date:Utc::today().to_string(),
    };
    return content::Json(serde_json::to_string(&address))
}

#[get("/cd-info")]
fn cd_info_json() -> Json<Result<String>> {
    let address = Json_Info {
        info:cd_info().to_string(),
        date:Utc::today().to_string(),
    };
    return content::Json(serde_json::to_string(&address))
}

fn main() {
    rocket::ignite().mount("/cd-rom_server", routes![dmesg_json,cd_info_json]).launch();

}