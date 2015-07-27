extern crate iron;
extern crate bodyparser;
extern crate persistent;
extern crate rustc_serialize;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate time;
extern crate router;

use persistent::Read;
use iron::status;
use iron::prelude::*;
mod middleware;
mod myrouter;

#[derive(Debug, Clone, RustcDecodable)]
struct MyStructure {
    a: String,
    b: Option<String>,
}

fn main() {

    match logging::init() {
        Err(e) => println!("Unable to initialize logging system: {}", e),
        _ => {}
    }
    // std::env::set_var("RUST_LOG", "info");
    // env_logger::init().unwrap();
    
    //let mut chain = Chain::new(log_body);
    //chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    // Iron::new(chain).http("localhost:3000").unwrap();

    //let mut chain = Chain::new();

    info!("starting iron...");
    let chain = myrouter::setup();
    Iron::new(chain).http("localhost:3000").unwrap();
    
    
}

