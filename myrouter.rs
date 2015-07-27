extern crate iron;
extern crate router;

extern crate bodyparser;
extern crate persistent;
extern crate rustc_serialize;

use persistent::Read;
use iron::prelude::*;
use iron::status;
use router::Router;
use middleware;

//const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

pub fn setup() -> Chain {
    let mut router = Router::new();  
    router.get("/", handler); 
    router.get("/:query", handler);
    router.post("/", handler); 

    let mut chain = Chain::new(router);
    //chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    chain.link_before(middleware::ResponseTime);
    chain.link_after(middleware::ResponseTime);

    fn handler(req: &mut Request) -> IronResult<Response> {
        
        info!("handling request.");
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        println!("query {}", query);
        //Ok(Response::with(status::Ok, *query))
        Ok(Response::with(status::Ok))
    }

    chain
}
