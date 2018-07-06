extern crate iron;
#[macro_use]
extern crate mime;
extern crate router;
extern crate urlencoded;

// iron = "0.5.1"
// mime = "0.2.3"
// router = "0.5.1"
// urlencoded = "0.5.0"

use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;

const URL: &str = "localhost";
const PORT: &str = "5000";

macro_rules! error_response {
   ($e: expr) => {{
       let mut response = Response::new();
       response.set_mut(status::BadRequest);
       response.set_mut($e);
       Ok(response)
   }}
}

macro_rules! ok_response {
   ($e: expr) => {{
       let mut response = Response::new();
       response.set_mut(status::Ok);
       response.set_mut(mime!(Text/Html;Charset=Utf8));
       response.set_mut($e);
       Ok(response)
   }}
}

fn main() {

   let mut router = Router::new();

   router.get("/", get_form, "root");
   router.get("/hello", get_hello_world, "hello");
   router.post("/gcd", post_gcd, "gcd");
    dnkndsknckdsncksndcksdkc
        
   match Iron::new(router).http(format!("{}:{}", URL, PORT)) {
       Ok(_) => {
           println!("Serving on http://{}:{}...", URL, PORT);
       }
       Err(error) => {
           writeln!(stderr(), "Got an error: {}", error);
           exit(1)
       }
   }
}

fn gcd(a: usize, b: usize) -> usize {
   if b == 0 {
       return a;
   }
   gcd(b, a % b)
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
   let welcome_msg = r#"
       <title>GCD Calculator</title>
       <form action="/gcd" method="post">
           <input type="text" name="n"/>
           <input type="text" name="n"/>
           <button type="submit">Compute GCD</button>
       </form>
   "#;
   ok_response!(welcome_msg)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {

   let form_data = match request.get_ref::<UrlEncodedBody>() {
       Err(e) => {
           return error_response!(format!("Error parsing from data: {:?}\n", e));
       }
       Ok(map) => map,
   };

   let unparsed_numbers = match form_data.get("n") {
       None => {
           return error_response!("Form data has no 'n' parameter\n");
       }
       Some(nums) => nums,
   };

   let mut numbers = Vec::new();
   for unparsed in unparsed_numbers {
       match unparsed.parse::<usize>() {
           Err(_) => {
               return error_response!("Value for 'n' parameter not a number\n");
           }
           Ok(n) => {
               numbers.push(n);
           }
       }
   }

   let mut d = numbers[0];
   for m in &numbers[1..] {
       d = gcd(d, *m);
   }

   ok_response!(format!(
       "The greatest common divisor of the numbers {:?} is <b>{}</b>\n",
       numbers,
       d
   ))
}

fn get_hello_world(_request: &mut Request) -> IronResult<Response> {
   ok_response!("<h1>Hello World</h1>")
}
 
