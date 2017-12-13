# Rust discovery (lunch & learn for Adomik)

### Introduction :
  https://www.cambridgeconsultants.com/sites/default/files/documents/resources/introduction_to_rust-v2.pdf

### Syntax Memo :
https://gist.github.com/brson/9dec4195a88066fa42e6

### Rust Book :
https://doc.rust-lang.org/book/second-edition

# Exercice : Compute gcd
https://en.wikipedia.org/wiki/Greatest_common_divisor

## Install Rust
    * https://www.rust-lang.org/fr-FR/install.html
    * `curl https://sh.rustup.rs -sSf | sh`

## Create Rust Project
  * `cargo new pgcd --bin`
  * Use following dependencies
```toml
[dependencies]
iron = "0.6.0"
mime = "0.2.6"
router = "0.6.0"
urlencoded = "0.6.0"
params = "0.8.0"
```
  * Use `cargo run` to compile and run your program

## Create pgcd function
  * Use `cargo test` to run tests
  * Take this as reference https://doc.rust-lang.org/book/second-edition/ch11-03-test-organization.html
  * Write tests
    * assert_eq!(pgcd(10, 10), 10);
    * assert_eq!(pgcd(30, 18), 6);
    * assert_eq!(pgcd(1071, 1029), 21);
  * Make tests pass

## Write web interface
  * Hello World : https://github.com/iron/iron/blob/master/examples/hello.rs
  * Make GET /greet (curl localhost:5000/greet) return "Hello Rust"
  * Make POST /params print form params
```bash
curl --request POST \
  --url http://localhost:5000/params \
  --header 'content-type: multipart/form-data;' \
  --form param=hello
```
```rust
fn handler(request: &mut Request) -> IronResult<Response> {
   let map = request.get_ref::<Params>().unwrap();
   let a = get_param(&map, "a");
   Ok(Response::with((iron::status::Ok, a)))
}

fn get_param(map:&params::Map, key:&str) -> String {
    match map.get(key) {
        Some(&Value::String(ref val)) => { val.clone() }
        _ => { String::new() }
    }
}
```
  * POST gcd (with a, b) return the gcd of `a` and `b`
  * GET gcd display a form to use POST gcd
```rust
      let form = r#"
       <title>GCD Calculator</title>
       <form action="/gcd" method="post">
           <input type="text" name="a"/>
           <input type="text" name="b"/>
           <button type="submit">Compute GCD</button>
       </form>
   "#;
```

## Bonus 1
Add some basic logging
## Bonus 2
Define some macros to enhance lisibility
## Bonus 3
Extract gcd functions to proper module
## Bonus 4
Extract web functions to proper module
