create a new Rust project with
`cargo new hello --bin`

`--bin` flag tells Cargo to create a project which will compile to a binary file, rather than a library.

Add [Iron](https://crates.io/crates/iron) to your dependencies in 'Cargo.toml':

```
[dependencies]
iron = "0.4.0"
```

Update `/src/main.rs` with the code from Iron's ['Hello world' example](http://ironframework.io/doc/iron/#hello-world), and run via `cargo run`. Cargo will install Iron and it's dependencies, so this may take some time, then compile and run the 'hello world' code.

```
extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
  Iron::new(|_: &mut Request| {
    Ok(Response::with((status::Ok, "Hello world!")))
  }).http("localhost:3009").unwrap();
}
```

![Terminal output](first-run.png)
![Hello!](first-output.png)

## json

Next, we'll change our response from text to JSON. Based on the [content type example](https://github.com/iron/iron/blob/master/examples/content_type.rs), add the `Mime` import, and update the response:

```
extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;

fn main() {
  Iron::new(|_: &mut Request| {
    let content_type = "application/json".parse::<Mime>().unwrap();

    Ok(Response::with((content_type, status::Ok, "{ response: \"Hello world!\" }")))
  }).http("localhost:3009").unwrap();
}
```

Let's change the response so it selects a random string. Add the 'rand' crate to the dependencies:

``` Cargo.toml
[dependencies]
iron = "0.4.0"
rand = "0.3"
```

then create a function in `main.rs` which picks a string at random:

```
extern crate rand;      // reference the random crate
extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rand::Rng;        // use random number generator

// pick a string at random
fn pick_response() -> String {

  // generate a number between 1 and 3 (4 is exclusive upper-bound)
  let num = rand::thread_rng().gen_range(1, 4);

  // match the random number and pick a random string
  let response = match num {
    1 => "Hello World!",
    2 => "Did you see that ludicrous display last night?",
    3 => "Nice weather for ducks",
    _ => ""     // match is exhaustive
  };

  // return our string
  response.to_string()
}

fn main() {
  // check it's working - this will appear in your terminal
  println!("{:?}", pick_response());

  Iron::new(|_: &mut Request| {
    let content_type = "application/json".parse::<Mime>().unwrap();

    Ok(Response::with((content_type, status::Ok, "{ response: \"Hello world!\" }")))
  }).http("localhost:3009").unwrap();
}

```

Run via `cargo run` - if all goes well, you'll see a string printed in the terminal:

![What was Wenger thinking, sending Walcott on that early?](random-string.png)

Now we need to transform the random string into JSON so we can send it. There's a choice of crates to use here: `rustc-serialize` and `serde-json`. `rustc-serialize` is an older option which isn't very active, while `serde` is actively developed and is the de facto standard for Rust serialisation. However, the downside of `serde` is that you'll need touse the nightly version of Rust, at least until custom traits have been added to the stable branch. [Here's a bit of a comparison between the two.](https://www.reddit.com/r/rust/comments/3v4ktz/differences_between_serde_and_rustc_serialize/).

I'm going to use `rustc-serialize`, since I've found using `serde` a huge pain on Windows.

### add serde
We need the `rustc-serialize` crate, so add it to your dependencies:

``` Cargo.toml
[dependencies]
iron = "0.4.0"
rand = "0.3"
rustc-serialize = "0.3"
```

aaand update the imports in `main.rs`:

```main.rs
extern crate rand;
extern crate iron;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rand::Rng;
use rustc_serialize::json;
```

We'll create a struct which holds the response string, and use `serde` to convert it into JSON, which we'll then send as a response.

Create a struct which has one string field, and initialise it in `main()`:

```main.rs
struct JsonResponse {
  response: String
}

fn main() {
  println!("{:?}", pick_response());

  Iron::new(|_: &mut Request| {
    let content_type = "application/json".parse::<Mime>().unwrap();

    // create the response
    let response = JsonResponse { response: pick_response() };

    // convert the response struct to JSON
    let out = json::encode(&response).unwrap();

    Ok(Response::with((content_type, status::Ok, out)))
  }).http("localhost:3009").unwrap();
}

```

If you try to run this, you'll get an error complaining that our struct doesn't satisfy the `serde::Serialize` trait

![](trait.png)

so add the appropriate derive to our struct:

```main.rs
#[derive(RustcEncodeable)]
struct JsonResponse {
  response: String
}
```
and run:

![](random-json.png)

## get and post
## gnu header
## logging
## deploy/compile flag
## nginx
## https