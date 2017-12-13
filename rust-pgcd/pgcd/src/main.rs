extern crate iron;
#[macro_use]
extern crate mime;
extern crate router;
extern crate params;


use std::io::stderr;
use std::process::exit;
use std::io::Write;

use iron::prelude::*;
use router::Router;
use params::{Params, Value};

const URL: &str = "localhost";
const PORT: &str = "5000";

macro_rules! log {
    ($e: expr) => {
        println!("INFO!! {}", $e)
    }
}

macro_rules! response_400 {
   ($e: expr) => {
       Ok(Response::with((iron::status::BadRequest, $e)))
   }
}

macro_rules! response_200 {
    ($e: expr) => {{
       let mut response = Response::new();
       response.set_mut(iron::status::Ok);
       response.set_mut(mime!(Text/Html;Charset=Utf8));
       response.set_mut($e);
       Ok(response)
   }}
}

fn main() {
    let address = format!("{}:{}", URL, PORT);
    let mut router = Router::new();
    router.get("/gcd", get_form, "gcd");
    router.post("/gcd", post_gcd, "gcd");

    match Iron::new(router).http(address) {
       Ok(_) => {
           println!("Serving on http://{}:{}...", URL, PORT);
       }
       Err(error) => {
           writeln!(stderr(), "Got an error: {}", error);
           exit(1)
       }
    }
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    log!("Received POST on /gcd");
    let map = request.get_ref::<Params>().unwrap();
    log!(format!("Params are {:?}", map));
    let a = get_post_param(&map, "a").parse::<usize>().unwrap();
    let b = get_post_param(&map, "b").parse::<usize>().unwrap();

    match (a, b) {
        (0, _) => { return response_400!("Bad Params") },
        (_, 0) => { return response_400!("Bad Params") },
        _ => { return response_200!(gcd(a, b).to_string()) }
    }

}

fn get_post_param(map:&params::Map, key:&str) -> String {
    match map.get(key) {
        Some(&Value::String(ref val)) => { val.clone() }
        _ => { String::new() }
    }
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    log!("Received GET on /gcd");
    let form = r#"
       <title>GCD Calculator</title>
       <form action="/gcd" method="post">
           <input type="text" name="a"/>
           <input type="text" name="b"/>
           <button type="submit">Compute GCD</button>
       </form>
   "#;
    response_200!(welcome_msg)
}

fn gcd(x: usize, y: usize) -> usize {
    let big = std::cmp::max(x, y);
    let small = std::cmp::min(x, y);
    let rem = big % small;
    if rem == 0 {
        small
    } else {
        gcd(small, rem)
    }
}


#[cfg(test)]
mod tests {
    use gcd;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(10, 10), 10);
        assert_eq!(gcd(30, 18), 6);
        assert_eq!(gcd(1071, 1029), 21);
    }
}
