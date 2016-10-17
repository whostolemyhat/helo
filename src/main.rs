extern crate rand;
extern crate iron;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rand::Rng;
use rustc_serialize::json;

#[derive(RustcEncodable)]
struct JsonResponse {
  response: String
}

fn pick_response() -> String {
  let num = rand::thread_rng().gen_range(1, 4);

  let response = match num {
    1 => "Hello World!",
    2 => "Did you see that ludicrous display last night?",
    3 => "Nice weather for ducks",
    _ => ""
  };

  response.to_string()
}

fn main() {
  Iron::new(|_: &mut Request| {
    let content_type = "application/json".parse::<Mime>().unwrap();
    let response = JsonResponse { response: pick_response() };
    let out = json::encode(&response).unwrap();

    Ok(Response::with((content_type, status::Ok, out)))
  }).http("localhost:3009").unwrap();
}
