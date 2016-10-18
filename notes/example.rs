extern crate rand;
extern crate iron;
extern crate rustc_serialize;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rand::Rng;
use rustc_serialize::json;
use router::Router;
use std::io::Read;

#[derive(RustcDecodable)]
struct JsonRequest {
  name: String
}

#[derive(RustcEncodable, RustcDecodable)]
struct JsonResponse {
  response: String,
  success: bool,
  error_message: String
}

impl JsonResponse {
  fn success(response: String) -> Self {
    JsonResponse { response: response, success: true, error_message: "".to_string() }
  }

  fn error(msg: String) -> Self {
    JsonResponse { response: "".to_string(), success: false, error_message: msg }
  }
}

fn pick_response(name: String) -> String {
  let num = rand::thread_rng().gen_range(1, 4);

  let response = match num {
    1 => format!("Hello {}!", name),
    2 => format!("Did you see that ludicrous display last night, {}?", name),
    3 => format!("Nice weather for ducks, isn't it {}", name),
    _ => format!("")     // match is exhaustive
  };

  response.to_string()
}

fn handler(req: &mut Request) -> IronResult<Response> {
  let response = JsonResponse::success(response: pick_response("Brian".to_string()));
  let out = json::encode(&response).unwrap();

  let content_type = "application/json".parse::<Mime>().unwrap();
  Ok(Response::with((content_type, status::Ok, out)))
}

fn post_handler(req: &mut Request) -> IronResult<Response> {
  let mut payload = String::new();

  // read the POST body
  req.body.read_to_string(&mut payload).unwrap();
  println!("{:?}", payload);

  let incoming: JsonRequest = json::decode(&payload).unwrap();

  // create a response with our random string, and pass in the string from the POST body
  let response = JsonResponse::success(response: pick_response(incoming.name));
  let out = json::encode(&response).unwrap();

  let content_type = "application/json".parse::<Mime>().unwrap();
  Ok(Response::with((content_type, status::Ok, out)))
}

fn main() {
  let mut router = Router::new();
  router.get("/", handler, "index");
  router.post("/", post_handler, "post_name");

  Iron::new(router).http("localhost:3009").unwrap();
}