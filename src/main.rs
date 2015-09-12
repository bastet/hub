extern crate iron;
#[macro_use]
extern crate router;

use iron::prelude::*;
use iron::{ Url, status, Headers };
use iron::request::{ Body };
use router::Router;

fn main() {
    let router = router!(
        get "*" => not_found_handler,
        get "/" => handler,
        get "/:query" => handler
    );

    Iron::new(router).http("localhost:3000").unwrap();


}

fn handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn not_found_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::NotFound, "Invalid Api Route")))
}

use std::net::{ Ipv4Addr, SocketAddrV4, SocketAddr };

#[test]
// These tests don't work yet as we cannot mock a network stream
fn test_route_not_found() {
    let mut req = Request {
        url: Url::parse("your/mum").unwrap(),
        remote_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0u8,0u8,0u8,0u8), 0u16)),
        local_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0u8,0u8,0u8,0u8), 0u16)),
        headers: Headers::new(),
        body: Body
    };
    not_found_handler(&mut req);
}
