extern crate iron;
#[macro_use]
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let router = router!(
        get "*" => not_found_handler,
        get "/" => handler,
        get "/:query" => handler
    );

    Iron::new(router).http("localhost:3000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }

    fn not_found_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::NotFound, "Invalid Api Route")))
    }
}
